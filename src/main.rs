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
	buffer.push(format!("{} welcomes you.", config.name));
	let mut rl = DefaultEditor::new()?;
	while let Ok(input) = rl.readline("") {
		let cmd = Command::parse(input.trim());
		if !cmd.execute(&mut buffer, &config) {
			break;
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
