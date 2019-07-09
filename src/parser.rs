#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
	/// Push the index of the column on the left onto the local stack
	PushLeftIndex,
	/// Push the index of the column on the right onto the local stack
	PushRightIndex,
	/// Push the index of the current column to the local stack.
	PushCurrentIndex,
	/// Pop value `a` and begin execution at the `a`th column.
	SetLocalColumn,
	/// Pop value `a` and set the remote stack to the `a`th column.
	SetRemoteStack,
	/// Pop value `a` from the local stack and push to the remote stack.
	MoveToRemote,
	/// Pop value `a` from the remote stack and push to the local stack
	MoveToLocal,
	/// Swap the top two values of the local stack.
	SwapTop,
	/// Duplicate the top value of the local stack.
	DuplicateTop,
	/// Discard the top value of the local stack.
	Discard,
	/// Clear the local stack.
	Clear,
	/// Swap the local and remote stacks.
	SwapStacks,
	/// Reverse the order of the local stack.
	Reverse,
	/// Push a value to the local stack.
	Value(u32),
	/// Skip past the matching `]` if popped value `a` is zero.
	LeftBracket,
	/// Skip back to after the matching `[` if popped value `a` is non-zero.
	RightBracket,
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
	/// Pop values `a` and `b` and push one if they're both non-zero, and push zero otherwise. Not a bitwise AND.
	And,
	/// Pop values `a` and `b` and push one if at least one is non-zero, and push zero if they are both zero. Not a bitwise OR.
	Or,
	/// Invert the top value of the local stack. If it's `0`, push one, otherwise push `1`;
	Invert,
	/// Toggle string mode. Until a matching "string mode" token is executed, characters will be interpreted as raw values.
	StringMode,
	/// Pop a value (interpreted from UTF-8 and push to the stack. If no more are available, push `0`.
	Input,
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
			'<' => Instruction::PushLeftIndex,
			'>' => Instruction::PushRightIndex,
			'.' => Instruction::PushCurrentIndex,
			';' => Instruction::SetLocalColumn,
			'~' => Instruction::SetRemoteStack,
			'^' => Instruction::MoveToRemote,
			'v' => Instruction::MoveToLocal,
			'\\' => Instruction::SwapTop,
			':' => Instruction::DuplicateTop,
			'x' => Instruction::Discard,
			'c' => Instruction::Clear,
			's' => Instruction::SwapStacks,
			'r' => Instruction::Reverse,
			'0'..='9' => Instruction::Value(c.to_digit(10).unwrap()),
			'A'..='F' => Instruction::Value(*c as u32 - 'A' as u32 + 10),
			'[' => Instruction::LeftBracket,
			']' => Instruction::RightBracket,
			'+' => Instruction::Add,
			'-' => Instruction::Subtract,
			'*' => Instruction::Multiply,
			'/' => Instruction::Divide,
			'%' => Instruction::Modulo,
			'=' => Instruction::Equals,
			'`' => Instruction::GreaterThan,
			'&' => Instruction::And,
			'|' => Instruction::Or,
			'!' => Instruction::Invert,
			'"' => Instruction::StringMode,
			'_' => Instruction::Input,
			'$' => Instruction::PrintChar,
			'#' => Instruction::PrintNumber,
			'p' => Instruction::PrintAll,
			'@' => Instruction::Terminate,
			_ => return None
		})
	}
}