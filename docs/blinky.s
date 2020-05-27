# Strip size: 300 LED's
# SPI transfer time: 7.246us,  max fps: 136Hz
# VM spin time:      8.216us,  max fps: 126Hz
# Total:             15.451us, max fps: 64Hz

.equ STRIP_SIZE 900 # 300 leds * 3 color components
.equ STRIP_BASE 0x1000
.equ LUMA 0x22

.equ SET_PSC 0
.equ PRESCALER 0x18

.alias frame s0
.alias led_idx s1
.alias luma s2

li ra PRESCALER
ecall ra SET_PSC(ra)

j start

reset:
  li frame STRIP_SIZE
  li luma LUMA
  dec frame

start:
  bltz frame reset
  li led_idx STRIP_SIZE

loop:
  dec led_idx
  bne frame led_idx off

on:
  sb luma STRIP_BASE(led_idx)
  j until

off:
  sb zero STRIP_BASE(led_idx)

until:
  bnez led_idx loop

dec frame