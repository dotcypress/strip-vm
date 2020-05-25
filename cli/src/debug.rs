use strip_shared::vm::*;

pub struct Trace<'input> {
  vm: VM<'input, Environment>,
  max_ops: Option<u32>,
  spins: u16,
  ops: u32,
}

impl<'input> Trace<'input> {
  pub fn new(
    spins: u16,
    max_ops: Option<u32>,
    ram_size: u16,
    trace_memory: bool,
    bytecode: &'input [u8],
  ) -> Result<Self, VMError> {
    let mut vm = VM::new(Environment {
      trace_memory,
      ram: vec![0; ram_size as usize],
    });
    vm.load(bytecode)?;
    Ok(Trace {
      vm,
      spins,
      max_ops,
      ops: 0,
    })
  }

  pub fn start(&mut self) -> Result<(), VMError> {
    while self.spins > 0 {
      println!("{:<4} {:?}", self.ops, self.vm);
      match self.vm.step() {
        Ok(true) => {
          println!("{:=<80}", "VM HALTED   ");
          self.spins -= 1;
          self.vm.rewind();
        }
        Ok(_) => {
          self.ops += 1;
          if let Some(max_ops) = self.max_ops {
            if self.ops >= max_ops {
              return Ok(());
            }
          }
        }
        Err(err) => return Err(err),
      }
    }
    Ok(())
  }
}

pub struct Environment {
  trace_memory: bool,
  ram: Vec<u8>,
}

impl Env for Environment {
  type Error = ();

  fn reset(&mut self) {
    self.ram = vec![0; self.ram.len()];
  }

  fn mem_fetch(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if self.trace_memory {
      println!("MEM  FETCH  0x{:x}", offset);
    }
    if offset >= 0x1000 {
      return Ok(());
    }
    let end = offset + buf.len();
    if end > self.ram.len() {
      return Err(());
    }
    buf.copy_from_slice(&self.ram[offset..end]);
    Ok(())
  }

  fn mem_set(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if self.trace_memory {
      println!("MEM  SET    0x{:x} {:?}", offset, val);
    }
    if offset >= 0x1000 {
      return Ok(());
    }
    let end = offset + val.len();
    if end > self.ram.len() {
      return Err(());
    }
    self.ram[offset..end].copy_from_slice(val);
    Ok(())
  }

  fn ecall(&mut self, ecall: i32, param: i32) -> Result<i32, Self::Error> {
    println!("ECALL       0x{:x}(0x{:x?})", ecall, param);
    Ok(0)
  }
}

impl core::fmt::Debug for Environment {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self.ram)
  }
}
