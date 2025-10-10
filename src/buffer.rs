#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub const fn next(&mut self) -> bool {
		if self.current < self.lines.len().saturating_sub(1) {
			self.current += 1;
			true
		} else {
			false
		}
	}

	pub const fn prev(&mut self) -> bool {
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
		self.lines.get(self.current).map(String::as_str)
	}

	pub const fn cursor(&self) -> usize {
		self.current
	}

	pub const fn len(&self) -> usize {
		self.lines.len()
	}
}
