use crate::{Instruction, Opcode, Reg};
use byteorder::{BigEndian, ByteOrder};

pub trait Env {
  type Error;

  fn reset(&mut self);
  fn mem_set(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error>;
  fn mem_fetch(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error>;
  fn ecall(&mut self, ecall: u8, param: i32) -> Result<i32, Self::Error>;
}

#[derive(Debug)]
pub enum VMError {
  EmptyProg,
  InvalidProg,
  EnvFault,
}

pub struct VM<'prog, E: Env> {
  env: E,
  pc: usize,
  reg: [i32; 32],
  prog: Option<&'prog [u8]>,
}

impl<'prog, E: Env> VM<'prog, E> {
  pub fn new(env: E) -> Self {
    Self {
      env,
      pc: 0,
      reg: [0; 32],
      prog: None,
    }
  }

  pub fn reset(&mut self) {
    self.rewind();
    self.reg = Default::default();
    self.prog = None;
    self.env.reset();
  }

  pub fn rewind(&mut self) {
    self.pc = 0;
  }

  pub fn get_env(&mut self) -> &mut E {
    &mut self.env
  }

  pub fn get_reg(&mut self) -> &mut [i32; 32] {
    &mut self.reg
  }

  pub fn get_pc(&mut self) -> &mut usize {
    &mut self.pc
  }

  pub fn load(&mut self, prog: &'prog [u8]) -> Result<(), VMError> {
    if prog.len() < 4 || prog[0] != 0xaf || prog[1] != 0xaf {
      return Err(VMError::InvalidProg);
    }
    self.reset();
    let ram_end = BigEndian::read_u16(&prog[2..4]) as usize + 4;
    if ram_end > 4 {
      self
        .env
        .mem_set(0, &prog[4..ram_end])
        .map_err(|_| VMError::EnvFault)?;
    }
    self.prog = Some(&prog[ram_end..]);
    Ok(())
  }

