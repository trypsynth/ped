use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
	pub name: String,
}

impl Config {
	pub fn path() -> Result<PathBuf> {
		let base = dirs::config_dir().context("failed to get config directory")?.join("ped");
		Ok(base.join("config.json"))
	}

	pub fn load() -> Result<Self> {
		let path = Self::path()?;
		if !path.exists() {
			return Ok(Self::default());
		}
		let contents = fs::read_to_string(&path).context("failed to read config file")?;
		serde_json::from_str(&contents).context("failed to parse config")
	}

	pub fn save(&self) -> Result<()> {
		let path = Self::path()?;
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent).context("failed to create config directory")?;
		}
		let mut buf = Vec::new();
		let formatter = PrettyFormatter::with_indent(b"\t");
		let mut ser = Serializer::with_formatter(&mut buf, formatter);
		self.serialize(&mut ser)?;
		let mut file = File::create(&path).context("failed to create config file")?;
		file.write_all(&buf)?;
		file.write_all(b"\n")?;
		Ok(())
	}
}
