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
mod tests {
	use super::*;

	use std::io::{Read, BufWriter};

	fn get_output_with_input<R: Read>(source: &str, reader: &mut R) -> String {
		let mut output = Vec::new();
		Interpreter::new(source, Some(reader), Some(&mut output))
			.run()
			.unwrap();
		return String::from_utf8_lossy(output.as_slice()).into_owned();
	}

	fn get_output(source: &str) -> String {
		let mut output = Vec::new();
		Interpreter::new(source, None, Some(&mut output))
			.run()
			.unwrap();
		return String::from_utf8_lossy(output.as_slice()).into_owned()
	}

	#[test]
	fn test_hello_world() {
		assert_eq!(get_output("\"Hello world!\"rp@"), "Hello world!");
	}

	#[test]
	fn test_math() {
		// misc stuff
		assert_eq!(get_output("0123456789ABCDEFr#[#]@"), "0123456789101112131415");
		assert_eq!(get_output("092++#@"), "11");
		assert_eq!(get_output("F1+F1+*#@"), "256");
		assert_eq!(get_output("15/4#@"), "4");

		// overflow/underflow
		assert_eq!(get_output("1-#@"), "4294967295");
		assert_eq!(get_output("1-  5+#@"), "4");
	}

	// TODO add more tests
}