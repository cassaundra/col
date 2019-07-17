use std::io::Read;

use crate::interpreter::Interpreter;
use crate::program::AdvancedProgramState;

fn get_output_with_input<R: Read>(source: &str, reader: &mut R) -> String {
	let mut output = Vec::new();
	Interpreter::<AdvancedProgramState>::new(source, Some(reader), Some(&mut output))
		.run()
		.unwrap();
	return String::from_utf8_lossy(output.as_slice()).into_owned();
}

fn get_output(source: &str) -> String {
	let mut output = Vec::new();
	Interpreter::<AdvancedProgramState>::new(source, None, Some(&mut output))
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
	assert_eq!(get_output("0123456789ABCDEFr#[#]@"), "0123456789101112131415"); // literals
	assert_eq!(get_output("092++#@"), "11"); // addition
	assert_eq!(get_output("F1+F1+*#@"), "256"); // addition, multiplication
	assert_eq!(get_output("15/4#@"), "4"); // division
	assert_eq!(get_output("FF-# FE-# @"), "01"); // subtraction

	// overflow/underflow
	assert_eq!(get_output("1-#@"), "4294967295");
	assert_eq!(get_output("1-  5+#@"), "4");
}

#[test]
fn test_logic() {
	assert_eq!(get_output("55=# FA=# @"), "10"); // equality
	assert_eq!(get_output("55`# 54`# 45`# @"), "010"); // greater than
	assert_eq!(get_output("5!# 0!# FF+!# 1!# @"), "0100"); // invert
}

#[test]
fn test_io() {

}

#[test]
fn test_flow() {

}

#[test]
fn test_stacks() {

}

#[test]
fn test_loops() {

}

#[test]
fn test_string_mode() {

}

#[test]
fn test_weird_chars() {

}

// TODO add more tests