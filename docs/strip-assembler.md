# Strip assembler reference

## Registers

Register |   Description
---------|----------------
zero     | Hard-wired zero
ra       | Return address
s0-s5    | Saved registers

## Assembler Directives

Directive        | Arguments    | Description
-----------------|--------------|--------------------
`.equ`           | `name value` | Constant definition
`.alias`, `.def` | `name reg`   | Register alias definition

## Assembler RAM Directives

Directive  | Arguments    | Description
-----------|--------------|--------------------
`.zero`    | `number`     | Emit zeros
`.string`  | `string `    | Emit string
`.incbin`  | `filename`   | Emit binary file
`.byte`    |              | Emit 8-bit words
`.half`    |              | Emit 16-bit words
`.word`    |              | Emit 32-bit words

## Assembler Instructions

Instruction          | Description
-------------------- |---------------------------
`ecall rd  rs  imm`  | Request to the execution environment
`beq   rs  rt  addr` | Branch if = ; ra <- pc + 1
`bne   rs  rt  addr` | Branch if != ; ra <- pc + 1
`bge   rs  rt  addr` | Branch if >= ; ra <- pc + 1
`bgeu  rs  rt  addr` | Branch if >=, unsigned ; ra <- pc + 1
`blt   rs  rt  addr` | Branch if < ; ra <- pc + 1
`bltu  rs  rt  addr` | Branch if <, unsigned ; ra <- pc + 1
`add   rd  rs1 rs2`  | `rd = rs1 + rs2`
`addi  rd  rs  imm`  | `rd = rs + imm`
`and   rd  rs1 rs2`  | `rd = rs1 & rs2`
`andi  rd  rs  imm`  | `rd = rs & imm`
`mul   rd  rs1 rs2`  | `rd = rs1 * rs2`
`muli  rd  rs  imm`  | `rd = rs * imm`
`or    rd  rs1 rs2`  | `rd = rs1 \| rs2`
`ori   rd  rs  imm`  | `rd = rs \| imm`
`sll   rd  rs1 rs2`  | `rd = rs1 << rs2`
`slli  rd  rs  imm`  | `rd = rs << imm`
`slt   rd  rs1 rs2`  | `rd = rs1 < rs2`
`sltiu rd  rs  imm`  | `rd = rs < imm`, unsigned
`sltu  rd  rs1 rs2`  | `rd = rs1 < rs2`, unsigned
`sra   rd  rs1 rs2`  | `rd = rs1 >>> rs2`
`srl   rd  rs1 rs2`  | `rd = rs1 >> rs2`
`srli  rd  rs  imm`  | `rd = rs >> imm`
`sub   rd  rs1 rs2`  | `rd = rs1 - rs2`
`xor   rd  rs1 rs2`  | `rd = rs1 ^ rs2`
`xori  rd  rs  imm`  | `rd = rs ^ imm`
`lui   rd      imm`  | Load upper 16-bit
`la    rs      addr` | Load address
`lb    rd      addr` | Load 8-bit word
`lbu   rd      addr` | Load 8-bit word, unsigned
`lh    rd      addr` | Load 16-bit word
`lhu   rd      addr` | Load 16-bit word, unsigned
`lw    rd      addr` | Load 32-bit word
`sb    rs      addr` | Store 8-bit word
`sh    rs      addr` | Store 16-bit word
`sw    rs      addr` | Store 32-bit word

## Pseudo-instructions

