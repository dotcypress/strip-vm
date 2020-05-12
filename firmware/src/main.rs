#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_semihosting;
extern crate rtfm;
extern crate stm32g0xx_hal as hal;

mod led_strip;

use hal::gpio::*;
use hal::prelude::*;
use hal::rcc::{self, PllConfig};
use hal::spi;
use hal::stm32;
use hal::time::Hertz;
use hal::timer;
use led_strip::LedStrip;
use rtfm::app;

type AnimationTimer = timer::Timer<stm32::TIM17>;
type SPIBus = spi::Spi<stm32::SPI2, (spi::NoSck, spi::NoMiso, gpioa::PA10<Input<Floating>>)>;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
  struct Resources {
    strip: LedStrip<SPIBus>,
    timer: AnimationTimer,
  }

  #[init]
  fn init(ctx: init::Context) -> init::LateResources {
    let pll_cfg = PllConfig::with_hsi(4, 24, 2);
    let rcc_cfg = rcc::Config::pll().pll_cfg(pll_cfg);
    let mut rcc = ctx.device.RCC.freeze(rcc_cfg);

    let mut timer = ctx.device.TIM17.timer(&mut rcc);
    timer.start(256.hz());
    timer.listen();

    let port_a = ctx.device.GPIOA.split(&mut rcc);
    let spi = ctx.device.SPI2.spi(
      (spi::NoSck, spi::NoMiso, port_a.pa10),
      spi::MODE_0,
      3.mhz(),
      &mut rcc,
    );

    let mut strip = LedStrip::new(spi);
    let stopwatch = ctx.device.TIM2.stopwatch(&mut rcc);
    let elapsed_us = stopwatch.trace(|| {
      strip.refresh();
    });
    let max_fps: Hertz = elapsed_us.into();
    cortex_m_semihosting::hprintln!("vm sin: {}us, max fps: {}Hz", elapsed_us.0, max_fps.0)
      .unwrap();

    init::LateResources { timer, strip }
  }

  #[task(binds = TIM17, resources = [timer, strip])]
  fn timer_tick(ctx: timer_tick::Context) {
    ctx.resources.strip.refresh();
    ctx.resources.timer.clear_irq();
  }
};
