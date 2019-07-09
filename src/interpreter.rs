use crate::parser::Instruction;
use std::io::{Read, Write, Error};
use crate::stack::{Stack, VecStack};

#[derive(Default)]
pub struct Interpreter<'a> {
	/// The source of the program
	source: Vec<&'a str>,
	reader: Option<&'a mut dyn Read>,
	writer: Option<&'a mut dyn Write>,
	stacks: Vec<VecStack>,
	/// The index of the current local column
	local_column: u32,
	/// The index of the current remote column
	remote_column: u32,
	is_string_mode: bool,
	skip_next: bool,
	/// Instruction pointer
	ip: u32,
}

#[derive(PartialEq)]
enum InterpreterState {
	Executing,
	Terminated,
}

impl<'a> Interpreter<'a> {
	/// Create a new col interpeter from a program
	pub fn new<R: Read, W: Write>(program: &'a str, reader: &'a mut R, writer: &'a mut W) -> Self {
		let mut interpeter = Self::default();

		interpeter.load_source(program);

		interpeter.reader = Some(reader);
		interpeter.writer = Some(writer);

		interpeter
	}

	fn load_source(&mut self, program: &'a str) -> &mut Self {
		self.source = program.lines().collect();

		let num_columns = program.lines().count();

		self.stacks = vec![VecStack::default(); num_columns];

		self
	}

	/// Executes the program until it terminates.
	pub fn run(&mut self) -> Result<(), Error> {
		self.ip = 0;

		while {
			self.step() == InterpreterState::Executing
		} {}

		Ok(())
	}

	fn increment_ip(&mut self, line_len: u32) {
		self.ip += 1;
		self.ip = self.ip % line_len;
	}

	fn step(&mut self) -> InterpreterState {
		let line = self.source[self.local_column as usize];

		if self.is_string_mode {
			let c = line.chars().nth(self.ip as usize).unwrap();

			if Instruction::from_char(&c) == Some(Instruction::StringMode) {
				self.is_string_mode = false;
			} else {
				self.stacks[self.local_column as usize].push(c as u32);
			}

			self.increment_ip(line.len() as u32);
		} else {
			loop {
				let instr = line.chars().nth(self.ip as usize).and_then(|c| Instruction::from_char(&c));

				self.increment_ip(line.len() as u32);

				if let Some(instr) = instr {
					return self.execute_instruction(instr).expect("An IO error occurred");
				}
			}
		}

		return InterpreterState::Executing;
	}

	fn execute_instruction(&mut self, instruction: Instruction) -> std::io::Result<InterpreterState> {
		let num_columns = self.stacks.len();

		let stacks_mut = self.stacks.iter_mut();

		let mut local_stack = None;
		let mut remote_stack = None;

		for (i, stack) in stacks_mut.enumerate() {
			let i = i as u32;

			if i == self.local_column {
				local_stack = Some(stack);
			} else if i == self.remote_column {
				remote_stack = Some(stack);
			}
		}

		if self.skip_next {
			self.skip_next = false;
			return Ok(InterpreterState::Executing);
		}

		let local_stack = local_stack.unwrap();

		match instruction {
			Instruction::PushLeftIndex => {
				let pos = self.local_column.wrapping_sub(1);

				local_stack.push(pos);
			}
			Instruction::PushRightIndex => {
				// no need for signed since we'd be wrapping above
				let pos = self.local_column.wrapping_add(1);

				local_stack.push(pos);
			}
			Instruction::PushCurrentIndex => {
				local_stack.push(self.local_column); // g
			}
			Instruction::SetLocalColumn => {
				self.local_column = local_stack.pop() % num_columns as u32;
				self.ip = 0; // we'll begin executing here
			}
			Instruction::SetRemoteStack => {
				self.remote_column = local_stack.pop() % num_columns as u32;
			},
			Instruction::MoveToRemote => {
				if self.local_column != self.remote_column {
					remote_stack.unwrap().push(local_stack.pop());
				}
			},
			Instruction::MoveToLocal => {
				if self.local_column != self.remote_column {
					local_stack.push(remote_stack.unwrap().pop());
				}
			},
			Instruction::Discard => {
				local_stack.pop();
			},
			Instruction::SwapTop => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a);
				local_stack.push(b);
			},
			Instruction::DuplicateTop => {
				local_stack.push(local_stack.peek())
			}
			Instruction::Clear => {
				local_stack.clear();
			},
			Instruction::SwapStacks => {
				if self.local_column != self.remote_column {
					let remote_stack = remote_stack.unwrap();

					let local_values = local_stack.values().clone();
					let remote_values = remote_stack.values().clone();

					local_stack.set_all(remote_values);
					remote_stack.set_all(local_values);
				}
			},
			Instruction::Reverse => {
				local_stack.reverse();
			},
			Instruction::Value(value) => {
				local_stack.push(value);
			},
			Instruction::If => {
				if local_stack.pop() == 0 {
					self.skip_next = true;
				}
			},
			Instruction::Add => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a + b);
			},
			Instruction::Subtract => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b - a);
			},
			Instruction::Multiply => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a * b);
			},
			Instruction::Divide => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b / a);
			},
			Instruction::Modulo => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b % a); // unsigned, so negative behavior is unimportant
			},
			Instruction::Equals => {
				let (a, b) = local_stack.pop2();
				local_stack.push((b == a) as u32);
			},
			Instruction::GreaterThan => {
				let (a, b) = local_stack.pop2();
				local_stack.push((b > a) as u32);
			},
			Instruction::Invert => {
				if local_stack.pop() == 0 {
					local_stack.push(1);
				} else {
					local_stack.push(0);
				}
			},
			Instruction::StringMode => {
				self.is_string_mode = !self.is_string_mode;
			},
			Instruction::Input => {
				if let Some(reader) = &mut self.reader {
					let mut buffer = [0; 1];
					reader.read(&mut buffer)?;

					local_stack.push(buffer[0] as u32);
				}
			},
			Instruction::Skip => {
				self.skip_next = true;
			},
			Instruction::PrintChar => {
				if let Some(writer) = &mut self.writer {
					let c = std::char::from_u32(local_stack.pop()).unwrap();
					write!(writer, "{}", c)?;
				}
			},
			Instruction::PrintNumber => {
				if let Some(writer) = &mut self.writer {
					let c = local_stack.pop();
					write!(writer, "{}", c)?;
				}
			},
			Instruction::PrintAll => {
				if let Some(writer) = &mut self.writer {
					let s = local_stack.values().iter().rev().map(|val| {
						std::char::from_u32(*val).unwrap()
					}).collect::<String>();

					write!(writer, "{}", s)?;
					writer.flush()?;

					local_stack.clear();
				}
			},
			Instruction::Terminate => {
				return Ok(InterpreterState::Terminated);
			},
		};

		return Ok(InterpreterState::Executing);
	}
}