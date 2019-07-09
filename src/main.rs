#![feature(exclusive_range_pattern)]

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};

mod parser;
mod interpreter;
mod stack;

fn main() {
	let program = "\"Hello world\"Arp@";
	let mut stdout = stdout();
	let mut stdin = stdin();

	let mut interpeter = Interpreter::new(program, &mut stdin, &mut stdout);
	interpeter.run();
}
