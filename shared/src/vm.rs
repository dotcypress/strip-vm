use crate::Opcode;
use byteorder::{BigEndian, ByteOrder};

pub trait Host {
  type Error;

  fn reset(&mut self);
  fn fetch_mem(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error>;
  fn store_mem(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub enum VMError {
  ProgramHalted,
  InvalidProg,
  HostFault,
}

pub struct VM<'prog, H: Host> {
  host: H,
  pc: usize,
  reg: [i32; 8],
  prog: Option<&'prog [u8]>,
}

impl<'prog, H: Host> VM<'prog, H> {
  pub fn new(host: H) -> Self {
    Self {
      host,
      pc: 0,
      reg: [0; 8],
      prog: None,
    }
  }

  pub fn reset(&mut self) {
    self.reg = Default::default();
    self.prog = None;
    self.rewind();
    self.host.reset();
  }

  pub fn rewind(&mut self) {
    self.pc = 0;
  }

  pub fn get_host(&mut self) -> &mut H {
    &mut self.host
  }

  pub fn load(&mut self, prog: &'prog [u8]) -> Result<(), VMError> {
    if prog.len() < 4 || prog[0] != 0xaf || prog[1] != 0xaf {
      return Err(VMError::InvalidProg);
    }
    self.reset();
    let ram_end = BigEndian::read_u16(&prog[2..4]) as usize + 4;
    if ram_end > 4 {
      self
        .host
        .store_mem(0, &prog[4..ram_end])
        .map_err(|_| VMError::HostFault)?;
    }
    self.prog = Some(&prog[ram_end..]);
    Ok(())
  }

  pub fn spin(&mut self) -> Result<(), VMError> {
    self.rewind();
    loop {
      match self.step() {
        Err(VMError::ProgramHalted) => return Ok(()),
        Err(err) => return Err(err),
        _ => {}
      }
    }
  }

  pub fn step(&mut self) -> Result<(), VMError> {
    let (opcode, r1, r2, r3, imm) = match self.current_op() {
      Some(op) => op,
      None => return Err(VMError::ProgramHalted),
    };
    match opcode {
      Opcode::BEQ => {
        if self.reg[r1] == self.reg[r2] {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::BNE => {
        if self.reg[r1] != self.reg[r2] {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::BGE => {
        if self.reg[r1] >= self.reg[r2] {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::BGEU => {
        if (self.reg[r1] as u32) >= (self.reg[r2] as u32) {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::BLT => {
        if self.reg[r1] < self.reg[r2] {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::BLTU => {
        if (self.reg[r1] as u32) < (self.reg[r2] as u32) {
          self.pc = imm as usize;
          return Ok(());
        }
      }
      Opcode::SB => {
        let offset = imm + self.reg[r3];
        let val = self.reg[r1] as i8;
        self
          .host
          .store_mem(offset as u16, &[val as u8])
          .map_err(|_| VMError::HostFault)?;
      }
      Opcode::SH => {
        let offset = imm + self.reg[r3];
        let mut buf = [0, 0];
        let val = self.reg[r1] as i16;
        BigEndian::write_i16(&mut buf, val);
        self
          .host
          .store_mem(offset as u16, &buf)
          .map_err(|_| VMError::HostFault)?;
      }
      Opcode::SW => {
        let offset = imm + self.reg[r3];
        let mut buf = [0, 0, 0, 0];
        BigEndian::write_i32(&mut buf, self.reg[r1]);
        self
          .host
          .store_mem(offset as u16, &buf)
          .map_err(|_| VMError::HostFault)?;
      }
      reg_op if r1 > 0 => {
        let reg_val = match reg_op {
          Opcode::LUI => imm << 16,
          Opcode::ADDI => self.reg[r2] + imm as i32,
          Opcode::ORI => self.reg[r2] | imm as i32,
          Opcode::XORI => self.reg[r2] ^ imm as i32,
          Opcode::ANDI => self.reg[r2] & imm as i32,
          Opcode::SLLI => self.reg[r2] << imm as i32,
          Opcode::SRLI => self.reg[r2] >> imm as i32,
          Opcode::ADD => self.reg[r2] + self.reg[r3],
          Opcode::AND => self.reg[r2] & self.reg[r3],
          Opcode::MUL => self.reg[r2] * self.reg[r3],
          Opcode::OR => self.reg[r2] | self.reg[r3],
          Opcode::SUB => self.reg[r2] - self.reg[r3],
          Opcode::XOR => self.reg[r2] ^ self.reg[r3],
          Opcode::SLT => (self.reg[r2] < self.reg[r3]) as i32,
          Opcode::SLTU => ((self.reg[r2] as u32) < (self.reg[r3] as u32)) as i32,
          Opcode::SLTIU => ((self.reg[r2] as u32) < (imm as u32)) as i32,
          Opcode::SLL => self.reg[r2] << self.reg[r3],
          Opcode::SRL => ((self.reg[r2] as u32) >> (self.reg[r3] as u32)) as i32,
          Opcode::SRA => self.reg[r2] >> self.reg[r3],
          Opcode::LB => {
            let offset = imm + self.reg[r3];
            let mut buf = [0];
            self
              .host
              .fetch_mem(offset as u16, &mut buf)
              .map_err(|_| VMError::HostFault)?;
            buf[0] as i8 as i32
          }
          Opcode::LBU => {
            let offset = imm + self.reg[r3];
            let mut buf = [0];
            self
              .host
              .fetch_mem(offset as u16, &mut buf)
              .map_err(|_| VMError::HostFault)?;
            buf[0] as i32
          }
          Opcode::LH => {
            let offset = imm + self.reg[r3];
            let mut buf = [0, 0];
            self
              .host
              .fetch_mem(offset as u16, &mut buf)
              .map_err(|_| VMError::HostFault)?;
            BigEndian::read_i16(&buf) as i32
          }
          Opcode::LHU => {
            let offset = imm + self.reg[r3];
            let mut buf = [0, 0];
            self
              .host
              .fetch_mem(offset as u16, &mut buf)
              .map_err(|_| VMError::HostFault)?;
            BigEndian::read_u16(&buf) as i32
          }
          Opcode::LW => {
            let offset = imm + self.reg[r3];
            let mut buf = [0, 0, 0, 0];
            self
              .host
              .fetch_mem(offset as u16, &mut buf)
              .map_err(|_| VMError::HostFault)?;
            BigEndian::read_i32(&buf) as i32
          }
          _ => unreachable!(),
        };
        self.reg[r1] = reg_val;
      }
      _ => {}
    }
    self.pc += 1;
    Ok(())
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
    let r1 = (word >> 7 & 0x7) as usize;
    let r2 = (word >> 10 & 0x7) as usize;
    let r3 = (word >> 13 & 0x7) as usize;
    let imm = (word >> 16) as i16;
    Some((op, r1, r2, r3, imm as i32))
  }
}

#[cfg(feature = "std")]
impl<H: Host + core::fmt::Debug> core::fmt::Debug for VM<'_, H> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let op = match self.current_op() {
      Some(op) => format!("{:?} x{} x{} x{} {}", op.0, op.1, op.2, op.3, op.3),
      None => String::from("HALTED"),
    };
    write!(
      f,
      "VM {:<4} {:<16} {:?}\t{:?}",
      self.pc,
      op,
      &self.reg[1..],
      self.host,
    )
  }
}
