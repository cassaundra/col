#![feature(exclusive_range_pattern)]

use crate::parser::{Program};

mod parser;

fn main() {
	let program = ">>^cv\"abc\"";
	let program = Program::parse(program);
	println!("{:?}", program);
}
