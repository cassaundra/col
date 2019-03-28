// string mode toggle is here but chars interpreted there are not
// ^ interpreter level
pub enum Instruction {
	/// Push the index of the column on the left onto the local stack
	LeftIndex,
	/// Push the index of the column on the right onto the local stack
	RightIndex,
	/// Pop value `a` and begin execution at the `a`th column.
	SetColumn,
	/// Pop value `a` and set the remote stack to the `a`th column.
	RemoteStack,
	/// Pop value `a` from the local stack and push to the remote stack.
	MoveToRemote,
	/// Pop value `a` from the remote stack and push to the local stack
	MoveToLocal,
	/// Discard the top value of the local stack.
	Discard,
	/// Swap the top two values of the local stack.
	Swap,
	/// Duplicate the top value of the local stack.
	Duplicate,
	/// Clear the local stack.
	Clear,
	/// Push a value to the local stack.
	PushValue(u8),
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
	OutputChar,
	/// Pop `a` and print its numeric value.
	OutputNumber,
	/// Terminate the entire program.
	Terminate
}