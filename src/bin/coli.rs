//#![feature(exclusive_range_pattern)]

use std::io::{stdout, stdin};
use clap::{App, Arg, crate_version, crate_authors, crate_description, value_t};

use col::interpreter::{Interpreter};
use col::program::{AdvancedProgramState};

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

	Interpreter::<AdvancedProgramState>::new(&program, Some(&mut stdin), Some(&mut stdout))
		.run_with_delay(delay)
		.expect("An I/O error occurred");
}