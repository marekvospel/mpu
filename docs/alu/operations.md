# ALU Operations
| SEL  | Name | Description  | link           |
|------|------|--------------|----------------|
| 0000 | NOT  | NOT A        | [#not](#not)   |
| 0001 | OR   | A OR B       | [#or](#or)     |
| 0010 | XOR  | A XOR B      | [#xor](#xor)   |
| 0011 | AND  | A AND B      | [#and](#and)   |
| 0100 | ADD  | A ADD B, CIN | [#add](#add)   |
| 0101 | SUB  | A SUB B, CIN | [#sub](#sub)   |
| 0110 | INC  | A ADD 1, CIN | [#inc](#inc)   |
| 0111 | DEC  | A SUB 1, CIN | [#dec](#dec)   |
| 1000 | SHL  | A SHL BY B   | [#shl](#shl)   |
| 1001 | SHR  | A SHR BY B   | [#shr](#shr)   |
| 1010 | ROTL | A ROTL BY B  | [#rotl](#rotl) |
| 1011 | ROTR | A ROTR BY B  | [#rotr](#rotr) |
| 1100 | MUL  | A MUL B      | [#mul](#mul)   |
| 1101 | CMP  | A SUB B      | [#cmp](#cmp)   |
| 1110 | NOOP | A            | [#noop](#noop) |
| 1111 | NOOP | A            | [#noop](#noop) |

> The examples are simplified to 4 bit inputs & outputs

## NOT
This does the bitwise NOT (`~A`) operation on A and returns the result in X/OUT. B and CIN is ignored. 

## OR
This does the bitwise OR (`A | B`) operation on A and B and returns the result in X/OUT. CIN is ignored.

## XOR
This does the bitwise XOR (`A ^ B`) operation on A and B and returns the result in X/OUT. CIN is ignored.

## AND
This does the bitwise AND (`A & B`) operation on A and B and returns the result in X/OUT. CIN is ignored.

## ADD
This operation adds A and B using the RCA, in the future the RCA will be replaced with CLA. CIN is taken as an input for
the adder circuit, so it can be used to increase the result by 1. Its purpose is to allow chaining ADD operations and
allow carrying 1 between double words.

## SUB
To save on transistors, SUB is just an ADD operation with B inverted (NOT B) and carry turned on. Because of that the
COUT for this operation doesn't make sense, so multiple SUB operations cannot be changed to subtract bigger numbers than
double word. For parity with the ADD operation, if CIN is 1, the carry for the add circuit is turned off, so it
effectively means subtract 1.

## INC
This is an operation, that adds constant 1 to A. B is ignored, but CIN isn't, so you can add 2 instead of one by
enabling CIN.

## DEC
This is the same as INC, but for subtracting. B is once again ignored and by enabling CIN, you can subtract 2.

## SHL
This ALU uses a barrel shifter, so shifting by multiple bits doesn't have to be done in multiple operations. The output
is A shifted by the 4 LSBs of B. (as shifting by more would have the same result) The last bit that was removed because
of the shift is returned as COUT.

## SHR
The same as SHL, but it shifts right.

## ROTL
The same as SHL, but instead of throwing out the bits, they are added as LSBs.

## ROTR
The same as ROTR, but it shifts left.

## MUL
This ALU has only 8bit multiplier, so only lower 8 bits of A and B are used for this operation. Thanks to that, no HOUT
is necessary, as the highest output is 16bits. CIN is ignored.

## CMP
This is an operation, that could be replaced in the future, as it does the same thing as SUB, without CIN. The idea of
this operation is to compare A and B and be sure CIN is 0, so we can write the comparison flags.

## NOOP
There are no other operations I wanted to implement, so the last two SEL values act as no-operation, returning A and
ignoring everything else. This might be changed in the future, as it can be implemented in the CPU and I might want to
add some more operations sometime.
