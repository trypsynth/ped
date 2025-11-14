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
		if input.trim().is_empty() {
			return Self::NextLine;
		}
		match input {
			"-" => Self::PrevLine,
			"." => Self::PrintLine,
			".=" => Self::PrintLineNumber,
			"=" => Self::PrintTotalLines,
			"s" => Self::ShowStats,
			"q" => Self::Quit,
			_ => input.parse::<usize>().map_or(Self::Unknown, Self::Goto),
		}
	}

	pub fn execute(&self, buffer: &mut EventBuffer, config: &Config) -> bool {
		match self {
			Self::NextLine => {
				match buffer.next() {
					Some(line) => println!("{line}"),
					None => println!("?"),
				}
				true
			}
			Self::PrevLine => {
				match buffer.prev() {
					Some(line) => println!("{line}"),
					None => println!("?"),
				}
				true
			}
			Self::PrintLine => {
				if let Some(line) = buffer.get() {
					println!("{line}");
				} else {
					println!("?");
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
				match buffer.goto(*line) {
					Some(line) => println!("{line}"),
					None => println!("?"),
				}
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
