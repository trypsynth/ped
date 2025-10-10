pub enum Command {
	Next,
	Prev,
	PrintCurrent,
	PrintLineNumber,
	PrintTotalLines,
	Goto(usize),
	Quit,
	Unknown,
}

impl Command {
	pub fn parse(input: &str) -> Self {
		match input {
			"" => Self::Next,
			"-" => Self::Prev,
			"." => Self::PrintCurrent,
			".=" => Self::PrintLineNumber,
			"=" => Self::PrintTotalLines,
			"q" => Self::Quit,
			_ => {
				if let Ok(line) = input.parse::<usize>() {
					Self::Goto(line)
				} else {
					Self::Unknown
				}
			}
		}
	}
}
