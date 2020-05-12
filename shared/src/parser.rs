use crate::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub type Exprs<'a> = Vec<Exp<'a>>;
pub type Parser = grammar::StripParser;

pub fn parse(code: &str) -> Result<Exprs, Error> {
  Parser::new().parse(code).map_err(|err| {
    println!("{:?}", err);
    Error::ParseError
  })
}

#[derive(Debug)]
pub enum Directive<'a> {
  Constant(&'a str, i16),
  Byte(Vec<u8>),
  Half(Vec<u16>),
  Word(Vec<u32>),
  IncBin(&'a str),
  Zero(i16),
}

#[derive(Debug)]
pub struct Word<'a> {
  pub opcode: Opcode,
  pub r1: Reg,
  pub r2: Reg,
  pub r3: Reg,
  pub imm: i16,
  pub addr: Option<parser::Address<'a>>,
}

impl<'a> Word<'a> {
  pub fn new(
    opcode: Opcode,
    r1: Reg,
    r2: Reg,
    r3: Reg,
    imm: i16,
    addr: Option<parser::Address<'a>>,
  ) -> Self {
    Self {
      opcode,
      r1,
      r2,
      r3,
      imm,
      addr,
    }
  }
}

#[derive(Debug)]
pub enum Exp<'a> {
  Word(Word<'a>),
  Label(&'a str),
  Comment(&'a str),
  Directive(Directive<'a>),
}

#[derive(Debug)]
pub struct Address<'a> {
  pub(crate) reg: Reg,
  pub(crate) offset: i16,
  pub(crate) ident: Option<&'a str>,
}

impl<'a> Address<'a> {
  pub fn new(reg: Reg, offset: i16, ident: Option<&'a str>) -> Self {
    Self { reg, offset, ident }
  }
}
