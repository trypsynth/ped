#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod buffer;
mod commands;
mod config;

use std::{
	io::{self, BufRead, Write},
	process,
};

use anyhow::Result;

use crate::{buffer::EventBuffer, commands::Command, config::Config};

fn main() -> Result<()> {
	let mut buffer = EventBuffer::default();
	let mut config = Config::load()?;
	if config.name.is_empty() {
		let Some(name) = ask_for_name() else {
			process::exit(1);
		};
		config.name = name;
		config.save()?;
		buffer.push(format!("{} welcomes you.", config.name));
	}
	let stdin = io::stdin();
	let mut stdin = stdin.lock();
	let mut line = String::new();
	loop {
		line.clear();
		let bytes_read = stdin.read_line(&mut line)?;
		if bytes_read == 0 {
			break;
		}
		let cmd = Command::parse(line.trim());
		if !cmd.execute(&mut buffer, &config) {
			break;
		}
	}
	Ok(())
}

fn ask_for_name() -> Option<String> {
	print!("Name: ");
	io::stdout().flush().ok()?;
	let mut line = String::new();
	io::stdin().read_line(&mut line).ok()?;
	let name = line.trim();
	if name.is_empty() { None } else { Some(name.to_string()) }
}
