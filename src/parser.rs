use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum Instruction {
	/// Push the index of the column on the left onto the local stack
	LeftIndex,
	/// Push the index of the column on the right onto the local stack
	RightIndex,
	/// Push the index of the current column to the local stack.
	CurrentIndex,
	/// Pop value `a` and begin execution at the `a`th column.
	SetLocalColumn,
	/// Pop value `a` and set the remote stack to the `a`th column.
	SetRemoteStack,
	/// Pop value `a` from the local stack and push to the remote stack.
	MoveToRemote,
	/// Pop value `a` from the remote stack and push to the local stack
	MoveToLocal,
	/// Discard the top value of the local stack.
	Discard,
	/// Swap the top two values of the local stack.
	SwapTop,
	/// Duplicate the top value of the local stack.
	Duplicate,
	/// Clear the local stack.
	Clear,
	/// Swap the local and remote stacks.
	SwapStacks,
	/// Reverse the order of the local stack.
	Reverse,
	/// Push a value to the local stack.
	Value(u8),
	/// Pop `a` and only execute the following instruction if `a` is not zero.
	If,
	/// Pop values `a` and `b` off the local stack and push the result of `b` plus `a`.
	Add,
	/// Pop values `a` and `b` off the local stack and push the result of `b` minus `a`.
	Subtract,
	/// Pop values `a` and `b` off the local stack and push the result of `b` times `a`.
	Multiply,
	/// Pop values `a` and `b` off the local stack and push the result of `b` divided by `a`.
	Divide,
	/// Pop values `a` and `b` off the local stack and push the remainder of the integer division of `b` divided by `a`.
	Modulo,
	/// Pop values `a` and `b` and push `1` if `a` equals `b` and `0` otherwise.
	Equals,
	/// Pop values `a` and `b` and push `1` if `b` is greater than `a` and `0` otherwise.
	GreaterThan,
	/// Invert the top value of the local stack. If it's `0`, push one, otherwise push `1`;
	Invert,
	/// Toggle string mode. Until a matching "string mode" token is executed, characters will be interpreted as raw values.
	StringMode,
	/// Pop a value (interpreted from UTF-8 and push to the stack. If no more are available, push `0`.
	Input,
	/// Skip the next instruction.
	Skip,
	/// Pop `a` and print its UTF-8 value.
	PrintChar,
	/// Pop `a` and print its numeric value.
	PrintNumber,
	/// Print all values in stack (from top to bottom) as UTF-8 characters.
	PrintAll,
	/// Terminate the entire program.
	Terminate
}

impl Instruction {
	/// Attempts to convert a single char to a col instruction
	pub fn from_char(c: &char) -> Option<Instruction> {
		Some(match c {
			'<' => Instruction::LeftIndex,
			'>' => Instruction::RightIndex,
			'.' => Instruction::CurrentIndex,
			';' => Instruction::SetLocalColumn,
			'~' => Instruction::SetRemoteStack,
			'^' => Instruction::MoveToRemote,
			'v' => Instruction::MoveToLocal,
			'&' => Instruction::Discard,
			'\\' => Instruction::SwapTop,
			':' => Instruction::Duplicate,
			'c' => Instruction::Clear,
			's' => Instruction::SwapStacks,
			'r' => Instruction::Reverse,
			'0'..'9' => Instruction::Value(c.to_digit(10).unwrap() as u8),
			'?' => Instruction::If,
			'+' => Instruction::Add,
			'-' => Instruction::Subtract,
			'*' => Instruction::Multiply,
			'/' => Instruction::Divide,
			'%' => Instruction::Modulo,
			'=' => Instruction::Equals,
			'`' => Instruction::GreaterThan,
			'!' => Instruction::Invert,
			'"' => Instruction::StringMode,
			'_' => Instruction::Input,
			'|' => Instruction::Skip,
			'$' => Instruction::PrintChar,
			'#' => Instruction::PrintNumber,
			'p' => Instruction::PrintAll,
			'@' => Instruction::Terminate,
			_ => return None
		})
	}
}