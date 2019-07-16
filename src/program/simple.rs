//! TODO document me

use std::collections::HashMap;
use std::cell::RefCell;

use super::*;

#[derive(Debug, Default)]
pub struct SimpleProgramState {
	stacks: HashMap<u32, RefCell<VecStack>>,
}

impl ProgramState for SimpleProgramState {
	fn new(initial_count: u32) -> Self {
		let mut stacks = HashMap::new();

		for i in 0..initial_count {
			stacks.insert(i, RefCell::new(VecStack::default()));
		}

		SimpleProgramState { stacks }
	}

	fn nth(&self, index: u32) -> Option<&RefCell<VecStack>> {
		self.stacks.get(&index)
	}

	fn collect_garbage(&mut self, program_defined: &u32, remote_index: &u32) {
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

	fn init_stack(&mut self, index: &u32) {
		if self.stacks.contains_key(index) {
			return
		}

		self.stacks.insert(*index, RefCell::new(VecStack::default()));
	}
}