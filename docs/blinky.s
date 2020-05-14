# Strip size: 300 LED's
# SPI transfer time: 7.246us,  max fps: 136Hz
# VM spin time:      8.216us,  max fps: 126Hz
# Total:             15.451us, max fps: 64Hz

.equ STRIP_SIZE 900 # leds * 3 color components
.equ STRIP_BASE 0x1000
.equ PRESCALER 24

ecall ra PRESCALER
j start

reset:
  li s0 STRIP_SIZE
  dec s0
  li s2 0x22

start:
  li s1 STRIP_SIZE
  bltz s0 reset

loop:
  dec s1
  bne s0 s1 off

on:
  sb s2 STRIP_BASE(s1)
  j until

off:
  sb zero STRIP_BASE(s1)

until:
  bnez s1 loop

dec s0