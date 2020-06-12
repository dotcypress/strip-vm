package strip

import chisel3._

class TangNano extends Module {
  val io = IO(new Bundle {
    val button = Input(Bool())
    val led_blue = Output(Bool())
    val led_red = Output(Bool())
    val led_green = Output(Bool())
    val lcd_clock = Output(Clock())
    val lcd_hsync = Output(Bool())
    val lcd_vsync = Output(Bool())
    val lcd_de = Output(Bool())
    val lcd_green = Output(UInt(6.W))
    val lcd_red = Output(UInt(5.W))
    val lcd_blue = Output(UInt(5.W))
  })

  io.led_red := io.button
  io.led_blue := ~io.button
  io.led_green := true.B

  val pll = Module(new RPLL())
  pll.io.clkin := clock
  io.lcd_clock := pll.io.clkoutd

  val display = withClock(io.lcd_clock) { 
    Module(new ILI6122()) 
  }

  io.lcd_hsync := display.io.hsync
  io.lcd_vsync := display.io.vsync
  io.lcd_de := display.io.de
  io.lcd_green := display.io.green
  io.lcd_red := display.io.red
  io.lcd_blue := display.io.blue
}

class RPLL extends BlackBox {
  val io = IO(new Bundle {
    val clkin = Input(Clock())
    val clkoutd = Output(Clock())
  })
}