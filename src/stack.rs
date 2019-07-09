#[derive(Clone, Default, Debug)]
pub struct VecStack {
	stack: Vec<u32>
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
		(self.stack.pop().unwrap_or_default(), self.stack.pop().unwrap_or_default())
	}

	fn peek(&self) -> u32 {
		*self.stack.last().unwrap()
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
}

pub trait Stack {
	fn values(&self) -> &Vec<u32>;
	fn push(&mut self, value: u32);
	fn pop(&mut self) -> u32;
	fn pop2(&mut self) -> (u32, u32);
	fn peek(&self) -> u32;
	fn clear(&mut self);
	fn reverse(&mut self);
	fn set_all(&mut self, values: Vec<u32>);
}