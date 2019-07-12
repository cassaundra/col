use std::cell::RefMut;
use std::io::{Write, Read};

use crate::parser::Instruction;
use crate::program::{Stack, VecStack};
use crate::program::ProgramState;

/// How often automatic garbage collection will occur.
/// The counter should be reset after manual memory cleanups.
const GC_STEPS: u32 = 512;

#[derive(Default)]
pub struct Interpreter<'a> {
	/// The source of the program
	source: Vec<&'a str>,
	/// Program input
	reader: Option<&'a mut dyn Read>,
	/// Program output
	writer: Option<&'a mut dyn Write>,
	/// The memory stacks
	state: ProgramState,
	/// The index of the current local column
	local_column: u32,
	/// The index of the current remote column
	remote_column: u32,
	/// Whether or not the interpreter is in string mode
	is_string_mode: bool,
	/// Instruction pointer
	ip: u32,
}

/// Result from an execution step
struct StepResponse {
	/// Is the program still alive after this step?
	is_alive: bool,
	/// Should
	should_adjust_mem: bool,
}

impl Default for StepResponse {
	fn default() -> Self {
		StepResponse { is_alive: true, should_adjust_mem: false }
	}
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
		self
	}

	/// Executes the program until it terminates.
	pub fn run(&mut self) -> std::io::Result<()> {
		self.ip = 0;

		let mut gc_count = 0;

		// keep stepping until terminated
		while {
			let result = self.step()?;

			// do garbage collection/manual mem adjustment
			if gc_count >= GC_STEPS - 1 || result.should_adjust_mem {
				gc_count = 0;
				self.state.adjust_memory(&(self.source.len() as u32), &self.remote_column);
			} else {
				gc_count += 1;
			}

			result.is_alive
		} {}

		Ok(())
	}

	fn current_line(&self) -> &'a str {
		self.source[self.local_column as usize]
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
	fn step(&mut self) -> std::io::Result<StepResponse> {
		let mut step_result = StepResponse::default();

		let line = self.current_line();

		// string mode stuff
		if self.is_string_mode {
			let c = line.chars().nth(self.ip as usize);
			let instr = c.and_then(|c| Instruction::from_char(&c));

			// prioritize exiting string mode
			if instr == Some(Instruction::StringMode) {
				self.is_string_mode = false;
			} else if let Some(c) = c {
				// push a raw value to the stack
				self.state.nth(self.local_column).unwrap().borrow_mut().push(c as u32);
			}

			self.increment_ip();
		} else {
			let mut instr = None;

			// TODO trim out invalid characters?

			// find the next valid instruction
			while instr == None {
				instr = line.chars().nth(self.ip as usize).and_then(|c| Instruction::from_char(&c));
				self.increment_ip();
			}

			// execute and pass on result
			self.execute_instruction(instr.unwrap(), &mut step_result)?;
		};

		return Ok(step_result);
	}

	fn execute_instruction(&mut self, instruction: Instruction, step_result: &mut StepResponse) -> std::io::Result<()> {
		// TODO this really doesn't need to be done each iteration
		// *but* it does simplify execution flow, so we'll keep it here until it becomes a problem
		let mut local_stack = self.state.nth(self.local_column).unwrap().borrow_mut();
		let mut remote_stack: Option<RefMut<VecStack>> = self.state.nth(self.local_column)
			.and_then(|v| {
				Some(v.borrow_mut())
			});

		match instruction {
			Instruction::PushLeftIndex => {
				let pos = self.local_column.wrapping_sub(1);
				local_stack.push(pos);
			},
			Instruction::PushRightIndex => {
				let pos = self.local_column.wrapping_add(1);
				local_stack.push(pos);
			},
			Instruction::PushCurrentIndex => {
				local_stack.push(self.local_column);
			},
			Instruction::SetLocalColumn => {
				self.local_column = local_stack.pop() % self.source.len() as u32;
				self.ip = 0; // we'll begin executing here
			}
			Instruction::SetRemoteStack => {
				self.remote_column = local_stack.pop();

				// this will ensure the stack is available the next iteration
				step_result.should_adjust_mem = true;
			},
			Instruction::MoveToRemote => {
				if let Some(remote_stack) = &mut remote_stack { // redundant otherwise
					remote_stack.push(local_stack.pop());
				}
			},
			Instruction::MoveToLocal => {
				if let Some(remote_stack) = &mut remote_stack { // redundant otherwise
					local_stack.push(remote_stack.pop());
				}
			},
			Instruction::SwapTop => {
				let (a, b) = local_stack.pop2();
				local_stack.push(a);
				local_stack.push(b);
			},
			Instruction::DuplicateTop => {
				let value = local_stack.peek();
				local_stack.push(value);
			},
			Instruction::Discard => {
				local_stack.pop();
			},
			Instruction::Clear => {
				local_stack.clear();
			},
			Instruction::SwapStacks => {
				if let Some(remote_stack) = &mut remote_stack {
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
				if local_stack.pop() == 0 {
					self.ip = self.matching_forwards();
				}
			},
			Instruction::RightBracket => {
				if local_stack.pop() != 0 {
					self.ip = self.matching_backwards();
				}
			},
			Instruction::Add => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_add(a));
			},
			Instruction::Subtract => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_sub(a));
			},
			Instruction::Multiply => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_mul(a));
			},
			Instruction::Divide => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_div(a));
			},
			Instruction::Modulo => {
				let (a, b) = local_stack.pop2();
				local_stack.push(b.wrapping_rem(a)); // actually equivalent to b % a because unsigned
			},
			Instruction::Equals => {
				let (a, b) = local_stack.pop2();
				local_stack.push((b == a) as u32);
			},
			Instruction::GreaterThan => {
				let (a, b) = local_stack.pop2();
				local_stack.push((b > a) as u32);
			},
			Instruction::And => {
				let (a, b) = local_stack.pop2();
				local_stack.push((a != 0 && b != 0) as u32);
			},
			Instruction::Or => {
				let (a, b) = local_stack.pop2();
				local_stack.push((a != 0 || b != 0) as u32);
			},
			Instruction::Invert => {
				if local_stack.pop() == 0 {
					local_stack.push(1);
				} else {
					local_stack.push(0);
				}
			},
			Instruction::Random => {
				local_stack.push(rand::random());
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
				step_result.is_alive = false;
			},
		};

		Ok(())
	}
}