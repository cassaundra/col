//! Program state.

use std::cell::{RefCell, Ref};

mod simple;
mod advanced;

pub use simple::SimpleProgramState;
pub use advanced::AdvancedProgramState;

pub trait ProgramState: Default {
	/// Create a new `ProgramState` with an initial number of empty `VecStack`s
	fn new(initial_count: u32) -> Self;

	/// Get the nth program stack as a RefCell to be borrowed.
	///
	/// This does NOT insert a new stack if one does not exist.
	/// See `insert_stack`.
	fn nth(&self, index: u32) -> Option<&RefCell<VecStack>>;

	/// Invoke basic garbage collection. The implementation will vary.
	fn discard_unused(&mut self, program_defined: &u32, remote_index: &u32);

	/// Insert a stack, especially for outside of the program defined range.
	/// If one already exists at the index, then nothing should happen.
	fn init_stack(&mut self, index: &u32);

	/// Immutable view of of the program memory.
	fn stacks(&self) -> Vec<(u32, Ref<Vec<u32>>)>;
}

#[derive(Clone, Default, Debug)]
pub struct VecStack {
	stack: Vec<u32>,
}

// this should probably be documented but it's laughably self-documenting so...
impl VecStack {
	pub fn values(&self) -> &Vec<u32> {
		&self.stack
	}

	pub fn push(&mut self, value: u32) {
		self.stack.push(value);
	}

	pub fn pop(&mut self) -> u32 {
		self.stack.pop().unwrap_or_default()
	}

	pub fn pop2(&mut self) -> (u32, u32) {
		(self.pop(), self.pop())
	}

	pub fn peek(&self) -> u32 {
		*self.stack.last().unwrap_or(&0)
	}

	pub fn clear(&mut self) {
		self.stack.clear()
	}

	pub fn reverse(&mut self) {
		self.stack.reverse()
	}

	pub fn set_all(&mut self, values: Vec<u32>) {
		self.stack = values;
	}

	pub fn is_empty(&self) -> bool {
		self.stack.is_empty()
	}
}