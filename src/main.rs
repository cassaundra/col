#![feature(euclidean_division)]
#![feature(exclusive_range_pattern)]

use crate::parser::{Program};

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};

mod parser;
mod interpreter;

fn main() {
	let program = "\"Hello, world!\"rp";
	let program = Program::parse(program);
	println!("{:?}", program);

	{
		let mut stdout = stdout();
		let mut stdin = stdin();

		let mut interpeter = Interpreter::new(program, &mut stdin, &mut stdout);
		interpeter.run();
	}
}
