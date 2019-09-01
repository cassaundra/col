//! Interpret a col program from a string.
//!
//! # Example
//!
//! ```
//! use std::io::{stdout, stdin};
//!
//! use col::interpreter::Interpreter;
//! use col::program::SimpleProgramState;
//!
//! let mut stdout = stdout();
//!	let mut stdin = stdin();
//!
//!	Interpreter::<SimpleProgramState>::new("\"Hello world\"Arp@", Some(&mut stdin), Some(&mut stdout))
//!		.run()
//!		.expect("An I/O error occurred");
//! ```
//!
//! It is also possible to register a callback after each step.
//!
//! ```
//! use col::interpreter::Interpreter;
//! use col::program::SimpleProgramState;
//!
//! Interpreter::<SimpleProgramState>::new("12345@", None, None)
//! 	.with_step_callback(&|stacks| {
//! 		for (index, stack) in stacks {
//! 			println!("{}: {:?}", index, stack);
//! 		}
//! 	})
//! 	.run_with_delay(20) // wait 20 ms between steps
//! 	.expect("An I/O error occurred")
//! ```

use std::cell::{Ref, RefMut};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

use crate::parser::Instruction;
use crate::program::{ProgramState, VecStack};
use std::borrow::BorrowMut;

/// How often automatic tne basic garbage collection will be called.
const GC_STEPS: u32 = 8192;

/// Callback function for after each step.
type StepCallback = dyn Fn(Vec<(u32, Ref<Vec<u32>>)>);

#[derive(Default)]
pub struct Interpreter<'a, P: ProgramState> {
	/// The source of the program
	source: Vec<&'a str>,
	/// Program input
	reader: Option<&'a mut dyn Read>,
	/// Program output
	writer: Option<&'a mut dyn Write>,
	/// User-defined step callback
	step_callback: Option<Box<&'a StepCallback>>,
	/// The memory stacks
	state: P,
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
	/// Should the remote stack be initialized?
	should_init_remote: bool,
}

impl Default for StepResponse {
	fn default() -> Self {
		StepResponse { is_alive: true, should_init_remote: false }
	}
}

impl<'a, P: ProgramState> Interpreter<'a, P> {
	/// Create a new col interpreter from a program
	pub fn new(program: &'a str, reader: Option<&'a mut dyn Read>, writer: Option<&'a mut dyn Write>) -> Self {
		let mut interpeter = Self::default();

		interpeter.load_source(program);

		interpeter.reader = reader;
		interpeter.writer = writer;

		interpeter
	}

	/// Set a callback function to be called after each program step.
	///
	/// See module documentation for example.
	pub fn with_step_callback(mut self, callback: &'a StepCallback) -> Self {
		self.step_callback = Some(Box::new(callback));
		self
	}

	/// Executes the program until terminates.
	/// This function is blocking and will return when the program has completed execution.
	pub fn run(&mut self) -> std::io::Result<()> {
		self.run_with_delay(0)
	}

	/// Executes the program until it terminates with a specified delay between each step.
	/// This function is blocking and will return when the program has completed execution
	pub fn run_with_delay(&mut self, delay_ms: u64) -> std::io::Result<()> {
		let mut gc_count = 0;

		// keep stepping until terminated
		// first group is condition, second is body for delay
		while {
			// do execution step
			let result = self.step()?;

			// do garbage collection
			if gc_count % GC_STEPS == 0 {
				self.state.discard_unused(&self.program_len(), &self.remote_column);
			}
			gc_count += 1;

			// ensure the remote stack is initialized
			if result.should_init_remote {
				self.state.init_stack(&self.remote_column);
			}

			if let Some(callback) = &self.step_callback {
				&callback(self.state.stacks());
			}

			result.is_alive
		} {
			// we don't even want to call the thread sleep if 0, because
			// it might still pause the thread for a bit (citation needed)
			if delay_ms != 0 {
				thread::sleep(Duration::from_millis(delay_ms));
			}
		}

		Ok(())
	}

	fn load_source(&mut self, program: &'a str) {
		self.source = program.lines().collect();
		self.state = P::new(self.program_len());
	}

	fn current_line(&self) -> &'a str {
		self.source.get(self.local_column as usize).unwrap_or(&"")
	}

	/// Number of program-defined columns
	fn program_len(&self) -> u32 {
		self.source.len() as u32
	}

	/// Find the matching right bracket forwards
	fn matching_forwards(&self) -> u32 {
		let iter = (self.ip + 1)..self.current_line().len() as u32;
		self.matching(&Instruction::LeftBracket, &Instruction::RightBracket, iter)
	}

	/// Find the matching left bracket backwards.
	fn matching_backwards(&self) -> u32 {
		self.matching(&Instruction::RightBracket, &Instruction::LeftBracket, (0..self.ip - 1).rev())
	}

	/// Used by `matching_backwards` and `matching_forwards`.
	fn matching<I>(&self, current: &Instruction, matching: &Instruction, iter: I) -> u32
		where I: Iterator<Item = u32> {
		let line = self.current_line();

		// depth of other bracket pairs, 0 is top-level
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
		if self.current_line().len() > 0 {
			self.ip = self.ip % self.current_line().len() as u32;
		}
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
		let mut local_stack: RefMut<VecStack> = self.state.nth(self.local_column).unwrap().borrow_mut();
		let mut remote_stack: Option<RefMut<VecStack>> = self.state.nth(self.remote_column)
			.filter(|_| self.local_column != self.remote_column) // avoid a BorrowMutError
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
				self.local_column = local_stack.pop() % self.program_len() as u32;
				self.ip = 0; // we'll begin executing here
			}
			Instruction::SetRemoteStack => {
				self.remote_column = local_stack.pop();

				// this will ensure the stack is available the next iteration
				if self.remote_column >= self.program_len() as u32 {
					step_result.should_init_remote = true;
				}
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
					// TODO could take advantage of RefCell swap
					VecStack::swap(local_stack.borrow_mut(), remote_stack);
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
			Instruction::BitwiseNand => {
				let (a, b) = local_stack.pop2();
				local_stack.push(!(a & b));
			},
			Instruction::LogicalAnd => {
				let (a, b) = local_stack.pop2();
				local_stack.push((a != 0 && b != 0) as u32);
			}
			Instruction::LogicalOr => {
				let (a, b) = local_stack.pop2();
				local_stack.push((a != 0 || b != 0) as u32);
			}
			Instruction::LogicalNot => {
				if local_stack.pop() == 0 {
					local_stack.push(1);
				} else {
					local_stack.push(0);
				}
			}
			Instruction::Random => {
				local_stack.push(rand::random());
			},
			Instruction::StringMode => {
				// realistically this will only ever *enable* string mode
				// because execute_instruction is not called while already
				// inside string mode
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
