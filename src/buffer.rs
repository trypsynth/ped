#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub fn next(&mut self) {
		if self.current < self.lines.len().saturating_sub(1) {
			self.current += 1;
			if let Some(line) = self.get() {
				println!("{line}");
			}
		} else {
			println!("?");
		}
	}

	pub fn prev(&mut self) {
		if self.current > 0 {
			self.current -= 1;
			if let Some(line) = self.get() {
				println!("{line}");
			}
		} else {
			println!("?");
		}
	}

	pub fn push(&mut self, line: impl Into<String>) {
		self.lines.push(line.into());
	}

	pub fn get(&self) -> Option<&str> {
		self.lines.get(self.current).map(String::as_str)
	}

	pub const fn len(&self) -> usize {
		self.lines.len()
	}

	pub const fn line_number(&self) -> usize {
		self.current + 1
	}

	pub fn goto(&mut self, line: usize) {
		if line > 0 && line <= self.lines.len() {
			self.current = line - 1;
			if let Some(content) = self.get() {
				println!("{content}");
			}
		} else {
			println!("?");
		}
	}
}
