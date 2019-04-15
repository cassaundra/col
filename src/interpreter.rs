use crate::parser::{Parser, Instruction};
use std::io::{Read, Write, Error};
use crate::stack::{Stack, VecStack};

#[derive(Default)]
pub struct Interpreter<'a> {
	/// The source of the program
	source: &'a str,
	reader: Option<&'a mut Read>,
	writer: Option<&'a mut Write>,
	stacks: &'a [VecStack],
	local_column: u8,
	remote_column: u8,
	is_string_mode: bool,
	/// Instruction pointer
	ip: usize,
}

impl<'a> Interpreter<'a> {
	/// Create a new col interpeter from a program
	pub fn new<R: Read, W: Write>(program: &'a str, reader: &'a mut R, writer: &'a mut W) -> Self {
		let mut interpeter = Self::default();

		interpeter.load_source(program);

		interpeter.read_from(reader);
		interpeter.write_to(writer);

		interpeter
	}

	fn load_source(&mut self, program: &'a str) -> &mut Self {
		self.program = program;

		let num_columns = program.lines().len();
		self.stacks = &[VecStack::default(), num_columns];

		self
	}

	/// Assign a reader for input as defined by the col spec.
	pub fn read_from<R: Read>(&mut self, reader: &'a mut R) -> &mut Self {
		self.reader = Some(reader);
		self
	}

	/// Assign a writer for output as defined by the col spec.
	pub fn write_to<W: Write>(&mut self, writer: &'a mut W) -> &mut Self {
		self.writer = Some(writer);
		self
	}

	/// Executes the program until it terminates.
	pub fn run(&mut self) -> Result<(), Error> {
		self.ip = 0;
		Ok(())
	}

	fn step(&mut self) {

	}

	// TODO ret result?
	fn execute_instruction(&mut self, instruction: Instruction) {
		// TODO
		let local_stack = &mut self.stacks[self.local_column as usize];
		let remote_stack = &(self.stacks[self.remote_column as usize]);

		match instruction {
			Instruction::LeftIndex => {
				let num_columns = self.program.columns.len() as isize;
				let pos = self.local_column as isize - 1; // conv. to signed for negative shift
				let pos = pos.rem_euclid(num_columns) as u8; // use math mod for wrapping

				local_stack.push(pos);
			},
			Instruction::RightIndex => {
				// no need for signed since we'd be wrapping above
				let num_columns = self.program.columns.len() as u8;
				let pos = (self.local_column + 1) % num_columns;

				local_stack.push(pos);
			},
			Instruction::CurrentIndex => {
				local_stack.push(self.local_column);
			},
			Instruction::SetLocalColumn => {
				let pos = local_stack().pop_safe();
				self.local_column = pos;
				// TODO begin execution there
			}
			Instruction::SetRemoteStack => {
				self.remote_column = local_stack().pop_safe();
			},
			Instruction::MoveToRemote => {
				remote_stack().push(local_stack().pop_safe());
			},
			Instruction::MoveToLocal => {
				local_stack().push(remote_stack().pop_safe());
			},
			Instruction::Discard => {
				local_stack().pop();
			},
			Instruction::SwapTop => {
				// TODO is there a better way to write this
				let len = local_stack().len();
				local_stack().swap(len, len - 1);
			},
			Instruction::Duplicate => {

			},
			Instruction::Clear => {
				local_stack().clear();
			},
			Instruction::SwapStacks => {

			},
			Instruction::Reverse => {
				local_stack().reverse();
			},
			Instruction::Value(value) => {
				local_stack().push(value);
			},
			Instruction::If => {
				// TODO need program state
			},
			Instruction::Add => {
				local_stack().push(local_stack().pop_safe() + local_stack().pop_safe());
			},
			Instruction::Subtract => {
				let a = local_stack().pop_safe();
				let b = local_stack().pop_safe();
				local_stack().push(b - a);
			},
			Instruction::Multiply => {
				local_stack().push(local_stack().pop_safe() * local_stack().pop_safe());
			},
			Instruction::Divide => {
				let a = local_stack().pop_safe();
				let b = local_stack().pop_safe();
				local_stack().push(b / a);
			},
			Instruction::Modulo => {
				let a = local_stack().pop_safe();
				let b = local_stack().pop_safe();
				local_stack().push(b % a); // TODO euclidean mod or?
			},
			Instruction::Equals => {
				// b == a
				local_stack().push((local_stack().pop_safe() == local_stack().pop_safe()) as u8);
			},
			Instruction::GreaterThan => {
				// b > a
				local_stack().push((local_stack().pop_safe() < local_stack().pop_safe()) as u8);
			},
			Instruction::Invert => {

			},
			Instruction::StringMode => {

			},
			Instruction::Input => {

			},
			Instruction::Skip => {

			},
			Instruction::PrintChar => {

			},
			Instruction::PrintNumber => {

			},
			Instruction::PrintAll => {

			},
			Instruction::Terminate => {

			},
		}
	}
}