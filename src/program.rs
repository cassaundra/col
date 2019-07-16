//! Program state.

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct ProgramState {
	stacks: HashMap<u32, RefCell<VecStack>>,
}

impl ProgramState {
	/// Create a new `ProgramState` with an initial number of empty `VecStack`s
	pub fn new(initial_count: u32) -> Self {
		let mut stacks = HashMap::new();

		for i in 0..initial_count {
			stacks.insert(i, RefCell::new(VecStack::default()));
		}

		ProgramState { stacks }
	}

	/// Get the nth program stack as a RefCell to be borrowed.
	///
	/// This does NOT insert a new stack if one does not exist.
	/// See `insert_stack`.
	pub fn nth(&self, index: u32) -> Option<&RefCell<VecStack>> {
		self.stacks.get(&index)
	}

	/// Invoke the garbage collector
	/// TODO improve, then explain how it actually works
	pub fn collect_garbage(&mut self, program_defined: &u32, remote_index: &u32) {
		// insert a stack at the remote column if one does not exist already
		self.stacks.entry(*remote_index)
			.or_insert(RefCell::new(VecStack::default()));

		// remove empty stacks that aren't being used by the program or the remote stack.
		// we could also use a queuing system that delays the removal, but I don't think allocating
		// individual stacks is that expensive so...
		self.stacks.retain(|index, stack| {
			if index >= program_defined && index != remote_index {
				return !stack.borrow().is_empty(); // remove only if empty
			}

			return true; // if it's part of the program defined stacks, keep it!
		});
	}

	// TODO document
	pub fn insert_stack(&mut self, index: &u32) {
		self.stacks.insert(*index, RefCell::new(VecStack::default()));
	}
}

#[derive(Clone, Default, Debug)]
pub struct VecStack {
	stack: Vec<u32>,
}

impl Stack for VecStack {
	fn values(&self) -> &Vec<u32> {
		&self.stack
	}

	fn push(&mut self, value: u32) {
		self.stack.push(value);
	}

	fn pop(&mut self) -> u32 {
		self.stack.pop().unwrap_or_default()
	}

	fn pop2(&mut self) -> (u32, u32) {
		(self.pop(), self.pop())
	}

	fn peek(&self) -> u32 {
		*self.stack.last().unwrap_or(&0u32)
	}

	fn clear(&mut self) {
		self.stack.clear()
	}

	fn reverse(&mut self) {
		self.stack.reverse()
	}

	fn set_all(&mut self, values: Vec<u32>) {
		self.stack = values;
	}

	fn is_empty(&self) -> bool {
		self.stack.is_empty()
	}
}

// TODO do we need other implementations?
pub trait Stack {
	fn values(&self) -> &Vec<u32>;
	fn push(&mut self, value: u32);
	fn pop(&mut self) -> u32;
	fn pop2(&mut self) -> (u32, u32);
	fn peek(&self) -> u32;
	fn clear(&mut self);
	fn reverse(&mut self);
	fn set_all(&mut self, values: Vec<u32>);
	fn is_empty(&self) -> bool;
}