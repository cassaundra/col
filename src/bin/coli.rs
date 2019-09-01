//#![feature(exclusive_range_pattern)]

use std::io::{stdout, stdin};
use clap::{App, Arg, crate_version, crate_authors, value_t};

use col::interpreter::{Interpreter};
use col::program::SimpleProgramState;
use std::error::Error;

fn main() {
	let matches = App::new("coli")
		.version(crate_version!())
		.author(crate_authors!())
		.about("col interpreter")
		.arg(Arg::with_name("file")
			.help("Source file to interpret.")
			.required(true)
			.validator(validate_path))
		.arg(Arg::with_name("step_delay")
			.help("Milliseconds to delay between steps")
			.takes_value(true)
			.long("delay")
			.required(false)
			.default_value("0"))
		.get_matches();

	let file = matches.value_of("file").unwrap();
	let program = std::fs::read_to_string(file)
		.unwrap_or_else(|e| panic!("Could not read source file: {}", e.description()));

	let delay = value_t!(matches.value_of("step_delay"), u64)
		.unwrap_or_else(|e| e.exit()); // clean exit if invalid

	let mut stdout = stdout();
	let mut stdin = stdin();

	Interpreter::<SimpleProgramState>::new(&program, Some(&mut stdin), Some(&mut stdout))
		.run_with_delay(delay)
		.expect("An I/O error occurred");
}

fn validate_path(val: String) -> Result<(), String> {
	let path = std::path::Path::new(&val);

	if !path.exists() {
		Err(String::from("The specified file could not be found"))
	} else if !path.is_file() {
		return Err(String::from("The specified path is not a file"))
	} else {
		Ok(())
	}
}