use std::{
	fs::{self, File},
	io::Write,
	path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, ser::PrettyFormatter};

#[derive(Deserialize, Serialize)]
pub struct Config {
	#[serde(default)]
	pub name: String,
	#[serde(default = "default_hunger")]
	pub hunger: u8,
	#[serde(default = "default_happiness")]
	pub happiness: u8,
	#[serde(default = "default_energy")]
	pub energy: u8,
	#[serde(default = "default_health")]
	pub health: u8,
}

fn default_hunger() -> u8 {
	50
}
fn default_happiness() -> u8 {
	50
}
fn default_energy() -> u8 {
	50
}
fn default_health() -> u8 {
	100
}

impl Default for Config {
	fn default() -> Self {
		Self { name: String::new(), hunger: 50, happiness: 50, energy: 50, health: 100 }
	}
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

	pub fn display_stats(&self) -> String {
		format!(
			"name: {}, Hunger: {}%, happiness: {}%, energy: {}%, health: {}%.",
			self.name, self.hunger, self.happiness, self.energy, self.health
		)
	}
}
