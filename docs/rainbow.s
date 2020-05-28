# Total: 22604us, max fps: 44
.alias psc s0
.alias hue s1
.alias sv s2
.alias led s3

.equ PRESCALER 24
.equ MAX_HUE 255
.equ SV 0xff20
.equ STRIP_BASE 0x1000
.equ STRIP_SIZE 300
.equ HSV2RGB 0x1
.equ SET_PSC 0x0

li psc PRESCALER
ecall zero SET_PSC(psc)

li sv SV

li led STRIP_SIZE
muli led led 3
la led STRIP_BASE(led)

loop:
  dec hue
  bgez hue 2(pc)
  li hue MAX_HUE

  addi led led -3
  sb hue (led)
  sh sv 1(led)
  ecall zero HSV2RGB(led)

  bnez led loop