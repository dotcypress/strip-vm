# Strip assembler reference

## Registers

Register | Alias  | Description
---------|--------|----------------------------------------------
none     | pc     | Program counter (readonly, compile time only)
x0       | zero   | Hard-wired zero
x1       | ra     | Return address
x2       | sp     | Stack pointer
x3-x10   | s0-s7  | Saved registers
x11-x18  | a0-a7  | Function arguments
x19-x31  | t0-t12 | Temporary registers

## Assembler Directives

Directive    | Arguments  | Description
-------------|------------|--------------------
.equ         | name value | Constant definition
.alias, .def | name reg   | Register alias definition

## Assembler RAM Directives

Directive | Arguments | Description
----------|-----------|-------------------
.zero     | number    | Emit zeros
.string   | string    | Emit string
.incbin   | filename  | Emit binary file
.byte     |           | Emit 8-bit words
.half     |           | Emit 16-bit words
.word     |           | Emit 32-bit words

## Assembler Instructions

Instruction           | Enc |  Description
----------------------|-----|-------------------------------------
halt                  | RA  | Halt program
ecall rd     addr(rl) | RA  | Request to the execution environment
lb    rd     addr(rl) | RA  | Load 8-bit word
lbu   rd     addr(rl) | RA  | Load 8-bit word, unsigned
lh    rd     addr(rl) | RA  | Load 16-bit word
lhu   rd     addr(rl) | RA  | Load 16-bit word, unsigned
lw    rd     addr(rl) | RA  | Load 32-bit word
la    rs     addr(rl) | RA  | Load address
sb    rs     addr(rl) | RA  | Store 8-bit word
sh    rs     addr(rl) | RA  | Store 16-bit word
sw    rs     addr(rl) | RA  | Store 32-bit word
lui   rd     imm      | RI  | Load upper 16-bit
addi  rd rs  imm      | RI  | rd = rs + imm
andi  rd rs  imm      | RI  | rd = rs & imm
muli  rd rs  imm      | RI  | rd = rs * imm
ori   rd rs  imm      | RI  | rd = rs \| imm
slli  rd rs  imm      | RI  | rd = rs << imm
sltiu rd rs  imm      | RI  | rd = rs < imm, unsigned
srli  rd rs  imm      | RI  | rd = rs >> imm
xori  rd rs  imm      | RI  | rd = rs ^ imm
add   rd rs1 rs2      | RM  | rd = rs1 + rs2
and   rd rs1 rs2      | RM  | rd = rs1 & rs2
mul   rd rs1 rs2      | RM  | rd = rs1 * rs2
or    rd rs1 rs2      | RM  | rd = rs1 \| rs2
sll   rd rs1 rs2      | RM  | rd = rs1 << rs2
slt   rd rs1 rs2      | RM  | rd = rs1 < rs2
sltu  rd rs1 rs2      | RM  | rd = rs1 < rs2, unsigned
sra   rd rs1 rs2      | RM  | rd = rs1 >>> rs2
srl   rd rs1 rs2      | RM  | rd = rs1 >> rs2
sub   rd rs1 rs2      | RM  | rd = rs1 - rs2
xor   rd rs1 rs2      | RM  | rd = rs1 ^ rs2
jal          offs(rl) | RO  | Call subroutine ; ra <- pc + 1
beq   rs rt  offs(rl) | RO  | Branch if rs = rt 
bne   rs rt  offs(rl) | RO  | Branch if rs !=rt 
bge   rs rt  offs(rl) | RO  | Branch if rs >=rt 
blt   rs rt  offs(rl) | RO  | Branch if rs < rt 
bgeu  rs rt  offs(rl) | RO  | Branch if rs >=rt, unsigned
bltu  rs rt  offs(rl) | RO  | Branch if rs < rt,  unsigned

## Pseudo-instructions

