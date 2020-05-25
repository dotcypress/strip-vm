# Strip size: 300 LED's
# SPI transfer time: 7.246us,  max fps: 136Hz
# VM spin time:      8.216us,  max fps: 126Hz
# Total:             15.451us, max fps: 64Hz

.equ STRIP_SIZE 900 # 300 leds * 3 color components
.equ STRIP_BASE 0x1000

.equ ECALL_SET_PSC 0
.equ PRESCALER 0x18

.equ LUMA 0x22

li ra PRESCALER
ecall ra ECALL_SET_PSC(ra)

j start

reset:
  li s0 STRIP_SIZE
  li s2 LUMA
  dec s0

start:
  bltz s0 reset
  li s1 STRIP_SIZE

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