Instruction       | Expansion         | Description
------------------|-------------------|-----------------------
`nop`             | `add x0 x0 x0`    | No operation
`ret`             | `beq x0 x0 ra`    | Jump to `ra` register
`j    addr`       | `beq x0 x0 addr`  | Jump to address
`inc  rd`         | `addi rd rd 1`    | Increment register
`dec  rd`         | `addi rd rd -1`   | Decrement register
`li   rd    imm`  | `addi rd x0 imm`  | Load immediate (lower 16-bit)
`mv   rd rs`      | `addi rd rs 0`    | Copy register
`not  rd rs`      | `xori rd rs -1`   | One’s complement
`neg  rd rs`      | `sub rd x0 rs`    | Two’s complement
`seqz rd rs`      | `sltiu rd rs 1`   | Set if = zero
`snez rd rs`      | `sltu rd x0 rs`   | Set if != zero
`sltz rd rs`      | `slt rd rs x0`    | Set if < zero
`sgtz rd rs`      | `slt rd x0 rs`    | Set if > zero
`beqz    rs addr` | `beq rs x0 addr`  | Branch if = zero ; ra <- pc + 1
`bnez    rs addr` | `bne rs x0 addr`  | Branch if != zero ; ra <- pc + 1
`blez    rs addr` | `bge x0 rs addr`  | Branch if <= zero ; ra <- pc + 1
`bgez    rs addr` | `bge rs x0 addr`  | Branch if ≥ zero ; ra <- pc + 1
`bltz    rs addr` | `blt rs x0 addr`  | Branch if < zero ; ra <- pc + 1
`bgtz    rs addr` | `blt x0 rs addr`  | Branch if > zero ; ra <- pc + 1
`bgt  rs rt addr` | `blt rt rs addr`  | Branch if > ; ra <- pc + 1
`ble  rs rt addr` | `bge rt rs addr`  | Branch if <= ; ra <- pc + 1
`bgtu rs rt addr` | `bltu rt rs addr` | Branch if >, unsigned ; ra <- pc + 1
`bleu rs rt addr` | `bltu rt rs addr` | Branch if <=, unsigned ; ra <- pc + 1

### Instruction layout

Imm/Addr  | rs2       | rs1       | rd      | Opcode 
----------|-----------|-----------|---------|--------
`[31:16]` | `[15:13]` | `[12:10]` | `[9:7]` | `[6:0]`

### Opcode layout

Func | R | B | I | Opcode
-----| --|---|---|--------
0000 | 0 | 0 | 0 | `add`
0000 | 0 | 0 | 1 | `addi`
0000 | 0 | 1 | 0 | `ecall`
0000 | 1 | 0 | 0 | `la`
0000 | 1 | 0 | 1 | `lui`
0001 | 0 | 0 | 0 | `and`
0001 | 0 | 0 | 1 | `andi`
0010 | 0 | 0 | 0 | `or`
0010 | 0 | 0 | 1 | `ori`
0011 | 0 | 0 | 0 | `xor`
0011 | 0 | 0 | 1 | `xori`
0100 | 0 | 0 | 0 | `sll`
0100 | 0 | 0 | 1 | `slli`
0101 | 0 | 0 | 0 | `srl`
0101 | 0 | 0 | 1 | `srli`
0110 | 0 | 0 | 0 | `sra`
0111 | 0 | 0 | 0 | `sub`
1000 | 0 | 0 | 0 | `mul`
1000 | 0 | 0 | 1 | `muli`
1001 | 0 | 0 | 0 | `slt`
1001 | 0 | 1 | 0 | `blt`
1010 | 0 | 0 | 0 | `sltu`
1010 | 0 | 0 | 1 | `sltiu`
1010 | 0 | 1 | 0 | `bltu`
1011 | 0 | 1 | 0 | `beq`
1100 | 0 | 1 | 0 | `bne`
1101 | 0 | 1 | 0 | `bge`
1110 | 0 | 1 | 0 | `bgeu`
1111 | 0 | 0 | 0 | `sb`
1111 | 0 | 0 | 1 | `sh`
1111 | 0 | 1 | 0 | `sw`
1111 | 0 | 1 | 1 | `lbu`
1111 | 1 | 0 | 0 | `lb`
1111 | 1 | 0 | 1 | `lh`
1111 | 1 | 1 | 0 | `lw`
1111 | 1 | 1 | 1 | `lhu`