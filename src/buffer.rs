#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub fn next(&mut self) -> Option<&str> {
		self.move_by(1)
	}

	pub fn prev(&mut self) -> Option<&str> {
		self.move_by(-1)
	}

	pub fn move_by(&mut self, offset: isize) -> Option<&str> {
		if self.lines.is_empty() {
			return None;
		}
		let Some(target) = (self.current as isize).checked_add(offset) else {
			return None;
		};
		if !(0..self.lines.len() as isize).contains(&target) {
			return None;
		}
		self.current = target as usize;
		self.get()
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
		self.lines.get(line - 1).map(|s| {
			self.current = line - 1;
			s.as_str()
		})
	}
}
