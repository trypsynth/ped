use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
	pub name: String,
}

impl Config {
	pub fn path() -> Result<PathBuf> {
		Ok(dirs::home_dir().context("failed to get home directory")?.join(".ped"))
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
		let mut buf = Vec::new();
		let formatter = PrettyFormatter::with_indent(b"\t");
		let mut ser = Serializer::with_formatter(&mut buf, formatter);
		self.serialize(&mut ser)?;
		fs::write(path, buf).context("failed to save configuration")?;
		Ok(())
	}
}
