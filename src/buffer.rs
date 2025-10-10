#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub fn next(&mut self) -> bool {
		if self.current < self.lines.len().saturating_sub(1) {
			self.current += 1;
			true
		} else {
			false
		}
	}

	pub fn prev(&mut self) -> bool {
		if self.current > 0 {
			self.current -= 1;
			true
		} else {
			false
		}
	}

	pub fn push(&mut self, line: impl Into<String>) {
		self.lines.push(line.into());
	}

	pub fn get(&self) -> Option<&str> {
		self.lines.get(self.current).map(|s| s.as_str())
	}

	pub fn cursor(&self) -> usize {
		self.current
	}

	pub fn len(&self) -> usize {
		self.lines.len()
	}

	pub fn is_empty(&self) -> bool {
		self.lines.is_empty()
	}
}
