package build

object Elaborate extends App {
  chisel3.Driver.execute(args, () => new strip.TangNano())
}