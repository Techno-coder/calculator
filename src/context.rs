use std::collections::HashMap;

use crate::error::Error;

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

	pub fn variable(&self, variable: &str) -> Result<f64, Error> {
		Ok(match variable.chars().all(|character| character == '$') {
			false => self.variables.get(variable)
				.ok_or_else(|| Error::UndefinedVariable(variable.to_owned())),
			true => {
				let index = self.current_index.checked_sub(variable.len() + 1)
					.ok_or(Error::InvalidEvaluationOffset)?;
				self.variables.get(&format!("{:x}", index))
					.ok_or(Error::InvalidEvaluationOffset)
			}
		}?.clone())
	}

	pub fn push_history(&mut self, expression: String) {
		self.history.push(expression);
		self.history_offset = 0;
	}

	pub fn history(&self) -> Option<&str> {
		let index = self.history.len().checked_sub(self.history_offset)?;
		self.history.get(index).map(String::as_str)
	}
}
