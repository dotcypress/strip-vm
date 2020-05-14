use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::parser::*;
use crate::*;
use byteorder::{BigEndian, ByteOrder};

pub fn compile(exprs: &[Exp]) -> Result<Vec<u8>, Error> {
  let mut consts: HashMap<&str, i16> = HashMap::new();
  let mut labels: HashMap<&str, i16> = HashMap::new();
  let mut ops: Vec<&Word> = Vec::with_capacity(1024);
  let mut mem: Vec<u8> = Vec::with_capacity(4096);
  let mut prog_started = false;

  for exp in exprs {
    match exp {
      Exp::Comment(_) => {}
      Exp::Label(label) => {
        let offset = if prog_started { ops.len() } else { mem.len() };
        labels.insert(label, offset as i16);
      }
      Exp::Word(op) => {
        ops.push(op);
        prog_started = true;
      }
      Exp::Directive(dir) => {
        match dir {
          Directive::Constant(ident, val) => {
            consts.insert(ident, *val);
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
            let mut file = File::open(file).map_err(|_| Error::CompileError)?;
            let mut buf = Vec::new();
            file
              .read_to_end(&mut buf)
              .map_err(|_| Error::CompileError)?;
            mem.extend(buf);
          }
        };
      }
    }
  }

  if ops.len() == 0 {
    return Ok(vec![]);
  }

  let mut prog: Vec<u8> = Vec::with_capacity(4096);
  prog.extend(&[0xaf, 0xaf]);

  let mut buf = [0; 2];
  BigEndian::write_u16(&mut buf, mem.len() as u16);
  prog.extend(&buf);
  prog.extend(&mem);

  let mut buf = [0; 4];
  for (pc, op) in ops.iter().enumerate() {
    let (r3, imm) = if let Some(addr) = &op.addr {
      let mut offset = addr.offset;
      if let Some(ident) = addr.ident {
        if ident == "pc" {
          offset += pc as i16;
        } else if let Some(constant) = consts.get(ident) {
          offset += constant;
        } else if let Some(label) = labels.get(ident) {
          offset += label;
        } else {
          return Err(Error::CompileError);
        }
      }
      (addr.reg, (offset as i16).to_be_bytes())
    } else {
      (op.r3, op.imm.to_be_bytes())
    };
    let mut word = (imm[0] as u32) << 24;
    word |= (imm[1] as u32) << 16;
    word |= (r3 as u32 & 0x7) << 13;
    word |= (op.r2 as u32 & 0x7) << 10;
    word |= (op.r1 as u32 & 0x7) << 7;
    word |= op.opcode as u32 & 0x7f;
    BigEndian::write_u32(&mut buf, word);
    prog.extend(&buf);
  }

  Ok(prog)
}
