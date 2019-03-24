# col

`col` is an esoteric programming language inspired by the syntax of Befunge. It's written in columns, each with their own memory stack, in addition to a global memory stack.

Once the specification is stable, I'll begin writing an interpreter in Rust.

## Specification

TODO

Points to make:
- Use UTF-8.
- Each column has a value.
- In addition to a global stack, each column has a stack. All stacks are reset each iteration.
- Columns are read left-to-right.
- Instructions are executed top-to-bottom.
- The number of columns is finite and determined by the number of columns defined in the source.
- Space characters are interpreted as an empty command.
- The grid is conceptually a circle and wraps at either end. When using the `,` command, the column number is also wrapped through modular arithmetic. For example, in a length-5 grid, the 8th column is the same as the 3rd and the negative -2nd.

### Instructions

| Cmd | Description                                                                                                                      |
|:---:|----------------------------------------------------------------------------------------------------------------------------------|
| `>` | Swap the values of this column and the column on the right.                                                                      |
| `<` | Swap the values of this column and the column on the left.                                                                       |
| `,` | Pop value `a` from the stack and swap the values of this column with the `a`th column.                                           |
| `^` | Pop value `a` from the stack and set this column's value to it.                                                                  |
| `~` | Push the column's current value to the stack (value remains unchanged).                                                          |
|`\|` | Switch to local stack.                                                                                                           |
| `#` | Switch to global stack.                                                                                                          |
| `!` | Pop a value from the stack and do nothing.                                                                                       |
| `\` | Swap the top two values on the stack.                                                                                            |
| `:` | Duplicate the top value of the stack.                                                                                            |
|`0-9`| Push a number value to the stack.                                                                                                |
| `{` | Begin a section with label popped from stack. The interpreter will ignore instructions until closing `}` with same "depth."      |
| `}` | End section. If this character is reached during execution, the column ends.                                                     |
| `;` | Pop a value `a` and go to the corresponding labeled section.                                                                     |
| `?` | Pop top of stack `a` and `b`. If `b` is not zero, then go to labeled section `b`.                                                |
| `+` | Pop values `a` and `b` and push the result of their addition.                                                                    |
| `-` | Pop values `a` and `b` and push the result of `b` minus `a`.                                                                     |
| `*` | Pop values `a` and `b` and push the result of their multiplication.                                                              |
| `/` | Pop values `a` and `b` and push the integer result of `b` divided by `a`. If `a` is zero, then zero will be pushed to the stack. |
| `%` | Pop values `a` and `b` and push the remainder of the integer division of `b` divided by `a`.                                     |
| `=` | Pop `a` and `b`, and push one if `a` equals `b`, and zero otherwise.                                                             |
|`` ` ``| Pop values `a` and `b` and push one if `b` is greater than `a`, and zero otherwise.                                            |
| `_` | Pop UTF-8 char from user input and push to the stack. If no more are available, push zero.                                       |
| `"` | Toggle string mode and push UTF-8 values until next `"`.                                                                         |
| `@` | Terminate the entire program.                                                                                                    |

## Examples

This program prints the user input, up to 10 characters.

```
" >>>>>>>>>
A
"
{
<
^
}
_
:
0
=
"
A
"
?
```