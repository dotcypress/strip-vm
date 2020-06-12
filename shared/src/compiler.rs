use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::parser::*;
use crate::*;
use byteorder::{BigEndian, ByteOrder};

pub fn compile(exprs: &[Exp]) -> Result<Vec<u8>, Error> {
  let mut aliases: HashMap<&str, Reg> = HashMap::new();
  let mut consts: HashMap<&str, i16> = HashMap::new();
  let mut labels: HashMap<&str, i16> = HashMap::new();
  let mut words: Vec<&Word> = Vec::with_capacity(2048);
  let mut mem: Vec<u8> = Vec::with_capacity(4096);
  let mut prog_started = false;

  for exp in exprs {
    match exp {
      Exp::Comment(_) => {}
      Exp::Label(label) => {
        let offset = if prog_started { words.len() } else { mem.len() };
        labels.insert(label, offset as i16);
      }
      Exp::Word(word) => {
        words.push(word);
        prog_started = true;
      }
      Exp::Directive(dir) => {
        match dir {
          Directive::Constant(ident, val) => {
            consts.insert(ident, *val);
          }
          Directive::Alias(ident, reg) => {
            aliases.insert(ident, *reg);
          }
          Directive::Zero(size) => {
            for _ in 0..*size {
              mem.push(0);
            }
          }
          Directive::Byte(data) => {
            mem.extend(data);
          }
          Directive::Half(data) => {
            let mut buf = [0; 2];
            for half in data {
              BigEndian::write_u16(&mut buf, *half);
              mem.extend(&buf);
            }
          }
          Directive::Word(data) => {
            let mut buf = [0; 4];
            for word in data {
              BigEndian::write_u32(&mut buf, *word);
              mem.extend(&buf);
            }
          }
          Directive::IncBin(file) => {
            let mut file = File::open(file)
              .map_err(|_| Error::CompilerError(CompilerError::FileReadFailed))?;
            let mut buf = Vec::new();
            file
              .read_to_end(&mut buf)
              .map_err(|_| Error::CompilerError(CompilerError::FileReadFailed))?;
            mem.extend(buf);
          }
        };
      }
    }
  }

  if words.is_empty() {
    return Ok(vec![]);
  }

  let resolve_reg = |reg_link| match reg_link {
    RegLink::Direct(reg) => Ok(reg),
    RegLink::Alias(ident) => match aliases.get(ident) {
      Some(reg) => Ok(*reg),
      None => Err(Error::CompilerError(CompilerError::AliasNotFound)),
    },
  };

  let mut prog: Vec<u8> = Vec::with_capacity(4096);
  prog.extend(&[0xaf, 0xaf]);

  let mut buf = [0; 2];
  BigEndian::write_u16(&mut buf, mem.len() as u16);
  prog.extend(&buf);
  prog.extend(&mem);

  let mut buf = [0; 4];
  for (pc, word) in words.iter().enumerate() {
    let (r3, imm) = if let Some(imm) = &word.imm {
      let mut val = imm.val;
      if let Some(ident) = imm.ident {
        if ident == "pc" {
          val += pc as i16;
        } else if let Some(offset) = consts.get(ident) {
          val += offset;
        } else if let Some(offset) = labels.get(ident) {
          val += offset;
        } else {
          return Err(Error::CompilerError(CompilerError::AliasNotFound));
        }
      }
      (imm.reg, val)
    } else {
      (word.r3, 0)
    };

    let inst = Instruction::new(
      word.opcode,
      resolve_reg(word.r1)?,
      resolve_reg(word.r2)?,
      resolve_reg(r3)?,
      imm,
    );
    BigEndian::write_u32(&mut buf, inst.build());
    prog.extend(&buf);
  }

  Ok(prog)
}
