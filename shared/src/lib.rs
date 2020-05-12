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
  X0 = 0,
  X1 = 1,
  X2 = 2,
  X3 = 3,
  X4 = 4,
  X5 = 5,
  X6 = 6,
  X7 = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
  ADDI = 0,
  ADD = 9,
  AND,
  ANDI,
  BEQ,
  BGE,
  BGEU,
  BLT,
  BLTU,
  BNE,
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
