# col

`col` is an esoteric programming language inspired by classical architectural columns and the syntax of other esolangs like [Befunge](https://esolangs.org/wiki/Befunge) and [Brainfuck](https://esolangs.org/wiki/Brainfuck).

Both instruction sets and the memory stacks are interpreted as columns. Each column can perform a variety of operations on its own stack, as well as push and pop to/from another column's stack. Below is a more visual representation of this general structure. The finite source code is written at "base" of the column, while the memory stack of each column spans the shaft (length).

```
         ┌─ #4
         │
 5       │   0
 4       │   1
 3   7   │   0
 2   5   v   1
 1   3       0               1) Instructions are executed
 0   1   6   1  <- #2           from top to bottom
─── ─── ─── ─── ...          2) Bottom of memory stack                
 a   d   g   j  <- #1        3) Column currently being executed
 b   e   h   k               4) Selected remote memory stack          
 c   f   i   l

     ^
     #3
```

The source of the above program would be:

```
abc
def
ghi
jkl
```

An interpreter is currently being written in Rust.

## Theory

`col` is not designed to to be a "good" programming language. Instead, it attempts to disrupt typical programming paradigms enough that an experienced programmer may have to stop and think periodically. Here are a couple of the main points regarding its implications:

There aren't clear assurances of immutability or privacy as any column may read and/or modify another column's stack. In traditional programming, this would be dangerous, but it's an intentional design choice in `col`. The accessibility of memory is unrestricted, but there are still clearly defined associations between instructions sets and their memory.

Furthermore, "functions" (instruction sets) may have memory that persists longer than a single execution. The programmer may choose to clear the memory stack as if it's scoped, or allow it to persist through future calls, or a hybrid of both. As a result, the line between persistent and ephemeral memory is blurred.

## Specification

### Rules

- Whitespace and undefined characters are ignored by the interpreter.
- There are a finite number of columns, defined by the source. Each has a finite instruction set and a memory stack of no official max capacity (depends on implementation).
- Lines (separated by line feeds) represent columns, where the line index maps to the column index (e.g. first line is column \#0). Leading and trailing empty lines are ignored.
- When an instruction stack has completed execution, it repeats.
- Each column may switch its "remote" stack, which is by default itself (so `^`, `v`, and `s` yield no change).
- The columns are organized conceptually as a circle, wrapping at either end.
- The runtime user's input is also represented as a stack.
- If any operation cannot be performed, the value zero is used instead.
	- Dividing by zero results in zero being pushed to the local stack.
	- Attempting to pop a value from an empty stack (whether it be local, remote, or user input) results in zero being returned instead.
- The defined remote stack of a column persists between executions.
- The program terminates only upon the terminator character, `@`.

### Implementation

There are certain aspects of `col` that are left up to implementation:

- **Max stack size**
    - Default: unlimited
- **Max number of columns**
    - Default: 4294967296 (2^32)
- **Charset**
    - Default: UTF-8
- **Value type**
    - Default: unsigned 32-bit integer
    
Ideally the max value would be greater than or equal to the number of columns, so the `;` and `~` commands can be used for every column.

### Instructions

| Cmd | Description                                                                                                                      |
|:---:|----------------------------------------------------------------------------------------------------------------------------------|
| `<` | Push the index of the column on the left to the local stack.                                                                     |
| `>` | Push the index of the column on the right to the local stack.                                                                    |
| `.` | Push the index of the current column to the local stack.                                                                         |
| `;` | Pop value `a` and begin execution at the `a`th column.                                                                           |
| `~` | Pop value `a` and set the remote stack to the `a`th column's stack.                                                              |
| `^` | Pop value `a` from the *local* stack and push to the *remote* stack.                                                             |
| `v` | Pop value `a` from the *remote* stack and push to the *local* stack.                                                             |
| `\` | Swap the top two values of the local stack.                                                                                      |
| `:` | Duplicate the top value of the local stack (peek + push).                                                                        |
| `x` | Discard the top value of the local stack.                                                                                        |
| `c` | Clear the local stack.                                                                                                           |
| `s` | Swap the local and remote stacks.                                                                                                |
| `r` | Reverse the order of the local stack.                                                                                            |
|`0-9`| Push a number value to the stack (*not* the UTF-8 value of the digit).                                                           |
|`A-F`| Push a number value to the stack from hexadecimal (decimal 10-15).                                                               | 
| `[` | Skip past the matching `]` if popped value `a` is zero. If none found, then the IP will return to the start.                     |
| `]` | Skip back to after the matching `[` if popped value `a` is non-zero. If none found, then the IP will return to the start.        |
| `+` | Pop values `a` and `b` and push the result of `a` plus `b`.                                                                      |
| `-` | Pop values `a` and `b` and push the result of `b` minus `a`.                                                                     |
| `*` | Pop values `a` and `b` and push the result of `a` times `b`.                                                                     |
| `/` | Pop values `a` and `b` and push the integer result of `b` divided by `a`. If `a` is zero, then zero will be pushed to the stack. |
| `%` | Pop values `a` and `b` and push the remainder of the integer division of `b` divided by `a`.                                     |
| `=` | Pop values `a` and `b`, and push one if `a` equals `b`, and zero otherwise.                                                      |
|`` ` ``| Pop values `a` and `b` and push one if `b` is greater than `a`, and zero otherwise.       |                                    
| `&` | Pop values `a` and `b` and push one if they're both non-zero, and push zero otherwise. Not a bitwise AND.                 .      |
| `\|` | Pop values `a` and `b` and push one if at least one is non-zero, and push zero if they are both zero. Not a bitwise OR.         |
| `!` | Invert the top value of the local stack. If it's zero, push one, and if it's non-zero, push zero.                                |
| `?` | TODO random                                                                                                                      |
| `"` | Toggle string mode and push UTF-8 values until next `"`.                                                                         |
| `_` | Pop UTF-8 char from user input and push to the stack. If no more are available, push zero.                                       |
| `$` | Pop `a` and print its UTF-8 value.                                                                                               |
| `#` | Pop `a` and print its numeric value.                                                                                             |
| `p` | Print all values in stack (from top to bottom) as UTF-8 characters.                                                              |
| `@` | Terminate the entire program.                                                                                                    |

## Examples

**Hello world:**

```
"Hello, world!"rp@
```