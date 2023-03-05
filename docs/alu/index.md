# ALU

As mentioned in the introduction, the alu uses 16bit inputs and outputs and all operations except multiply use all 16
input bits. Multiply uses only the lower 8bits of its inputs, as it would overflow anyway. (and HOUT output would have
to be added)

The ALU has 4 inputs (A, B, SEL and CIN) and 6 outputs, (X/OUT, COUT, OVERFLOW, SIGN, ZERO, CMP) let's go over all of
them.

## Inputs

### A and B
These are the 16bit values the ALU is working with, A is the primary operand and B serves as an argument, which is
ignored in operations like NOT, INC or DEC.

### SEL
SEL (short for select) is a 4bit input, that tells the ALU which operation should be executed. See the [list of
operations](operations).

### CIN
Last but not least, CIN is a 1bit input used in ADD, SUB, INC, DEC and CMP operations. CIN stands for carry-in and
behaves as the third value for the adder (can be used to pass the carry when adding 32bit numbers)

## Outputs

### X
X is the output of the ALU, it's 16bit wide. In case of NOOP the output is equal to the A input.

### COUT
Just like with the input CIN, this is the carry-out. Opposed to CIN, COUT is also returned by shift operations, in which
it returns the last bit that was omitted from X (the output).

### Flags (OVERFLOW, SIGN, ZERO and CMP)
These outputs are each 1bit wide and serve as flags for the cpu. They all are pretty much self-descriptive. ZERO is 1
only if all bits of X/OUT are 0 and SIGN is only the first bit of X/OUT, which in case of signed integers means the
output number is negative. Overflow is 1 when there is an overflow and CMP is the only special output, that signifies
subtract without carry operation was executed, so LT,EQ and GT flags can be written.

## Negative numbers
I haven't put much thought into toggling signed and unsigned numbers and working with them, in most operations it
doesn't matter, as they only do logical operations with the input bits, but in case of addition, subtraction and
comparison, the ALU expects all numbers to be signed integers. 
