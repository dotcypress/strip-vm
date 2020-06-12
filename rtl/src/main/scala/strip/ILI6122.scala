package strip

import chisel3._

class ILI6122 extends Module {
  val io = IO(new Bundle {
    val hsync = Output(Bool())
    val vsync = Output(Bool())
    val de = Output(Bool())
    val green = Output(UInt(6.W))
    val red = Output(UInt(5.W))
    val blue = Output(UInt(5.W))
  })

  val HDisplay = 800.U
  val HSync = 2.U
  val HBPorch = 182.U
  val HFPorch = 210.U

  val VDisplay = 480.U
  val VSync = 5.U
  val VBPorch = 6.U
  val VFPorch = 62.U

  val HPeriod = HBPorch + HDisplay + HFPorch
  val VPeriod = VBPorch + VDisplay + VFPorch

  val hPos = RegInit(0.U(16.W))
  val vPos = RegInit(0.U(16.W))

  when (hPos >= HPeriod) {
    hPos := 0.U
    vPos := vPos + 1.U
  } .elsewhen (vPos >= VPeriod) { 
    vPos := 0.U
  } .otherwise {
    hPos := hPos + 1.U 
  }

  val hActive = (hPos > HBPorch) && (hPos <= (HBPorch + HDisplay))
  val vActive = (vPos > VBPorch) && (vPos <= (VBPorch + VDisplay - 1.U))
  val drawing = hActive && vActive

  io.hsync := hPos >= HSync
  io.vsync := vPos >= VSync
  io.de := drawing

  // val x = hPos - HBPorch
  // val y = vPos - VBPorch

  io.red := hPos(10, 6)
  io.blue := vPos(7, 3)
  io.green := hPos(5, 0)
}
