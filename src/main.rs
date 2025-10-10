#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod buffer;
mod commands;
mod config;

use std::process;

use anyhow::Result;
use rustyline::DefaultEditor;

use crate::{buffer::EventBuffer, commands::Command, config::Config};

fn main() -> Result<()> {
	let mut config = Config::load()?;
	if config.name.is_empty() {
		let Some(name) = ask_for_name() else {
			process::exit(1);
		};
		config.name = name;
		config.save()?;
	}
	let mut buffer = EventBuffer::default();
	for i in 1..5 {
		buffer.push(format!("Hi {i}!"));
	}
	let mut rl = DefaultEditor::new()?;
	while let Ok(input) = rl.readline("") {
		let cmd = Command::parse(input.trim());
		match cmd {
			Command::Next => {
				if buffer.next() {
					if let Some(line) = buffer.get() {
						println!("{line}");
					}
				} else {
					println!("?");
				}
			}
			Command::Prev => {
				if buffer.prev() {
					if let Some(line) = buffer.get() {
						println!("{line}");
					}
				} else {
					println!("?");
				}
			}
			Command::PrintCurrent => {
				if let Some(line) = buffer.get() {
					println!("{line}");
				}
			}
			Command::PrintLineNumber => {
				println!("{}", buffer.line_number());
			}
			Command::PrintTotalLines => {
				println!("{}", buffer.len());
			}
			Command::Goto(line) => {
				if buffer.goto(line) {
					if let Some(content) = buffer.get() {
						println!("{content}");
					}
				} else {
					println!("?");
				}
			}
			Command::Quit => break,
			Command::Unknown => println!("?"),
		}
	}
	Ok(())
}

fn ask_for_name() -> Option<String> {
	let mut rl = DefaultEditor::new().ok()?;
	let line = rl.readline("Name: ").ok()?;
	let name = line.trim();
	if name.is_empty() { None } else { Some(name.to_string()) }
}
