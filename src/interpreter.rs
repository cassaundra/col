use crate::parser::{Program, Instruction};
use std::io::{Read, Write, Error};

pub struct ProgramState {
	stacks: Vec<Vec<u8>>,
	output: String
}

#[derive(Default)]
pub struct Interpreter<'a> {
	program: Program,
	reader: Option<&'a mut Read>,
	writer: Option<&'a mut Write>,
	stacks: Vec<Vec<u8>>,
	/// Instruction pointer
	local_column: u8,
	remote_column: u8
}

impl<'a> Interpreter<'a> {
	/// Create a new col interpeter from a program
	pub fn new<R: Read, W: Write>(program: Program, reader: &'a mut R, writer: &'a mut W) -> Self {
		let mut interpeter = Self::default();
		interpeter.load(program);
		interpeter.read_from(reader);
		interpeter.write_to(writer);

		interpeter
	}

	pub fn load(&mut self, program: Program) -> &mut Self {
		self.program = program;
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
//		for instr in self.program.asl {
//			self.execute_instruction(instr);
//		}
		Ok(())
	}

	fn step(&mut self) {

	}

	// TODO ret result?
	fn execute_instruction(&mut self, instruction: Instruction) {
		match instruction {
			Instruction::LeftIndex => {
				let num_columns = self.program.asl.len() as isize;
				let pos = self.local_column as isize - 1; // conv. to signed for negative shift
				let pos = new_pos.rem_euclid(num_columns) as u8; // use math mod for wrapping

				self.local_stack().push(pos);
			},
			Instruction::RightIndex => {
				// no need for signed since we'd be wrapping above
				let num_columns = self.program.asl.len();
				let pos = (self.local_column + 1) % num_columns;

				self.local_stack().push(pos);
			},
			Instruction::CurrentIndex => {
				self.local_stack().push(self.local_column);
			},
			Instruction::SetLocalColumn => {
				let pos = self.pop_local();
				self.local_column = pos;
				// TODO begin execution there
			}
			Instruction::SetRemoteStack => {
				self.remote_column = self.pop_local();
			},
			Instruction::MoveToRemote => {
				self.remote_stack().push(self.pop_local());
			},
			Instruction::MoveToLocal => {
				self.local_stack().push(self.pop_remote());
			},
			Instruction::Discard => {
				self.local_stack().pop();
			},
			Instruction::SwapTop => {
				// TODO is there a better way to write this
				let len = self.local_stack().len();
				self.local_stack().swap(len, len - 1);
			}
			_ => {}
		}
	}

	fn local_stack(&self) -> Vec<u8> {
		self.stacks[self.local_column]
	}

	fn remote_stack(&self) -> Vec<u8> {
		self.stacks[self.remote_column]
	}

	fn pop_local(&self) -> u8 {
		self.local_stack().pop().unwrap_or_default()
	}

	fn pop_remote(&self) -> u8 {
		self.remote_stack().pop().unwrap_or_default()
	}
}