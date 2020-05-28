use hal::hal::spi::FullDuplex;
use rgb::FromSlice;
use smart_leds::SmartLedsWrite;
use strip_shared::vm::*;
use ws2812_spi::Ws2812;

const LEDS: usize = 300;

pub struct LedStrip<SPI> {
  link: Ws2812<SPI>,
  vm: VM<'static, Environment>,
}

impl<SPI> LedStrip<SPI>
where
  SPI: FullDuplex<u8>,
{
  pub fn new(spi: SPI) -> LedStrip<SPI> {
    let link = Ws2812::new(spi);
    let mut vm = VM::new(Environment {
      ram: [0; 1024],
      led_ram: [0; LEDS * 3],
      ops: 0,
      psc: 0,
    });
    vm.load(include_bytes!("../../docs/rainbow.bin")).unwrap();
    LedStrip { vm, link }
  }

  pub fn refresh(&mut self) {
    let env = self.vm.get_env();
    if env.psc > 0 && env.ops < env.psc {
      env.ops += 1;
      return;
    }
    env.ops = 0;
    self.vm.respin().ok();
    self
      .link
      .write(self.vm.get_env().led_ram.as_rgb().iter().cloned())
      .ok();
  }
}

pub enum StripError {
  MemoryOverread,
}

pub struct Environment {
  ops: u32,
  psc: u32,
  ram: [u8; 1024],
  led_ram: [u8; LEDS * 3],
}

impl Env for Environment {
  type Error = StripError;

  fn reset(&mut self) {
    for byte in self.ram.iter_mut() {
      *byte = 0;
    }
    for byte in self.led_ram.iter_mut() {
      *byte = 0;
    }
  }

  fn mem_fetch(&self, addr: u16, buf: &mut [u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if offset >= 0x1000 {
      let offset = offset - 0x1000;
      let end = offset + buf.len();
      buf.copy_from_slice(&self.led_ram[offset..end]);
      return Ok(());
    }
    let end = offset + buf.len();
    if end > self.ram.len() {
      return Err(StripError::MemoryOverread);
    }
    buf.copy_from_slice(&self.ram[offset..end]);
    Ok(())
  }

  fn mem_set(&mut self, addr: u16, val: &[u8]) -> Result<(), Self::Error> {
    let offset = addr as usize;
    if offset >= 0x1000 {
      let offset = offset - 0x1000;
      let end = offset + val.len();
      self.led_ram[offset..end].copy_from_slice(val);
      return Ok(());
    }

    let end = offset + val.len();
    if end > self.ram.len() {
      return Err(StripError::MemoryOverread);
    }
    self.ram[offset..end].copy_from_slice(val);
    Ok(())
  }

  fn ecall(&mut self, ecall: i32, param: i32) -> Result<i32, Self::Error> {
    match ecall {
      0 => {
        self.psc = param as u32;
      }
      1 => {
        let offset = param as usize - 0x1000;
        hsv2grb(&mut self.led_ram[offset..(offset + 3)]);
      }
      _=> {}
    }
    Ok(0)
  }
}

fn hsv2grb(buf: &mut [u8]) {
  let hue = buf[0];
  let sat = buf[1] as u16;
  let val = buf[2] as u16;
  let f = (hue as u16 * 2 % 85) * 3;
  let p: u16 = val * (255 - sat) / 255;
  let q: u16 = val * (255 - (sat * f) / 255) / 255;
  let t: u16 = val * (255 - (sat * (255 - f)) / 255) / 255;
  let grb = match hue {
    0..=42 => (t, val, p),
    43..=84 => (val, q, p),
    85..=127 => (val, p, t),
    128..=169 => (q, p, val),
    170..=212 => (p, t, val),
    213..=254 => (p, val, q),
    255 => (t, val, p),
  };
  buf[0] = grb.0 as u8;
  buf[1] = grb.1 as u8;
  buf[2] = grb.2 as u8;
}
