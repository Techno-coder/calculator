use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Context {
	current_index: usize,
	variables: HashMap<String, f64>,
	history: Vec<String>,

	pub history_offset: usize,
	pub expression: String,
}

impl Context {
	pub fn push_value(&mut self, value: f64) -> usize {
		loop {
			let index_key = format!("{:x}", self.current_index);
			if self.variables.contains_key(&index_key) {
				self.current_index += 1;
				continue;
			}

			self.current_index += 1;
			self.variables.insert(index_key, value);
			break self.current_index - 1;
		}
	}

	pub fn push_history(&mut self, expression: String) {
		self.history.push(expression);
	}

	pub fn history(&self) -> Option<&str> {
		let index = self.history.len().checked_sub(self.history_offset)?;
		self.history.get(index).map(String::as_str)
	}
}
