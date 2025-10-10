use crate::{buffer::EventBuffer, config::Config};

pub enum Command {
	NextLine,
	PrevLine,
	PrintLine,
	PrintLineNumber,
	PrintTotalLines,
	Goto(usize),
	ShowStats,
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
			"s" => Self::ShowStats,
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

	pub fn execute(&self, buffer: &mut EventBuffer, config: &Config) -> bool {
		match self {
			Self::NextLine => {
				buffer.next();
				true
			}
			Self::PrevLine => {
				buffer.prev();
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
				buffer.goto(*line);
				true
			}
			Self::ShowStats => {
				let stats = config.display_stats();
				buffer.push(stats);
				buffer.goto(buffer.len());
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
