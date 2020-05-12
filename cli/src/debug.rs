use strip_shared::vm::*;

pub struct Trace<'input> {
  vm: VM<'input, HostMock>,
  spins: u16,
}

impl<'input> Trace<'input> {
  pub fn new(
    spins: u16,
    ram_size: u16,
    trace_memory: bool,
    bytecode: &'input [u8],
  ) -> Result<Self, VMError> {
    let mut vm = VM::new(HostMock {
      trace_memory,
      ram: vec![0; ram_size as usize],
    });
    vm.load(bytecode)?;
    Ok(Trace { vm, spins })
  }

  pub fn start(&mut self) -> Result<(), VMError> {
    while self.spins > 0 {
      match self.vm.step() {
        Err(VMError::ProgramHalted) => {
          self.spins -= 1;
          self.vm.rewind();
          println!("{:=<72}", "");
        }
        Err(err) => return Err(err),
        _ => {
          println!("{:?}", self.vm);
        }
      }
    }
    Ok(())
  }
}

pub struct HostMock {
  trace_memory: bool,
  ram: Vec<u8>,
}

impl Host for HostMock {
  type Error = ();

  fn reset(&mut self) {
    self.ram = vec![0; self.ram.len()];
  }

  fn fetch_mem(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if self.trace_memory {
      println!("MEM     FETCH 0x{:x}", offset);
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

  fn store_mem(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if self.trace_memory {
      println!("MEM     STORE 0x{:<9x}{:?}", offset, val);
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
}

impl core::fmt::Debug for HostMock {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "{:?}", self.ram)
  }
}
