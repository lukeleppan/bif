# bif - The Rusty Brainfuck Interpreter

bif is a cli that lets you run brainfuck programs blazingly fast.

## Installation
Run
```BASH
cargo install bif
```

## Usage

```
Usage: bif [OPTIONS] <source file>

Arguments:
  <source file>  Brainfuck Source File

Options:
  -i, --input <INPUT>        Input to the program [default: ]
  -c, --capacity <TAPE_CAP>  Tape Capacity [default: 30000]
  -s, --step <STEP>          Number of millis between steps [default: 0]
  -h, --help                 Print help
  -V, --version              Print version
```
