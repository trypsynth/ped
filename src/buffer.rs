#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub fn next(&mut self) -> Option<&str> {
		if self.current < self.lines.len().saturating_sub(1) {
			self.current += 1;
			self.get()
		} else {
			None
		}
	}

	pub fn prev(&mut self) -> Option<&str> {
		if self.current > 0 {
			self.current -= 1;
			self.get()
		} else {
			None
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

	pub fn goto(&mut self, line: usize) -> Option<&str> {
		if line > 0 && line <= self.lines.len() {
			self.current = line - 1;
			self.get()
		} else {
			None
		}
	}
}
