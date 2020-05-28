# Strip assembler reference

## Registers

Register |   Description
---------|----------------
zero     | Hard-wired zero
ra       | Return address
s0-s5    | Saved registers

## Assembler Directives

Directive  | Arguments    | Description
-----------|--------------|--------------------
`.equ`     | `name value` | Constant definition
`.alias`   | `name reg`   | Register alias definition
`.zero`    | `number`     | Emit zeros
`.string`  | `string `    | Emit string
`.incbin`  | `filename`   | Emit binary file
`.byte`    |              | Emit 8-bit words
`.half`    |              | Emit 16-bit words
`.word`    |              | Emit 32-bit words

## Assembler Instructions

Instruction            | Description
-----------------------|---------------------------
`ecall rd      imm`    | Request to the execution environment
`lui   rd      imm`    | Load upper 16-bit
`la    rs1     addr`   | Load address
`lb    rd      addr`   | Load 8-bit word
`lbu   rd      addr`   | Load 8-bit word, unsigned
`lh    rd      addr`   | Load 16-bit word
`lhu   rd      addr`   | Load 16-bit word, unsigned
`lw    rd      addr`   | Load 32-bit word
`sb    rs1     addr`   | Store 8-bit word
`sh    rs1     addr`   | Store 16-bit word
`sw    rs1     addr`   | Store 32-bit word
`beq   rs  rt  offset` | Branch if = ; ra <- pc + 1
`bne   rs  rt  offset` | Branch if != ; ra <- pc + 1
`bge   rs  rt  offset` | Branch if >= ; ra <- pc + 1
`blt   rs  rt  offset` | Branch if < ; ra <- pc + 1
`bgeu  rs  rt  offset` | Branch if >=, unsigned ; ra <- pc + 1
`bltu  rs  rt  offset` | Branch if <, unsigned ; ra <- pc + 1
`add   rd  rs1 rs2`    | `rd = rs1 + rs2`
`addi  rd  rs1 imm`    | `rd = rs1 + imm`
`and   rd  rs1 rs2`    | `rd = rs1 & rs2`
`andi  rd  rs1 imm`    | `rd = rs1 & imm`
`mul   rd  rs1 rs2`    | `rd = rs1 * rs2`
`muli  rd  rs1 imm`    | `rd = rs1 * imm`
`or    rd  rs1 rs2`    | `rd = rs1 \| rs2`
`ori   rd  rs1 imm`    | `rd = rs1 \| imm`
`sll   rd  rs1 rs2`    | `rd = rs1 << rs2`
`slli  rd  rs1 imm`    | `rd = rs1 << imm`
`slt   rd  rs1 rs2`    | `rd = rs1 < rs2`
`sltiu rd  rs1 imm`    | `rd = rs1 < imm`, unsigned
`sltu  rd  rs1 rs2`    | `rd = rs1 < rs2`, unsigned
`sra   rd  rs1 rs2`    | `rd = rs1 >>> rs2`
`srl   rd  rs1 rs2`    | `rd = rs1 >> rs2`
`srli  rd  rs1 imm`    | `rd = rs1 >> imm`
`sub   rd  rs1 rs2`    | `rd = rs1 - rs2`
`xor   rd  rs1 rs2`    | `rd = rs1 ^ rs2`
`xori  rd  rs1 imm`    | `rd = rs1 ^ imm`

## Pseudo-instructions

