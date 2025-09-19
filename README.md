# BRAINFUCK JIT COMPILER

## Usage

This application currently only supports x86_64 processors running on linux (why does windows make printing/getting input require a linker). If I continue to work on this I will try to support x86, and arm architectures.

Command to run:

```bash
cargo run [filepath]
```

## Simple optimiser

Brainfuck doesn't natively support `mov`, `add`, `mul` instructions, instead loops are used to mimic the behavior. As a simple optimisation, during the parsing of the file, I replace loops which perform this behavior with these instructions.

Future optimisations could involve recursively going up the tree and searching for more patterns which can be replaced with a single assembly instruction.

## Why I built the project

When building my Gameboy and Gameboy Advanced emulators, I would come across people who were implementing JIT compiler's into their projects to improve performance. I wanted to give it a shot in a standalone project before trying to implement it in any future emulators.

I decided to hardcode the machine code instead of using a library such as `cranelift` as I wanted to gain appreciation for the low level stuff without immediately abstracting it away. Future projects will most likely use either `cranelift` or `LLVM`, as the JIT compiler won't be the sole focus of those projects.

## Resources I used to learn

[Defuse Assembler to get the machine code](https://defuse.ca/online-x86-assembler.htm#disassembly)

[Writing a JIT compiler in golang](https://medium.com/kokster/writing-a-jit-compiler-in-golang-964b61295f)

[Assembly Reference](https://www.felixcloutier.com/x86)

[Getting Started with x64 Assembly](https://laihoconsulting.com/blog/2021-08-getting-started-with-x64-assembly/)
