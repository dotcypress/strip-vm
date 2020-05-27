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
  ADD = 0b000_0000,
  ADDI = 0b000_0001,
  LA = 0b000_0100,
  LUI = 0b000_0101,
  AND = 0b000_1000,
  ANDI = 0b000_1001,
  OR = 0b001_0000,
  ORI = 0b001_0001,
  XOR = 0b001_1000,
  XORI = 0b001_1001,
  SLL = 0b010_0000,
  SLLI = 0b010_0001,
  SRL = 0b010_1000,
  SRLI = 0b010_1001,
  SRA = 0b011_0000,
  SUB = 0b011_1000,
  MUL = 0b100_0000,
  ECALL = 0b100_0001,
  SLT = 0b100_1000,
  BLT = 0b100_1010,
  SLTU = 0b101_0000,
  SLTIU = 0b101_0001,
  BLTU = 0b101_0010,
  BEQ = 0b101_1010,
  BNE = 0b110_0010,
  BGE = 0b110_1010,
  BGEU = 0b111_0010,
  SB = 0b111_1000,
  SH = 0b111_1001,
  SW = 0b111_1010,
  LBU = 0b111_1011,
  LB = 0b111_1100,
  LH = 0b111_1101,
  LW = 0b111_1110,
  LHU = 0b111_1111,
}
