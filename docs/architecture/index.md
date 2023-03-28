# CPU Architecture

## Features
- 16bit data & address width
- Software stack
- Von Neumann architecture for loading instructions

## Inspirations
- x86\_64 (registers)
- jdah/jdh-8 (spec, some instructions)

## Registers
The cpu will have 16 user registers, r0-9, ra-rd and stack registers. (rsp and rbp - stack and base pointer) In the 
future I might consider adding rsa and rdi if I add interrupts.

In addition to user accessible registers, there will be a [flags register](#flags-register) and a
[status register](#status-register), which are not directly accessible.

### Flags register

| Mask | Flag     |
|------|----------|
| 0x1  | Carry    |
| 0x2  | Zero     |
| 0x4  | Sign     |
| 0x8  | Overflow |

### Status register

| Value | Status |
| 0x0   | Off    |
| 0x1   | Power  |
| 0x2   | Halt   |

## Instructions
- halt                       -> status = Halt(0x2)
- noop                       -> skip cycle
- mov reg, reg/imm16         -> reg = reg/imm16 (move value into register)
- ldm reg, [reg/imm16]       -> reg = mem\[reg/imm16] (load from memory)
- sim [reg/imm16], reg/imm16 -> mem\[reg/imm16] = reg/imm16 (store into memory)
- push reg/imm16             -> rsp++; mem[rsp] = reg/imm16 (push value into stack)
- pop reg                    -> reg = mem[rsp]; rsp-- (pop value off stack)
- jmp reg/imm16              -> pc = reg/imm16 (Jump to address, jc, jnc, jz, jnz, js, jns, jo and jno)
- set(f)                     -> flags = Mask[f] | 1 << Mask\[f] (set flag, f = flag)
- clr(f)                     -> flags = Mask[f] & ~Mask\[f] (clear flag, f = flag)

### ALU Instructions
- not reg                    -> reg = ~reg (bitwise not)
- or reg, reg/imm16          -> reg = reg | reg/imm16 (bitwise or)
- xor reg, reg/imm16         -> reg = reg ^ reg/imm16 (bitwise xor)
- and reg, reg/imm16         -> reg = reg & reg/imm16 (bitwise and)
- add reg, reg/imm16         -> reg = reg + reg/imm16 (add)
- addc reg, reg/imm16        -> reg = reg + reg/imm16 + c (add with carry)
- sub reg, reg/imm16         -> reg = reg - reg/imm16 (subtract operation)
- subb reg, reg/imm16        -> reg = reg - reg/imm16 - c (subtract with carry)
- inc reg                    -> reg = reg + 1 (increment)
- dec reg                    -> reg = reg - 1 (decrement)
- shl reg, reg4/imm4         -> reg = reg << reg4/imm4 (shift left)
- shr reg, reg4/imm4         -> reg = reg >> reg4/imm4 (shift right)
- rotl reg, reg4/imm4        -> reg = reg rotate left by reg4/imm4 (rotate left)
- rotr reg, reg4/imm4        -> reg = reg rotate left by reg4/imm4 (rotate right)
- mul reg8, reg8/imm8        -> reg = reg8 * reg8/imm8 (multiply)

imm4/8/16 are either values immediately after the instruction, or in case of imm4 can be inside the instruction.
