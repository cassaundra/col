use crate::parser::Instruction;
use std::io::{Read, Write};
use crate::stack::{Stack, VecStack};

#[derive(Default)]
pub struct Interpreter<'a> {
	/// The source of the program
	source: Vec<&'a str>,
	/// Program input
	reader: Option<&'a mut dyn Read>,
	/// Program output
	writer: Option<&'a mut dyn Write>,
	/// The memory stacks
	stacks: Vec<VecStack>,
	/// The index of the current local column
	local_column: u32,
	/// The index of the current remote column
	remote_column: u32,
	/// Whether or not the interpreter is in string mode
	is_string_mode: bool,
	/// Whether or not the next instruction will be skipped
	skip_next: bool,
	/// Instruction pointer
	ip: u32,
}

#[derive(PartialEq)]
enum InterpreterState {
	Alive,
	Terminated,
}

impl<'a> Interpreter<'a> {
	/// Create a new col interpreter from a program
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
	pub fn run(&mut self) -> std::io::Result<()> {
		self.ip = 0;

		while {
			// keep stepping until terminated
			self.step()? != InterpreterState::Terminated
		} {}

		Ok(())
	}

	fn current_line(&self) -> String {
		self.source[self.local_column as usize].to_owned()
	}

	/// Find the matching right bracket forwards
	fn matching_forwards(&self) -> u32 {
		let iter = (self.ip + 1)..self.current_line().len() as u32;
		self.matching(&Instruction::LeftBracket, &Instruction::RightBracket, iter)
	}

	/// Find the matching left bracket backwards
	fn matching_backwards(&self) -> u32 {
		self.matching(&Instruction::RightBracket, &Instruction::LeftBracket, (0..self.ip - 1).rev())
	}

	/// Used by matching_backwards and matching_forwards
	fn matching<I>(&self, current: &Instruction, matching: &Instruction, iter: I) -> u32
		where I: Iterator<Item = u32> {
		let line = self.current_line();

		let mut depth = 0;

		for i in iter {
			let instr = line.chars().nth(i as usize)
				.and_then(|c| Instruction::from_char(&c));

			if instr == Some(*current) {
				depth += 1;
			} else if instr == Some(*matching) {
				if depth == 0 {
					return i;
				} else {
					depth -= 1;
				}
			}
		}

		return 0;
	}

	/// Safely increment the instruction pointer by one
	fn increment_ip(&mut self) {
		self.ip += 1;
		self.ip = self.ip % self.current_line().len() as u32;
	}

	/// Perform one program step
	fn step(&mut self) -> std::io::Result<InterpreterState> {
		let line = self.current_line();

		// handle special skip mode
		if self.skip_next {
			self.skip_next = false;
			return Ok(InterpreterState::Alive);
		}

		// string mode stuff
		let state = if self.is_string_mode {
			let c = line.chars().nth(self.ip as usize);
			let instr = c.and_then(|c| Instruction::from_char(&c));

			// prioritize exiting strng mode
			if instr == Some(Instruction::StringMode) {
				self.is_string_mode = false;
			} else if let Some(c) = c {
				// push a raw value to the stack
				self.stacks[self.local_column as usize].push(c as u32); // TODO is this a safe cast?
			}

			InterpreterState::Alive // cool no matter what (:
		} else {
			let mut instr = None;

			// find the next valid instruction
			while instr == None {
				instr = line.chars().nth(self.ip as usize).and_then(|c| Instruction::from_char(&c));
			}

			// execute and pass on result
			self.execute_instruction(instr.unwrap())?
		};

		// increment *safely* for next iteration
		self.increment_ip();

		return Ok(state);
	}

	fn execute_instruction(&mut self, instruction: Instruction) -> std::io::Result<InterpreterState> {
		let num_columns = self.stacks.len();

		// TODO scary iter_mut stuff to appease the borrow checker

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

		let local_stack = local_stack.unwrap();

		match instruction {
			Instruction::PushLeftIndex => {
				let pos = self.local_column.wrapping_sub(1);

				local_stack.push(pos);
			},
			Instruction::PushRightIndex => {
				// no need for signed since we'd be wrapping above
				let pos = self.local_column.wrapping_add(1);

				local_stack.push(pos);
			},
			Instruction::PushCurrentIndex => {
				local_stack.push(self.local_column); // g
			},
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
			Instruction::LeftBracket => {
				if local_stack.peek() == 0 {
					self.ip = self.matching_forwards();
				}
			},
			Instruction::RightBracket => {
				if local_stack.peek() != 0 {
					self.ip = self.matching_backwards();
				}
			},
			Instruction::Add => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a.wrapping_add(b));
			},
			Instruction::Subtract => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_sub(a));
			},
			Instruction::Multiply => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a.wrapping_mul(b));
			},
			Instruction::Divide => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_div(a));
			},
			Instruction::Modulo => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b % a); // unsigned, so no worries about wrapping
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
					write!(writer, "{}", local_stack.pop())?;
				}
			},
			Instruction::PrintAll => {
				if let Some(writer) = &mut self.writer {
					let s = local_stack.values().iter().rev().filter_map(|val| {
						std::char::from_u32(*val)
					}).collect::<String>();

					write!(writer, "{}", s)?;

					local_stack.clear();
				}
			},
			Instruction::Terminate => {
				return Ok(InterpreterState::Terminated);
			},
		};

		return Ok(InterpreterState::Alive);
	}
}