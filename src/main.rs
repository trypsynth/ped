mod buffer;
mod config;

use std::process;

use anyhow::Result;
use rustyline::DefaultEditor;

use crate::{buffer::EventBuffer, config::Config};

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
		buffer.push(&format!("Hi {i}!"));
	}
	let mut rl = DefaultEditor::new()?;
	loop {
		let input = match rl.readline("") {
			Ok(line) => line,
			Err(_) => break,
		};
		let cmd = input.trim();
		match cmd {
			"" => {
				if buffer.next() {
					if let Some(line) = buffer.get() {
						println!("{}", line);
					}
				} else {
					println!("?");
				}
			}
			"-" => {
				if buffer.prev() {
					if let Some(line) = buffer.get() {
						println!("{}", line);
					}
				} else {
					println!("?");
				}
			}
			"q" => break,
			_ => {}
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
