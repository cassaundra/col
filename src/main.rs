#![feature(exclusive_range_pattern)]

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};
use clap::{App, Arg, crate_version, crate_authors, crate_description};

mod parser;
mod interpreter;
mod program;

fn main() {
	let matches = App::new("col")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(Arg::with_name("file")
			.help("The source file to interpret.")
			.required(true))
		.get_matches();

	let file = matches.value_of("file").unwrap();
	let program = std::fs::read_to_string(file).expect("Could not read file");

	let mut stdout = stdout();
	let mut stdin = stdin();

	Interpreter::new(&program, Some(&mut stdin), Some(&mut stdout))
		.run()
		.expect("An unexpected I/O error occurred.");
}

#[cfg(test)]
mod test;