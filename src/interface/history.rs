use std::io::{stdout, Write};

use crossterm::*;

use crate::context::Context;

use super::{interface, Result};

pub fn history_up(context: &mut Context) -> Result {
	context.history_offset += 1;
	match context.history().map(ToOwned::to_owned) {
		None => context.history_offset -= 1,
		Some(history) => {
			if context.expression.len() > 0 {
				queue!(stdout(), Left(context.expression.len() as u16))?;
			}

			context.expression = history;
			context.cursor_position = context.expression.len();
			queue!(stdout(), Clear(ClearType::UntilNewLine), Output(context.expression.clone()))?;
			interface::evaluate(context, false)?;
		}
	}
	Ok(())
}

pub fn history_down(context: &mut Context) -> Result {
	context.history_offset = context.history_offset.saturating_sub(1);
	let current_length = context.expression.len();
	match context.history_offset {
		0 => context.expression.clear(),
		_ => context.expression = context.history().unwrap().to_owned(),
	}

	let expression = &mut context.expression;
	context.cursor_position = expression.len();
	if current_length > 0 {
		queue!(stdout(), Left(current_length as u16))?;
	}

	queue!(stdout(), Clear(ClearType::UntilNewLine), Output(expression.clone()))?;
	interface::evaluate(context, false)?;
	Ok(())
}