Instruction          | Expansion           | Description
---------------------|---------------------|-----------------------
`nop`                | `addi x0 x0 0`      | No operation
`j    offset`           | `beq x0 x0 offset`  | Jump
`inc  rd`            | `addi rd rd 1`      | Increment register
`dec  rd`            | `addi rd rd -1`     | Decrement register
`li   rd imm`        | `addi rd zero imm`  | Load immediate (lower 16-bit)
`mv   rd rs`         | `addi rd rs 0`      | Copy register
`not  rd rs1`        | `xori rd rs -1`     | One’s complement
`neg  rd rs1`        | `sub rd x0 rs`      | Two’s complement
`seqz rd rs1`        | `sltiu rd rs 1`     | Set if = zero
`snez rd rs1`        | `sltu rd x0 rs`     | Set if != zero
`sltz rd rs1`        | `slt rd rs x0`      | Set if < zero
`sgtz rd rs1`        | `slt rd x0 rs`      | Set if > zero
`beqz rs1 offset`    | `beq rs x0 offset`  | Branch if = zero ; ra <- pc + 1
`bnez rs1 offset`    | `bne rs x0 offset`  | Branch if != zero ; ra <- pc + 1
`blez rs1 offset`    | `bge x0 rs offset`  | Branch if <= zero ; ra <- pc + 1
`bgez rs1 offset`    | `bge rs x0 offset`  | Branch if ≥ zero ; ra <- pc + 1
`bltz rs1 offset`    | `blt rs x0 offset`  | Branch if < zero ; ra <- pc + 1
`bgtz rs1 offset`    | `blt x0 rs offset`  | Branch if > zero ; ra <- pc + 1
`bgt  rs  rt offset` | `blt rt rs offset`  | Branch if > ; ra <- pc + 1
`ble  rs  rt offset` | `bge rt rs offset`  | Branch if <= ; ra <- pc + 1
`bgtu rs  rt offset` | `bltu rt rs offset` | Branch if >, unsigned ; ra <- pc + 1
`bleu rs  rt offset` | `bltu rt rs offset` | Branch if <=, unsigned ; ra <- pc + 1

### Instruction layout

Imm/Addr  | rs2       | rs1       | rd      | Opcode 
----------|-----------|-----------|---------|--------
`[31:16]` | `[15:13]` | `[12:10]` | `[9:7]` | `[6:0]`

### Opcode layout

Func | R | B | I | Opcode
-----| --|---|---|--------
0000 | 0 | 0 | 0 | `ADD`
0000 | 0 | 0 | 1 | `ADDI`
0000 | 1 | 0 | 0 | `LA`
0000 | 1 | 0 | 1 | `LUI`
0001 | 0 | 0 | 0 | `AND`
0001 | 0 | 0 | 1 | `ANDI`
0010 | 0 | 0 | 0 | `OR`
0010 | 0 | 0 | 1 | `ORI`
0011 | 0 | 0 | 0 | `XOR`
0011 | 0 | 0 | 1 | `XORI`
0100 | 0 | 0 | 0 | `SLL`
0100 | 0 | 0 | 1 | `SLLI`
0101 | 0 | 0 | 0 | `SRL`
0101 | 0 | 0 | 1 | `SRLI`
0110 | 0 | 0 | 0 | `SRA`
0111 | 0 | 0 | 0 | `SUB`
1000 | 0 | 0 | 0 | `MUL`
1000 | 1 | 0 | 1 | `MULI`
1000 | 0 | 0 | 1 | `ECALL`
1001 | 0 | 0 | 0 | `SLT`
1001 | 0 | 1 | 0 | `BLT`
1010 | 0 | 0 | 0 | `SLTU`
1010 | 0 | 0 | 1 | `SLTIU`
1010 | 0 | 1 | 0 | `BLTU`
1011 | 0 | 1 | 0 | `BEQ`
1100 | 0 | 1 | 0 | `BNE`
1101 | 0 | 1 | 0 | `BGE`
1110 | 0 | 1 | 0 | `BGEU`
1111 | 0 | 0 | 0 | `SB`
1111 | 0 | 0 | 1 | `SH`
1111 | 0 | 1 | 0 | `SW`
1111 | 0 | 1 | 1 | `LBU`
1111 | 1 | 0 | 0 | `LB`
1111 | 1 | 0 | 1 | `LH`
1111 | 1 | 1 | 0 | `LW`
1111 | 1 | 1 | 1 | `LHU`