Instruction     | Expansion       | Description
----------------|-----------------|-----------------------
nop             | add x0 x0 x0    | No operation
ret             | beq x0 x0 ra    | Jump to ra register
j    addr       | beq x0 x0 addr  | Jump to address
inc  rd         | addi rd rd 1    | Increment register
dec  rd         | addi rd rd -1   | Decrement register
li   rd    imm  | addi rd x0 imm  | Load immediate (lower 16-bit)
mv   rd rs      | addi rd rs 0    | Copy register
not  rd rs      | xori rd rs -1   | One’s complement
neg  rd rs      | sub rd x0 rs    | Two’s complement
seqz rd rs      | sltiu rd rs 1   | Set if = zero
snez rd rs      | sltu rd x0 rs   | Set if != zero
sltz rd rs      | slt rd rs x0    | Set if < zero
sgtz rd rs      | slt rd x0 rs    | Set if > zero
beqz    rs addr | beq rs x0 addr  | Branch if = zero ; ra <- pc + 1
bnez    rs addr | bne rs x0 addr  | Branch if != zero ; ra <- pc + 1
blez    rs addr | bge x0 rs addr  | Branch if <= zero ; ra <- pc + 1
bgez    rs addr | bge rs x0 addr  | Branch if ≥ zero ; ra <- pc + 1
bltz    rs addr | blt rs x0 addr  | Branch if < zero ; ra <- pc + 1
bgtz    rs addr | blt x0 rs addr  | Branch if > zero ; ra <- pc + 1
bgt  rs rt addr | blt rt rs addr  | Branch if > ; ra <- pc + 1
ble  rs rt addr | bge rt rs addr  | Branch if <= ; ra <- pc + 1
bgtu rs rt addr | bltu rt rs addr | Branch if >, unsigned ; ra <- pc + 1
bleu rs rt addr | bltu rt rs addr | Branch if <=, unsigned ; ra <- pc + 1

## Instructions layout

### RA

Addr    | r3      | r1     | Opcode
--------|---------|--------|-------
[31:16] | [15:11] | [10:6] | [5:0]

### RI

Imm     | r2      | r1     | Opcode
--------|---------|--------|-------
[31:16] | [15:11] | [10:6] | [5:0]

### RM

Reserved | r3      | r2      | r1     | Opcode
---------|---------|---------|--------|-------
[31:21]  | [20:16] | [15:11] | [10:6] | [5:0]

### RO

Offset  | r3      | r2      | r1     | Opcode
--------|---------|---------|--------|-------
[31:21] | [20:16] | [15:11] | [10:6] | [5:0]

## Opcodes layout

X | FN   | A | Opcode
--|------|---|-------
0 | 0000 | 0 | halt
1 | 0000 | 0 | ecall
0 | 0001 | 0 | sb
1 | 0001 | 0 | lb
0 | 0010 | 0 | sh
1 | 0010 | 0 | lh
0 | 0011 | 0 | sw
1 | 0011 | 0 | lw
1 | 0100 | 0 | lbu
1 | 0101 | 0 | lhu
1 | 0110 | 0 | lui
1 | 0111 | 0 | la
0 | 1111 | 0 | jal
0 | 0000 | 1 | add
1 | 0000 | 1 | addi
0 | 0001 | 1 | and
1 | 0001 | 1 | andi
0 | 0010 | 1 | or
1 | 0010 | 1 | ori
0 | 0011 | 1 | xor
1 | 0011 | 1 | xori
0 | 0100 | 1 | sll
1 | 0100 | 1 | slli
0 | 0101 | 1 | srl
1 | 0101 | 1 | srli
0 | 0110 | 1 | sra
0 | 0111 | 1 | sub
0 | 1000 | 1 | mul
1 | 1000 | 1 | muli
0 | 1001 | 1 | slt
1 | 1001 | 1 | blt
0 | 1010 | 1 | sltu
1 | 1010 | 1 | bltu
1 | 1011 | 1 | beq
1 | 1100 | 1 | bne
1 | 1101 | 1 | bge
1 | 1110 | 1 | bgeu
0 | 1111 | 1 | sltiu