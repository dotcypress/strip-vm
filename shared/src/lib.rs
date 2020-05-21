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
  ADD = 0b0000000,
  ADDI = 0b0000001,
  LA = 0b0000100,
  LUI = 0b0000101,
  AND = 0b0001000,
  ANDI = 0b0001001,
  OR = 0b0010000,
  ORI = 0b0010001,
  XOR = 0b0011000,
  XORI = 0b0011001,
  SLL = 0b0100000,
  SLLI = 0b0100001,
  SRL = 0b0101000,
  SRLI = 0b0101001,
  SRA = 0b0110000,
  SUB = 0b0111000,
  MUL = 0b1000000,
  ECALL = 0b1000001,
  SLT = 0b1001000,
  BLT = 0b1001010,
  SLTU = 0b1010000,
  SLTIU = 0b1010001,
  BLTU = 0b1010010,
  BEQ = 0b1011010,
  BNE = 0b1100010,
  BGE = 0b1101010,
  BGEU = 0b1110010,
  SB = 0b1111000,
  SH = 0b1111001,
  SW = 0b1111010,
  LBU = 0b1111011,
  LB = 0b1111100,
  LH = 0b1111101,
  LW = 0b1111110,
  LHU = 0b1111111,
}
