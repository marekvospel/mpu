# MPU
<sup>Marek Vospěl</sup>

Welcome to the documentation of mpu. (Mark's processing unit) The goal of this project is to learn more about how
computers (and processors in particular) work. This documentation serves the purpose of planning the architecture design
and for documentation of all different instructions.

## Architecture
Let's get started with the most basic information about the architecture. Both the CPU and ALU will be 16bit, the ram
is going to have 16bit address bus and it's data will be 16bits wide. I'll use the Von Neumann's architecture for
loading instructions, so the instructions will be inside the RAM.

I also plan to implement software stack, (values from the end of the RAM will be used) so two registers will be reserved
to serve as base pointer (rbp) and stack pointer. (rsp) Implementing stack isn't a priority though, so if there isn't
enough time, I'll just skip it and add it later.

More about the architecture will be located in [architecture/](architecture/).

## ALU
ALU is the part of the CPU, that does the arithmetical and logical computation. It's inputs, outputs and all different
operations are explained in [alu/](alu/).
