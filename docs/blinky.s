# Strip size: 300 LED's
# Core clock: 48MHz
# Frame ops:         3608,  vm clk: 450KHz
# SPI Transfer time: 7246us,   fps: 136Hz
# Frame render time: 8023us,   fps: 124Hz
# Total frame time:  15269us,  fps: 65Hz

.equ STRIP_SIZE  900 # 300 leds * 3 color components
.equ STRIP_BASE  0x1000

li x2 STRIP_SIZE
beqz x1 reset
j next

reset:
  li x1 STRIP_SIZE
  dec x1
  li x3 0x22

next:
  dec x2
  bne x1 x2 off

on:
  sb x3 STRIP_BASE(x2)
  j done

off:
  sb x0 STRIP_BASE(x2)

done:
  bnez x2 next

dec x1