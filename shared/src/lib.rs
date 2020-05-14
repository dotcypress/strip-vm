#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod compiler;
#[cfg(feature = "std")]
pub mod parser;
pub mod vm;

#[derive(Debug)]
pub enum Error {
  ParseError,
  CompileError,
  VMError,
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
  Zero = 0,
  RA = 1,
  S0 = 2,
  S1 = 3,
  S2 = 4,
  S3 = 5,
  S4 = 6,
  S5 = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
  ADDI = 0,
  ADD,
  AND,
  ANDI,
  BEQ,
  BGE,
  BGEU,
  BLT,
  BLTU,
  BNE,
  ECALL,
  LA,
  LB,
  LBU,
  LH,
  LHU,
  LUI,
  LW,
  MUL,
  OR,
  ORI,
  SB,
  SH,
  SLL,
  SLLI,
  SLT,
  SLTU,
  SLTIU,
  SRA,
  SRL,
  SRLI,
  SUB,
  SW,
  XOR,
  XORI,
}
