//! TODO document me

use std::collections::HashMap;
use std::cell::RefCell;

use super::*;

#[derive(Debug, Default)]
pub struct AdvancedProgramState {
	program_stacks: Vec<RefCell<VecStack>>,
	extended_stacks: HashMap<u32, RefCell<VecStack>>,
}

impl ProgramState for AdvancedProgramState {
	fn new(initial_count: u32) -> Self {
		AdvancedProgramState {
			program_stacks: vec![RefCell::new(VecStack::default()); initial_count as usize],
			extended_stacks: HashMap::new(),
		}
	}

	fn nth(&self, index: u32) -> Option<&RefCell<VecStack>> {
		if index < self.program_stacks.len() as u32 {
			return self.program_stacks.get(index as usize);
		} else {
			return self.extended_stacks.get(&index);
		}
	}

	fn discard_unused(&mut self, _program_defined: &u32, remote_index: &u32) {
		self.extended_stacks.retain(|index, stack| {
			// retain only if it's the reserved remote stack or contains values
			index == remote_index || !stack.borrow().is_empty()
		});
	}

	fn init_stack(&mut self, index: &u32) {
		// only search extended stacks if we know it's extended
		if index < &(self.program_stacks.len() as u32) || self.extended_stacks.contains_key(index) {
			return;
		}

		self.extended_stacks.insert(*index, RefCell::new(VecStack::default()));
	}
}