use hal::hal::spi::FullDuplex;
use rgb::FromSlice;
use smart_leds::SmartLedsWrite;
use strip_shared::vm::*;
use ws2812_spi::Ws2812;

const LEDS: usize = 300;

pub struct LedStrip<SPI> {
  link: Ws2812<SPI>,
  vm: VM<'static, VMHost>,
}

impl<SPI> LedStrip<SPI>
where
  SPI: FullDuplex<u8>,
{
  pub fn new(spi: SPI) -> LedStrip<SPI> {
    let link = Ws2812::new(spi);
    let mut vm = VM::new(VMHost {
      ram: [0; 1024],
      led_buf: [0; LEDS * 3],
    });
    vm.load(include_bytes!("../../docs/blinky.bin")).unwrap();
    LedStrip { vm, link }
  }

  pub fn refresh(&mut self) {
    self.vm.spin().unwrap();
    self
      .link
      .write(self.vm.get_host().led_buf.as_rgb().iter().cloned())
      .ok();
  }
}

pub enum HostError {
  MemoryOverread,
}

pub struct VMHost {
  ram: [u8; 1024],
  led_buf: [u8; LEDS * 3],
}

impl Host for VMHost {
  type Error = HostError;

  fn reset(&mut self) {
    for byte in self.ram.iter_mut() {
      *byte = 0;
    }
    for byte in self.led_buf.iter_mut() {
      *byte = 0;
    }
  }

  fn fetch_mem(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if offset >= 0x1000 {
      let offset = offset - 0x1000;
      let end = offset + buf.len();
      buf.copy_from_slice(&self.ram[offset..end]);
      return Ok(());
    }
    let end = offset + buf.len();
    if end > self.ram.len() {
      return Err(HostError::MemoryOverread);
    }
    buf.copy_from_slice(&self.ram[offset..end]);
    Ok(())
  }

  fn store_mem(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if offset >= 0x1000 {
      let offset = offset - 0x1000;
      let end = offset + val.len();
      self.led_buf[offset..end].copy_from_slice(val);
      return Ok(());
    }

    let end = offset + val.len();
    if end > self.ram.len() {
      return Err(HostError::MemoryOverread);
    }
    self.ram[offset..end].copy_from_slice(val);
    Ok(())
  }
}
