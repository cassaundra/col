#![feature(euclidean_division)]
#![feature(exclusive_range_pattern)]

use crate::parser::{Parser};

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};

mod parser;
mod interpreter;
mod stack;

fn main() {
	let program = "
		\"test\"\
		\"test2\"
	";

	{
		let mut stdout = stdout();
		let mut stdin = stdin();

		let mut interpeter = Interpreter::new(program, &mut stdin, &mut stdout);
		interpeter.run();
	}
}
