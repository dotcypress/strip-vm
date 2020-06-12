j main
sum:
  add s0 s0 s1
  ret
main:
  li s0 10
  li s1 32
  jal sum
  halt