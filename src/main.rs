//! *col* is an esoteric programming language inspired by classical architectural columns and the
//! syntax of other esolangs like [Befunge](https://esolangs.org/wiki/Befunge) and
//! [Brainfuck](https://esolangs.org/wiki/Brainfuck).
//!
//! Learn more in the [project repository](https://github.com/cassaundra/col).
//!
//! To interpret col in your own program, see the [interpreter](interpreter)
//! documentation.

#![feature(exclusive_range_pattern)]

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};
use clap::{App, Arg, crate_version, crate_authors, crate_description, value_t};

pub mod parser;
pub mod interpreter;
pub mod program;

fn main() {
	let matches = App::new("col")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(Arg::with_name("file")
			.help("Source file to interpret.")
			.required(true))
		.arg(Arg::with_name("step_delay")
			.help("Milliseconds to delay between steps")
			.takes_value(true)
			.long("delay")
			.required(false))
		.get_matches();

	let file = matches.value_of("file").unwrap();
	let program = std::fs::read_to_string(file).expect("Could not read file");

	let delay = value_t!(matches.value_of("step_delay"), u64).unwrap_or(0);

	let mut stdout = stdout();
	let mut stdin = stdin();

	Interpreter::new(&program, Some(&mut stdin), Some(&mut stdout))
		.run(delay)
		.expect("An I/O error occurred");
}

#[cfg(test)]
mod test;