# Strip assembler reference

## Registers

Register  |   Description  
----------|----------------
x0 / zero | Hard-wired zero
x1 - x7   | Saved registers

## Assembler Directives

Directive  | Arguments    | Description
-----------|--------------|--------------------
`.equ`     | `name value` | Constant definition
`.zero`    | `number`     | Emit zeros
`.string`  | `string `    | Emit string
`.incbin`  | `filename`   | Emit binary file
`.byte`    |              | Emit 8-bit words
`.half`    |              | Emit 16-bit words
`.word`    |              | Emit 32-bit words

## Assembler Instructions

Instruction           | Description
----------------------|---------------------------
`lui rd imm`          | Load upper 16-bit
`lb rd offset`        | Load 8-bit word
`lbu rd offset`       | Load 8-bit word, unsigned
`lh rd offset`        | Load 16-bit word
`lhu rd offset`       | Load 16-bit word, unsigned
`lw rd offset`        | Load 32-bit word
`sb rs1 offset`       | Store 8-bit word
`sh rs1 offset`       | Store 16-bit word
`sw rs1 offset`       | Store 32-bit word
`beq rs1 rs2 offset`  | Branch if =
`bne rs1 rs2 offset`  | Branch if !=
`bge rs1 rs2 offset`  | Branch if >=
`blt rs1 rs2 offset`  | Branch if <
`bgeu rs1 rs2 offset` | Branch if >=, unsigned
`bltu rs1 rs2 offset` | Branch if <, unsigned
`add rd rs1 rs2`      | `rd = rs1 + rs2`
`and rd rs1 rs2`      | `rd = rs1 & rs2`
`mul rd rs1 rs2`      | `rd = rs1 * rs2`
`or rd rs1 rs2`       | `rd = rs1 \| rs2`
`sub rd rs1 rs2`      | `rd = rs1 - rs2`
`xor rd rs1 rs2`      | `rd = rs1 ^ rs2`
`sll rd rs1 rs2`      | `rd = rs1 << rs2`
`srl rd rs1 rs2`      | `rd = rs1 >> rs2`
`sra rd rs1 rs2`      | `rd = rs1 >>> rs2`
`slt rd rs1 rs2`      | `rd = rs1 < rs2`
`sltu rd rs1 rs2`     | `rd = rs1 < rs2`, unsigned
`sltiu rd rs1 imm`    | `rd = rs1 ^ imm`, unsigned
`addi rd rs1 imm`     | `rd = rs1 + imm`
`ori rd rs1 imm`      | `rd = rs1 \| imm`
`andi rd rs1 imm`     | `rd = rs1 & imm`
`xori rd rs1 imm`     | `rd = rs1 ^ imm`
`slli rd rs1 imm`     | `rd = rs1 << imm`
`srli rd rs1 imm`     | `rd = rs1 >> imm`

## Pseudo-instructions

Instruction         | Expansion          | Description
--------------------|--------------------|-----------------------
`nop`               | `addi x0 x0 0`     | No operation
`j offset`          | `beq x0 x0 offset` | Jump
`inc rd`            | `addi rd rd 1`     | Increment register
`dec rd`            | `addi rd rd -1`    | Decrement register
`li rd imm`         | `addi rd rd imm`   | Load lower 16-bit
`mv rd rs`          | `addi rd rs 0`     | Copy register
`bgt rs rt offset`  | `blt rt rs offset` | Branch if >
`ble rs rt offset`  | `bge rt rs offset` | Branch if <=
`bgtu rs rt offset` | `bltu rt rs offset`| Branch if >, unsigned
`bleu rs rt offset` | `bltu rt rs offset`| Branch if <=, unsigned
`beqz rs1 offset`   | `beq rs x0 offset` | Branch if = zero
`bnez rs1 offset`   | `bne rs x0 offset` | Branch if != zero
`blez rs1 offset`   | `bge x0 rs offset` | Branch if <= zero
`bgez rs1 offset`   | `bge rs x0 offset` | Branch if ≥ zero
`bltz rs1 offset`   | `blt rs x0 offset` | Branch if < zero
`bgtz rs1 offset`   | `blt x0 rs offset` | Branch if > zero
`not rd rs1`        | `xori rd rs -1`    | One’s complement
`seqz rd rs1`       | `sltiu rd rs 1`    | Set if = zero
`snez rd rs1`       | `sltu rd x0 rs`    | Set if != zero
`sltz rd rs1`       | `slt rd rs x0`     | Set if < zero
`sgtz rd rs1`       | `slt rd x0 rs`     | Set if > zero
`neg rd rs1`        | `sub rd x0 rs`     | Two’s complement