# col

`col` is an esoteric programming language inspired by classical architectural columns and the syntax of other esolangs like [Befunge](https://esolangs.org/wiki/Befunge) and [Brainfuck](https://esolangs.org/wiki/Brainfuck).

Both the instructions and the memory stacks are written in columns, each with its own defined instruction set and runtime stack. Each column can pop and push to another column's stack. Below is a more visual representation of the this concept. The finite source code is written at base of the column, while the memory stack of each column spans the shaft (length).

```
 5   5   5   5
 4   4   4   4
 3   3   3   3
 2   2   2   2
 1   1   1   1
 0   0   0   0
--- --- --- --- ...
 a   d   g   j
 b   e   h   k
 c   f   i   l
```

Because any column can modify another column's stack, there are really no assurances of immutability or privacy. In "traditional" programming, this would be terrifying, but it's an intentional design choice here. By not restricting the accessibility of memory *but* still defining clear associations between instruction sets and their memory, interesting solutions emerge. Furthermore, "functions" (instruction sets) may have memory that persists longer than a single call.

`col` is not designed to to be a "good" programming language. It's nothing more than a fun experiment that encourages the programmer to break out of traditional habits and invent alternative ways of solving problems. I hope you enjoy playing around with it :)

Once the specification is stable, I'll begin writing an interpreter in Rust.

## Specification

TODO

Misc. points to organize:
- Use UTF-8.
- Values are unsigned 8-bit integers.
- Columns are represented horizontally, line-by-line, in the source.
- Each column has a stack.
- The program begin in the 0th column.
- Instructions are executed top-to-bottom.
- Each column uses the nth register for local memory.
- The columns are organized conceptually a circle and wraps at either end. When using the `,` command, the column number is also wrapped through modular arithmetic. For example, in a length-5 grid, the 8th column is the same as the 3rd and the -2nd.
- If a stack is empty and a pop operation is performed, zero is returned.

### Instructions

| Cmd | Description                                                                                                                      |
|:---:|----------------------------------------------------------------------------------------------------------------------------------|
| `>` | Begin execution at the start of the column on the right. Wrap if need be.                                                        |
| `<` | Begin execution at the start of the column on the left. Wrap if need be.                                                         |
| `;` | Pop a value `a` and begin execution at the `a`th column.                                                                         |
| `^` | Pop value `a` from the *local* stack and push to the *remote* stack.                                                             |
| `v` | Pop value `a` from the *remote* stack and push to the *local* stack.                                                             |
| `~` | Pop value `a` from the local stack and set the remote stack of index `a`.                                                        |
| `!` | Pop a value from the local stack and do nothing.                                                                                 |
| `\` | Swap the top two values on the local stack.                                                                                      |
| `:` | Duplicate the top value of the local stack (peek + push).                                                                        |
|`0-9`| Push a number value to the stack (base 10).                                                                                      |
| `?` | Pop `a` and only run next instruction if `a` not zero.                                                                           |
| `+` | Pop values `a` and `b` and push the result of `a` plus `b`.                                                                      |
| `-` | Pop values `a` and `b` and push the result of `b` minus `a`.                                                                     |
| `*` | Pop values `a` and `b` and push the result of `a` times `b`.                                                                     |
| `/` | Pop values `a` and `b` and push the integer result of `b` divided by `a`. If `a` is zero, then zero will be pushed to the stack. |
| `%` | Pop values `a` and `b` and push the remainder of the integer division of `b` divided by `a`.                                     |
| `=` | Pop `a` and `b`, and push one if `a` equals `b`, and zero otherwise.                                                             |
|`` ` ``| Pop values `a` and `b` and push one if `b` is greater than `a`, and zero otherwise.                                            |
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
0~v:?@$
```

**Fibonacci:**

```
1:$>
0~v+::^#
```