  pub fn step(&mut self) -> Result<bool, VMError> {
    let inst = self.get_active_instruction()?;
    let r1 = inst.r1 as usize;
    let r2 = inst.r2 as usize;
    let r3 = inst.r3 as usize;
    let imm = inst.imm;

    let res = match inst.opcode {
      Opcode::halt => {
        return Ok(true);
      }
      Opcode::ecall => {
        let res = self
          .env
          .ecall(inst.imm as u8, self.reg[r3])
          .map_err(|_| VMError::EnvFault)?;
        Some(res)
      }
      Opcode::jal => {
        self.reg[Reg::ra as usize] = (self.pc + 1) as i32;
        return self.jump(r3, imm);
      }
      Opcode::beq => {
        if self.reg[r1] == self.reg[r2] {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::bne => {
        if self.reg[r1] != self.reg[r2] {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::bge => {
        if self.reg[r1] >= self.reg[r2] {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::bgeu => {
        if (self.reg[r1] as u32) >= (self.reg[r2] as u32) {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::blt => {
        if self.reg[r1] < self.reg[r2] {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::bltu => {
        if (self.reg[r1] as u32) < (self.reg[r2] as u32) {
          return self.jump(r3, imm);
        }
        None
      }
      Opcode::addi => Some(self.reg[r2] + imm as i32),
      Opcode::ori => Some(self.reg[r2] | imm as i32),
      Opcode::xori => Some(self.reg[r2] ^ imm as i32),
      Opcode::andi => Some(self.reg[r2] & imm as i32),
      Opcode::slli => Some(self.reg[r2] << imm),
      Opcode::srli => Some(self.reg[r2] >> imm),
      Opcode::add => Some(self.reg[r2] + self.reg[r3]),
      Opcode::and => Some(self.reg[r2] & self.reg[r3]),
      Opcode::mul => Some(self.reg[r2] * self.reg[r3]),
      Opcode::muli => Some(self.reg[r2] * imm as i32),
      Opcode::or => Some(self.reg[r2] | self.reg[r3]),
      Opcode::sub => Some(self.reg[r2] - self.reg[r3]),
      Opcode::xor => Some(self.reg[r2] ^ self.reg[r3]),
      Opcode::slt => Some((self.reg[r2] < self.reg[r3]) as i32),
      Opcode::sltu => Some(((self.reg[r2] as u32) < (self.reg[r3] as u32)) as i32),
      Opcode::sltiu => Some(((self.reg[r2] as u32) < (imm as u32)) as i32),
      Opcode::srl => Some(((self.reg[r2] as u32) >> (self.reg[r3] as u32)) as i32),
      Opcode::sll => Some(self.reg[r2] << self.reg[r3]),
      Opcode::sra => Some(self.reg[r2] >> self.reg[r3]),
      Opcode::sb => {
        let offset = self.reg[r3] + imm as i32;
        let val = self.reg[r1] as i8 as u8;
        self
          .env
          .mem_set(offset as u16, &[val])
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::sh => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0, 0];
        let val = self.reg[r1] as i16;
        BigEndian::write_i16(&mut buf, val);
        self
          .env
          .mem_set(offset as u16, &buf)
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::sw => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0, 0, 0, 0];
        BigEndian::write_i32(&mut buf, self.reg[r1]);
        self
          .env
          .mem_set(offset as u16, &buf)
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::lui => Some((self.reg[r1] & 0xffff) | (imm as i32) << 16),
      Opcode::la => Some(self.reg[r3] + imm as i32),
      Opcode::lb => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(buf[0] as i8 as i32)
      }
      Opcode::lbu => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(buf[0] as i32)
      }
      Opcode::lh => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_i16(&buf) as i32)
      }
      Opcode::lhu => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_u16(&buf) as i32)
      }
      Opcode::lw => {
        let offset = self.reg[r3] + imm as i32;
        let mut buf = [0, 0, 0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_i32(&buf) as i32)
      }
    };
    self.pc += 1;
    match res {
      Some(val) if r1 > 0 => {
        self.reg[r1] = val;
      }
      _ => {}
    }
    Ok(false)
  }

  pub fn spin(&mut self) -> Result<(), VMError> {
    loop {
      match self.step() {
        Ok(true) => {
          return Ok(());
        }
        Err(err) => {
          return Err(err);
        }
        _ => {}
      }
    }
  }

  pub fn respin(&mut self) -> Result<(), VMError> {
    self.rewind();
    self.spin()
  }

  fn jump(&mut self, r: usize, offset: i16) -> Result<bool, VMError> {
    match self.prog {
      None => Err(VMError::EmptyProg),
      Some(prog) => {
        let new_pc = self.reg[r] as i16 + offset;
        if new_pc < 0 || (new_pc * 4 + 4) as usize > prog.len() {
          return Ok(true);
        }
        self.pc = new_pc as usize;
        Ok(false)
      }
    }
  }

  fn get_active_instruction(&self) -> Result<Instruction, VMError> {
    let prog = match self.prog {
      Some(prog) => prog,
      None => {
        return Err(VMError::EmptyProg);
      }
    };
    let offset = self.pc * 4;
    if (offset + 4) > prog.len() {
      return Ok(Instruction::new(Opcode::halt, Reg::x0, Reg::x0, Reg::x0, 0));
    }
    let word = BigEndian::read_u32(&prog[offset..(offset + 4)]);
    Instruction::parse(word).map_err(|_| VMError::InvalidProg)
  }
}

impl<E: Env + core::fmt::Debug> core::fmt::Debug for VM<'_, E> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match self.get_active_instruction() {
      Ok(inst) => write!(
        f,
        "pc:{:<3} {:?}\t{:?}\t{:?}",
        self.pc,
        inst,
        &self.reg[1..17],
        self.env,
      ),
      Err(err) => write!(f, "pc:{:<3} {:?}", self.pc, err,),
    }
  }
}
