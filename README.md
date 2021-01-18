# intcode-rust

This is an Intcode interpreter for [Advent of Code 2019](https://www.adventofcode.com/2019) written in Rust.

The Intcode programming language was created in the [Advent of Code 2019](https://www.adventofcode.com/2019). For details on the language, see the [Intcode Language Specifications](#intcode-language-specifications) below.

## Installation

Clone this repository and compile it via

```bash
cargo build --release
```

## Execution

Execute the Intcode interpreter using the command below:

```bash
./intcode program.ic
```

## Intcode Language Specifications

The Intcode programming language was created in the [Advent of Code 2019](adventofcode.com/2019).
The characteristic of the language is that it only consists of integers (`int64`s to be exact). An instruction consists of an operation code, or [opcode](#opcodes) for short, and arguments. The number of arguments depends on the opcode.

### Opcodes

An opcode (operation code) is a 2-digit number indicating which instruction should be performed. If no [parameter modes](#parameter-modes) are specified, the leading 0 can also be omitted.

| Opcode | Params | Name                 | Description                                                  |
| ------ | ------ | -------------------- | ------------------------------------------------------------ |
| 01     | 3      | Addition             | arg[2] = arg[0] + arg[1]                                     |
| 02     | 3      | Multiplication       | arg[2] = arg[0] * arg[1]                                     |
| 03     | 1      | Input                | arg[0] = input                                               |
| 04     | 1      | Output               | output = arg[0]                                              |
| 05     | 2      | Jump non-zero        | If arg[0] is ≠ 0, sets the instruction pointer to arg[1]     |
| 06     | 2      | Jump zero            | If arg[0] == 0, sets the instruction pointer to arg[1]       |
| 07     | 3      | Less Than            | If arg[0] < arg[1], sets arg[2] = 1. If not less, sets it to 0 |
| 08     | 3      | Equals               | If arg[0] == arg[1], sets arg[2] = 1. If not equal, sets it to 0 |
| 09     | 1      | Add to relative base | relative base register += arg[0]                             |

### Parameter Modes

As you can guess from the name: The parameter mode sets the mode for each parameter. This means that a parameter following an [opcode](#opcodes) may be a reference to a memory address, or may be the value itself with which the operation is to be performed. For each parameter belonging to an opcode, the mode can be different. The modes are appended to the 2-digit [opcode](#opcodes) on the left and are in reversed order to the parameters. Why in reverse 🤷? Good question. I didn't make it up.

Because the integers are stored in an array, the first memory address is `0` and negative memory addresses are invalid. A memory address beyond the program length is valid. Thus memory can be occupied dynamically at runtime of the program by enlarging the array accordingly. A possibility to release this memory during runtime does not exist according to the specifications currently.

The relative base register is initiated with the value `0` at program start and can be changed by [opcode](#opcodes) `09`.

| Mode | Name           | Description                                                  |
| ---- | -------------- | ------------------------------------------------------------ |
| 0    | Position Mode  | The parameter is the address of the value.                   |
| 1    | Immediate Mode | The parameter is the value itself (Not used for writing).    |
| 2    | Relative Mode  | The parameter is added to the relative base register, which results in the memory address of the value. |

Below is an example showing how the parameter modes work:

```bash
ABCDE
 1002

DE - two-digit opcode:		02 == opcode 02
 C - mode of 1st parameter:	 0 == position mode
 B - mode of 2nd parameter:	 1 == immediate mode
 A - mode of 3rd parameter:	 0 == position mode,
                             omitted due to being a leading zero
```
> From [adventofcode.com/2019/day/5](https://adventofcode.com/2019/day/5)
