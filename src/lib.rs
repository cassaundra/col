//! *col* is an esoteric programming language inspired by classical architectural columns and the
//! syntax of other esolangs like [Befunge](https://esolangs.org/wiki/Befunge) and
//! [Brainfuck](https://esolangs.org/wiki/Brainfuck).
//!
//! Learn more in the [project repository](https://github.com/cassaundra/col).
//!
//! To interpret col in your own program, see the [interpreter](interpreter)
//! documentation.

pub mod parser;
pub mod interpreter;
pub mod program;

#[cfg(test)]
mod test;