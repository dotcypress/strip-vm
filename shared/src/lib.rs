#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod compiler;
#[cfg(feature = "std")]
pub mod parser;
pub mod vm;

#[derive(Debug)]
pub enum Error {
  ParseError,
  CompilerError(CompilerError),
  VMError,
}

#[derive(Debug)]
pub enum CompilerError {
  AliasNotFound,
  FileReadFailed,
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Reg {
  x0 = 0,
  ra = 1,
  sp = 2,
  s0 = 3,
  s1 = 4,
  s2 = 5,
  s3 = 6,
  s4 = 7,
  s5 = 8,
  s6 = 9,
  s7 = 10,
  a0 = 11,
  a1 = 12,
  a2 = 13,
  a3 = 14,
  a4 = 15,
  a5 = 16,
  a6 = 17,
  a7 = 18,
  t0 = 19,
  t1 = 20,
  t2 = 21,
  t3 = 22,
  t4 = 23,
  t5 = 24,
  t6 = 25,
  t7 = 26,
  t8 = 27,
  t9 = 28,
  t10 = 29,
  t11 = 30,
  t12 = 31,
}

impl Reg {
  pub fn parse(val: u8) -> Result<Self, Error> {
    if val <= 31 {
      return Ok(unsafe { core::mem::transmute(val) });
    }
    Err(Error::ParseError)
  }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Opcode {
  halt = 0b0_00000,
  ecall = 0b1_00000,
  sb = 0b0_00010,
  lb = 0b1_00010,
  sh = 0b0_00100,
  lh = 0b1_00100,
  sw = 0b0_00110,
  lw = 0b1_00110,
  lbu = 0b1_01000,
  lhu = 0b1_01010,
  lui = 0b1_01100,
  la = 0b1_01110,
  jal = 0b0_11110,
  add = 0b0_00001,
  addi = 0b1_00001,
  and = 0b0_00011,
  andi = 0b1_00011,
  or = 0b0_00101,
  ori = 0b1_00101,
  xor = 0b0_00111,
  xori = 0b1_00111,
  sll = 0b0_01001,
  slli = 0b1_01001,
  srl = 0b0_01011,
  srli = 0b1_01011,
  sra = 0b0_01101,
  sub = 0b0_01111,
  mul = 0b0_10001,
  muli = 0b1_10001,
  slt = 0b0_10011,
  blt = 0b1_10011,
  sltu = 0b0_10101,
  bltu = 0b1_10101,
  beq = 0b1_10111,
  bne = 0b1_11001,
  bge = 0b1_11011,
  bgeu = 0b1_11101,
  sltiu = 0b0_11111,
}

impl Opcode {
  pub fn parse(val: u8) -> Result<Self, Error> {
    // TODO: implement safe parsing
    Ok(unsafe { core::mem::transmute(val) })
  }
}

pub struct Instruction {
  opcode: Opcode,
  r1: Reg,
  r2: Reg,
  r3: Reg,
  imm: i16,
}

impl Instruction {
  pub fn new(opcode: Opcode, r1: Reg, r2: Reg, r3: Reg, imm: i16) -> Self {
    Instruction {
      opcode,
      r1,
      r2,
      r3,
      imm,
    }
  }

  pub fn parse(word: u32) -> Result<Self, Error> {
    let opcode = Opcode::parse(word as u8 & 0x3f)?;
    let fst = Reg::parse((word >> 6) as u8 & 0x1f)?;
    let snd = Reg::parse((word >> 11) as u8 & 0x1f)?;

    match get_instructions_type(opcode) {
      InstructionType::RM | InstructionType::RO => {
        let r3 = Reg::parse((word >> 16) as u8 & 0x1f)?;
        let imm = (word >> 21) as i16;
        Ok(Instruction::new(opcode, fst, snd, r3, imm))
      }
      InstructionType::RI => {
        let imm = (word >> 16) as i16;
        Ok(Instruction::new(opcode, fst, snd, Reg::x0, imm))
      }
      InstructionType::RA => {
        let imm = (word >> 16) as i16;
        Ok(Instruction::new(opcode, fst, Reg::x0, snd, imm))
      }
    }
  }

  pub fn build(&self) -> u32 {
    match get_instructions_type(self.opcode) {
      InstructionType::RM | InstructionType::RO => {
        let mut word = (self.imm as u32) << 21;
        word |= (self.r3 as u32 & 0x1f) << 16;
        word |= (self.r2 as u32 & 0x1f) << 11;
        word |= (self.r1 as u32 & 0x1f) << 6;
        word |= self.opcode as u32 & 0x3f;
        word
      }
      InstructionType::RA => {
        let mut word = (self.imm as u32) << 16;
        word |= (self.r3 as u32 & 0x1f) << 11;
        word |= (self.r1 as u32 & 0x1f) << 6;
        word |= self.opcode as u32 & 0x3f;
        word
      }
      InstructionType::RI => {
        let mut word = (self.imm as u32) << 16;
        word |= (self.r2 as u32 & 0x1f) << 11;
        word |= (self.r1 as u32 & 0x1f) << 6;
        word |= self.opcode as u32 & 0x3f;
        word
      }
    }
  }
}

enum InstructionType {
  RA,
  RM,
  RO,
  RI,
}

fn get_instructions_type(opcode: Opcode) -> InstructionType {
  match opcode {
    Opcode::halt
    | Opcode::ecall
    | Opcode::lb
    | Opcode::lbu
    | Opcode::lh
    | Opcode::lhu
    | Opcode::lw
    | Opcode::la
    | Opcode::sb
    | Opcode::sh
    | Opcode::sw => InstructionType::RA,
    Opcode::lui
    | Opcode::addi
    | Opcode::andi
    | Opcode::muli
    | Opcode::ori
    | Opcode::slli
    | Opcode::sltiu
    | Opcode::srli
    | Opcode::xori => InstructionType::RI,
    Opcode::add
    | Opcode::and
    | Opcode::mul
    | Opcode::or
    | Opcode::sll
    | Opcode::slt
    | Opcode::sltu
    | Opcode::sra
    | Opcode::srl
    | Opcode::sub
    | Opcode::xor => InstructionType::RM,
    Opcode::jal
    | Opcode::beq
    | Opcode::bne
    | Opcode::bge
    | Opcode::blt
    | Opcode::bgeu
    | Opcode::bltu => InstructionType::RO,
  }
}

impl core::fmt::Debug for Instruction {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{:?} {:?} {:?} {:?} {:?}",
      self.opcode, self.r1, self.r2, self.r3, self.imm,
    )
  }
}
