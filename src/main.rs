#![feature(exclusive_range_pattern)]

use crate::interpreter::{Interpreter};
use std::io::{stdout, stdin};
use clap::{App, Arg, crate_version, crate_authors, crate_description, value_t};

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
		.arg(Arg::with_name("step_delay")
			.help("How many milliseconds to delay between steps")
			.required(false))
		.get_matches();

	let file = matches.value_of("file").unwrap();
	let program = std::fs::read_to_string(file).expect("Could not read file");

	let delay = value_t!(matches.value_of("step_delay"), u64).unwrap_or(0);

	let mut stdout = stdout();
	let mut stdin = stdin();

	let mut interpreter = Interpreter::new(&program, Some(&mut stdin), Some(&mut stdout));
	interpreter.delay_ms = delay;

	interpreter.run().expect("An I/O error occurred");
}

#[cfg(test)]
mod test;