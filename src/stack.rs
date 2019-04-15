#[derive(Default, Debug)]
pub struct VecStack {
	stack: Vec<u8>
}

impl Stack for VecStack {
	fn push(&mut self) {
		unimplemented!()
	}

	fn pop(&mut self) -> u8 {
		unimplemented!()
	}

	fn peek(&mut self) -> u8 {
		unimplemented!()
	}

	fn duplicate_top(&mut self) {
		unimplemented!()
	}

	fn swap_top(&mut self) {
		unimplemented!()
	}

	fn clear(&mut self) {
		unimplemented!()
	}

	fn reverse(&mut self) {
		unimplemented!()
	}
}

pub trait Stack {
	fn push(&mut self);
	fn pop(&mut self) -> u8;
	fn peek(&mut self) -> u8;
	fn duplicate_top(&mut self);
	fn swap_top(&mut self);
	fn clear(&mut self);
	fn reverse(&mut self);
}