pub struct ProgramState {
	stacks: Vec<Vec<u8>>,
	output: String
}

#[derive(Default)]
pub struct Interpreter<'a> {
	program: Program,
	reader: Option<&'a mut Read>,
	writer: Option<&'a mut Write>,
	stacks: Vec<Vec<u8>>
}

impl<'a> Interpreter<'a> {
	/// Create a new col interpeter from a program
	pub fn new<R: Read, W: Write>(program: Program, reader: &'a mut R, writer: &'a mut W) -> Self {
		let mut interpeter = Self::default();
		Interpreter {
			program, reader, writer, stacks: vec![]
		}
	}

	/// Assign a reader for input as defined by the col spec.
	pub fn read_from<R: Read>(&mut self, reader: &'a mut R) -> &mut Self {
		self.reader = Some(reader);
		self
	}

	/// Assign a writer for output as defined by the col spec.
	pub fn write_to<W: Write>(&mut self, writer: &'a mut W) -> &mut Self {
		self.writer = Some(writer);
		self
	}

	/// Executes the program until it terminates.
	pub fn run(&self) -> Result<(), Error> {
		Ok(())
	}
}