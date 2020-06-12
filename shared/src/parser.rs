use crate::*;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(grammar);

pub type Exprs<'a> = Vec<Exp<'a>>;
pub type Parser = grammar::StripParser;

pub fn parse(code: &str) -> Result<Exprs, Error> {
  Parser::new().parse(code).map_err(|err| {
    println!("Parser error: {:?}", err);
    Error::ParseError
  })
}

#[derive(Debug)]
pub enum Directive<'a> {
  Constant(&'a str, i16),
  Alias(&'a str, Reg),
  Byte(Vec<u8>),
  Half(Vec<u16>),
  Word(Vec<u32>),
  IncBin(&'a str),
  Zero(i16),
}

#[derive(Debug)]
pub struct Word<'a> {
  pub opcode: Opcode,
  pub r1: RegLink<'a>,
  pub r2: RegLink<'a>,
  pub r3: RegLink<'a>,
  pub imm: Option<parser::Immediate<'a>>,
}

impl<'a> Word<'a> {
  pub fn new(
    opcode: Opcode,
    r1: RegLink<'a>,
    r2: RegLink<'a>,
    r3: RegLink<'a>,
    imm: Option<parser::Immediate<'a>>,
  ) -> Self {
    Self {
      opcode,
      r1,
      r2,
      r3,
      imm,
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

#[derive(Debug, Clone, Copy)]
pub enum RegLink<'a> {
  Alias(&'a str),
  Direct(Reg),
}

impl<'a> RegLink<'a> {
  pub fn zero() -> Self {
    RegLink::Direct(Reg::x0)
  }
}

#[derive(Debug)]
pub struct Immediate<'a> {
  pub(crate) reg: RegLink<'a>,
  pub(crate) val: i16,
  pub(crate) ident: Option<&'a str>,
}

impl<'a> Immediate<'a> {
  pub fn relative(reg: RegLink<'a>, val: i16, ident: Option<&'a str>) -> Self {
    Self { reg, val, ident }
  }

  pub fn absolute(val: i16, ident: Option<&'a str>) -> Self {
    Self {
      reg: RegLink::zero(),
      val,
      ident,
    }
  }
}
