use crate::buffer::EventBuffer;

pub enum Command {
	NextLine,
	PrevLine,
	PrintLine,
	PrintLineNumber,
	PrintTotalLines,
	Goto(usize),
	Quit,
	Unknown,
}

impl Command {
	pub fn parse(input: &str) -> Self {
		match input {
			"" => Self::NextLine,
			"-" => Self::PrevLine,
			"." => Self::PrintLine,
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

	pub fn execute(&self, buffer: &mut EventBuffer) -> bool {
		match self {
			Self::NextLine => {
				if buffer.next() {
					if let Some(line) = buffer.get() {
						println!("{line}");
					}
				} else {
					println!("?");
				}
				true
			}
			Self::PrevLine => {
				if buffer.prev() {
					if let Some(line) = buffer.get() {
						println!("{line}");
					}
				} else {
					println!("?");
				}
				true
			}
			Self::PrintLine => {
				if let Some(line) = buffer.get() {
					println!("{line}");
				}
				true
			}
			Self::PrintLineNumber => {
				println!("{}", buffer.line_number());
				true
			}
			Self::PrintTotalLines => {
				println!("{}", buffer.len());
				true
			}
			Self::Goto(line) => {
				if buffer.goto(*line) {
					if let Some(content) = buffer.get() {
						println!("{content}");
					}
				} else {
					println!("?");
				}
				true
			}
			Self::Quit => false,
			Self::Unknown => {
				println!("?");
				true
			}
		}
	}
}
