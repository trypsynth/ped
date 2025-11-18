use crate::{buffer::EventBuffer, config::Config};

pub enum Command {
	Move(isize),
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
		let trimmed = input.trim();
		if trimmed.is_empty() {
			return Self::Move(1);
		}
		if let Some(step) = Self::parse_relative(trimmed, '+') {
			return Self::Move(step);
		}
		if let Some(step) = Self::parse_relative(trimmed, '-') {
			return Self::Move(-step);
		}
		match trimmed {
			"." => Self::PrintLine,
			".=" => Self::PrintLineNumber,
			"=" => Self::PrintTotalLines,
			"s" => Self::ShowStats,
			"q" => Self::Quit,
			_ => trimmed.parse::<usize>().map_or(Self::Unknown, Self::Goto),
		}
	}

	fn parse_relative(input: &str, symbol: char) -> Option<isize> {
		if input.chars().all(|c| c == symbol) {
			return isize::try_from(input.len()).ok();
		}
		if input.starts_with(symbol) {
			let rest = &input[1..];
			if rest.chars().all(|c| c.is_ascii_digit()) {
				let count = rest.parse::<usize>().ok()?;
				return isize::try_from(count).ok();
			}
		}
		None
	}

	pub fn execute(&self, buffer: &mut EventBuffer, config: &Config) -> bool {
		match self {
			Self::Move(offset) => {
				match buffer.move_by(*offset) {
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
