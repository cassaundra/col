# col

`col` is an esoteric programming language inspired by classical architectural columns and the syntax of other esolangs like [Befunge](https://esolangs.org/wiki/Befunge) and [Brainfuck](https://esolangs.org/wiki/Brainfuck).

Both instruction sets and the memory stacks are written in columns. Each column can perform a variety of operations on its own stack, as well as pop and push to another column's stack. Below is a more visual representation of this general structure. The finite source code is written at "base" of the column, while the memory stack of each column spans the shaft (length).

```
 5   5   5   5
 4   4   4   4
 3   3   3   3
 2   2   2   2
 1   1   1   1
 0   0   0   0  <- bottom of stack
--- --- --- --- ...
 a   d   g   j  <- first instruction
 b   e   h   k
 c   f   i   l
```

Because any column may modify another column's stack, there are really no assurances of immutability or privacy. In "traditional" programming, this would be terrifying, but it's an intentional design choice here. By not restricting the accessibility of memory *but* still defining clear associations between instruction sets and their memory, interesting solutions emerge. Furthermore, "functions" (instruction sets) may have memory that persists longer than a single call.

`col` is not designed to to be a "good" programming language. It's nothing more than a fun experiment that encourages the programmer to break out of traditional habits and invent alternative ways of solving problems. I hope you enjoy playing around with it :)

Once the specification is stable, I'll begin writing an interpreter in Rust.

## Specification

- `col` sources are UTF-8 plain text.
- There are a finite number of columns, defined by the source. Each has a finite instruction set and a memory stack of no official max capacity (depends on implementation).
- Lines (separated by line feeds) represent columns, where the line index maps to the column index (e.g. first line is column \#0).
- Values are unsigned 8-bit integers. If a program accepts user input, it's interpreted as UTF-8 and translated into an unsigned 8-bit integer.
- Each column may switch its "remote" stack, which is by default itself (so `^` and `v` yield no change).
- The columns are organized conceptually as a circle, wrapping at either end.
- The runtime user's input is also represented as a stack.
- If any operation cannot be performed, the value zero is used.
	- Dividing by zero results in zero being pushed to the local stack.
	- Attempting to pop a value from an empty stack (whether it be local, remote, or user input) results in zero being pushed to the local stack.

### Instructions

| Cmd | Description                                                                                                                      |
|:---:|----------------------------------------------------------------------------------------------------------------------------------|
| `<` | Begin executing the instruction set of the column on the left.                                                                   |
| `>` | Begin executing the instruction set of the column on the right.                                                                  |
| `;` | Pop value `a` and begin execution at the `a`th column.                                                                           |
| `~` | Pop value `a` and set the remote stack to the `a`th column's stack.                                                              |
| `^` | Pop value `a` from the *local* stack and push to the *remote* stack.                                                             |
| `v` | Pop value `a` from the *remote* stack and push to the *local* stack.                                                             |
| `~` | Pop value `a` from the local stack and set the remote stack of index `a`.                                                        |
| `!` | Pop a value from the local stack and do nothing.                                                                                 |
| `\` | Swap the top two values on the local stack.                                                                                      |
| `:` | Duplicate the top value of the local stack (peek + push).                                                                        |
|`0-9`| Push a number value to the stack (*not* the UTF-8 value of the digit).                                                           |
| `?` | Pop `a` and only run the next instruction if `a` is not zero.                                                                    |
| `+` | Pop values `a` and `b` and push the result of `a` plus `b`.                                                                      |
| `-` | Pop values `a` and `b` and push the result of `b` minus `a`.                                                                     |
| `*` | Pop values `a` and `b` and push the result of `a` times `b`.                                                                     |
| `/` | Pop values `a` and `b` and push the integer result of `b` divided by `a`. If `a` is zero, then zero will be pushed to the stack. |
| `%` | Pop values `a` and `b` and push the remainder of the integer division of `b` divided by `a`.                                     |
| `=` | Pop `a` and `b`, and push one if `a` equals `b`, and zero otherwise.                                                             |
|`` ` ``| Pop values `a` and `b` and push one if `b` is greater than `a`, and zero otherwise.                                            |
| `&` | Invert the top value of the local stack. If it's zero, push one, and if it's non-zero, push zero.                                |
| `"` | Toggle string mode and push UTF-8 values until next `"`.                                                                         |
| `_` | Pop UTF-8 char from user input and push to the stack. If no more are available, push zero.                                       |
|`\|` | Skip next instruction.                                                                                                           |
| `$` | Pop `a` and print its UTF-8 value.                                                                                               |
| `#` | Pop `a` and print its numeric value.                                                                                             |
| `@` | Terminate the entire program.                                                                                                    |

## Examples

**Hello world:**

```
"!dlrow, olleH">
0~v:&?@$
```

**Fibonacci:**

```
1:#>
0~v+::^#
```
