use crate::{Opcode, Reg};
use byteorder::{BigEndian, ByteOrder};

pub trait Env {
  type Error;

  fn reset(&mut self);
  fn mem_set(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error>;
  fn mem_fetch(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error>;
  fn ecall(&mut self, ecall: i32, param: i32) -> Result<i32, Self::Error>;
}

#[derive(Debug)]
pub enum VMError {
  InvalidProg,
  EnvFault,
}

pub struct VM<'prog, E: Env> {
  env: E,
  pc: usize,
  reg: [i32; 8],
  prog: Option<&'prog [u8]>,
}

impl<'prog, E: Env> VM<'prog, E> {
  pub fn new(env: E) -> Self {
    Self {
      env,
      pc: 0,
      reg: [0; 8],
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
  pub fn get_reg(&mut self) -> &mut [i32; 8] {
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
    let (opcode, rd, rs1, rs2, imm) = match self.current_op() {
      Some(op) => op,
      None => return Ok(true),
    };
    let res = match opcode {
      Opcode::ECALL => {
        let res = self
          .env
          .ecall(imm, self.reg[rs2])
          .map_err(|_| VMError::EnvFault)?;
        Some(res)
      }
      Opcode::BEQ => {
        if self.reg[rd] == self.reg[rs1] {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::BNE => {
        if self.reg[rd] != self.reg[rs1] {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::BGE => {
        if self.reg[rd] >= self.reg[rs1] {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::BGEU => {
        if (self.reg[rd] as u32) >= (self.reg[rs1] as u32) {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::BLT => {
        if self.reg[rd] < self.reg[rs1] {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::BLTU => {
        if (self.reg[rd] as u32) < (self.reg[rs1] as u32) {
          self.branch(self.reg[rs2] + imm);
          return Ok(false);
        }
        None
      }
      Opcode::SB => {
        let offset = imm + self.reg[rs2];
        let val = self.reg[rd] as i8 as u8;
        self
          .env
          .mem_set(offset as u16, &[val])
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::SH => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0, 0];
        let val = self.reg[rd] as i16;
        BigEndian::write_i16(&mut buf, val);
        self
          .env
          .mem_set(offset as u16, &buf)
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::SW => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0, 0, 0, 0];
        BigEndian::write_i32(&mut buf, self.reg[rd]);
        self
          .env
          .mem_set(offset as u16, &buf)
          .map_err(|_| VMError::EnvFault)?;
        None
      }
      Opcode::LUI => Some((self.reg[rd] & 0xffff) | (imm << 16)),
      Opcode::LA => Some(self.reg[rs2] + imm as i32),
      Opcode::ADDI => Some(self.reg[rs1] + imm as i32),
      Opcode::ORI => Some(self.reg[rs1] | imm as i32),
      Opcode::XORI => Some(self.reg[rs1] ^ imm as i32),
      Opcode::ANDI => Some(self.reg[rs1] & imm as i32),
      Opcode::SLLI => Some(self.reg[rs1] << imm as i32),
      Opcode::SRLI => Some(self.reg[rs1] >> imm as i32),
      Opcode::ADD => Some(self.reg[rs1] + self.reg[rs2]),
      Opcode::AND => Some(self.reg[rs1] & self.reg[rs2]),
      Opcode::MUL => Some(self.reg[rs1] * self.reg[rs2]),
      Opcode::MULI => Some(self.reg[rs1] * imm as i32),
      Opcode::OR => Some(self.reg[rs1] | self.reg[rs2]),
      Opcode::SUB => Some(self.reg[rs1] - self.reg[rs2]),
      Opcode::XOR => Some(self.reg[rs1] ^ self.reg[rs2]),
      Opcode::SLT => Some((self.reg[rs1] < self.reg[rs2]) as i32),
      Opcode::SLTU => Some(((self.reg[rs1] as u32) < (self.reg[rs2] as u32)) as i32),
      Opcode::SLTIU => Some(((self.reg[rs1] as u32) < (imm as u32)) as i32),
      Opcode::SLL => Some(self.reg[rs1] << self.reg[rs2]),
      Opcode::SRL => Some(((self.reg[rs1] as u32) >> (self.reg[rs2] as u32)) as i32),
      Opcode::SRA => Some(self.reg[rs1] >> self.reg[rs2]),
      Opcode::LB => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(buf[0] as i8 as i32)
      }
      Opcode::LBU => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(buf[0] as i32)
      }
      Opcode::LH => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_i16(&buf) as i32)
      }
      Opcode::LHU => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_u16(&buf) as i32)
      }
      Opcode::LW => {
        let offset = imm + self.reg[rs2];
        let mut buf = [0, 0, 0, 0];
        self
          .env
          .mem_fetch(offset as u16, &mut buf)
          .map_err(|_| VMError::EnvFault)?;
        Some(BigEndian::read_i32(&buf) as i32)
      }
    };
    match res {
      Some(val) if rd > 0 => {
        self.reg[rd] = val;
      }
      _ => {}
    }
    self.pc += 1;
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

  fn branch(&mut self, offset: i32) {
    self.reg[Reg::RA as usize] = (self.pc + 1) as i32;
    self.pc = offset as usize;
  }

  fn current_op(&self) -> Option<(Opcode, usize, usize, usize, i32)> {
    let prog = match self.prog {
      Some(prog) => prog,
      None => {
        return None;
      }
    };
    let offset = self.pc * 4;
    if (offset + 4) > prog.len() {
      return None;
    }
    let word = BigEndian::read_u32(&prog[offset..(offset + 4)]);
    let op = unsafe { core::mem::transmute(word as u8 & 0x7f) };
    let rd = (word >> 7 & 0x7) as usize;
    let rs1 = (word >> 10 & 0x7) as usize;
    let rs2 = (word >> 13 & 0x7) as usize;
    let imm = (word >> 16) as i16;
    Some((op, rd, rs1, rs2, imm as i32))
  }
}

#[cfg(feature = "std")]
impl<E: Env + core::fmt::Debug> core::fmt::Debug for VM<'_, E> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    fn fmt_reg(reg: usize) -> String {
      match reg {
        0 => "r0".to_string(),
        1 => "ra".to_string(),
        x => format!("s{}", x - 2),
      }
    };
    let op = match self.current_op() {
      Some(op) => format!(
        "{:?} {} {} {} {}",
        op.0,
        fmt_reg(op.1),
        fmt_reg(op.2),
        fmt_reg(op.3),
        op.4
      ),
      None => String::from("HALTED"),
    };
    write!(
      f,
      "pc:{:<3} {:<18} {:?}\t{:?}",
      self.pc,
      op,
      &self.reg[1..],
      self.env,
    )
  }
}
