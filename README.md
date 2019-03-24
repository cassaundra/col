# col

`col` is an esoteric programming language inspired by the syntax of Befunge. It's written in columns, each with their own memory stack, in addition to a global memory stack.

Once the specification is stable, I'll begin writing an interpreter in Rust.

## Specification

TODO

### Instructions

| Cmd | Description           |
|:---:| -------------|
|  >  | Shift the current value to the column on the right
|  <  | Shift the current value to the column on the left
|  .  | Switch to local stack.
|  #  | Switch to global stack.
|  ^  | Pop a value `a` from the stack and set this column's value to it
|  ~  | Push current value to the stack (value remains unchanged)
|  !  | Pop a value from the stack and do nothing
|  \  | Swap the top two values on the stack.
|  :  | Duplicate the top value of the stack.
| 0-9 | Push a number value to the stack
| A-Z | Push an ASCII A-Z to the stack for use in labels. See `"` command for string input.
|  {  | Begin a section with label popped from stack. The interpreter will ignore instructions until closing `}` with same "depth"
|  }  | End section. If this character is reached during execution, the column ends.
|  ;  | Pop a value `a` and go to the corresponding labeled section.
|  \|  | Peek top of stack `a`. If it's not zero, then pop `a` and `b` from stack and go to labeled section `b`.
|  +  | Pop values `a` and `b` and push the result of their addition.
|  -  | Pop values `a` and `b` and push the result of `b` minus `a`.
|  *  | Pop values `a` and `b` and push the result of their multiplication.
|  /  | Pop values `a` and `b` and push the integer result of `b` divided by `a`. If `a` is zero, then zero will be pushed to the stack.
|  %  | Pop values `a` and `b` and push the remainder of the integer division of `b` divided by `a`.
|  =  | Pop `a` and `b`, and push one if `a` equals `b`, and zero otherwise.
|  `  | Pop values `a` and `b` and push one if `b` is greater than `a`, and zero otherwise.
|  _  | Pop ASCII char from user input and push to the stack. If no more are available, push zero.
|  "  | Toggle string mode and push ASCII values until next `"`.
|  @  | Terminate the entire program.

## Examples

This program prints the user input, up to 10 characters.

```
A>>>>>>>>>
{
>
^
}
_
:
0
=
A
|
```
