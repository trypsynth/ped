#[derive(Default)]
pub struct EventBuffer {
	lines: Vec<String>,
	current: usize,
}

impl EventBuffer {
	pub fn move_by(&mut self, offset: isize) -> Option<&str> {
		if self.lines.is_empty() {
			return None;
		}
		let target = if offset >= 0 {
			self.current.checked_add(usize::try_from(offset).ok()?)?
		} else {
			self.current.checked_sub(offset.unsigned_abs())?
		};
		if target >= self.lines.len() {
			return None;
		}
		self.current = target;
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
