use std::io::{stdout, Write};

use crossterm::*;

use crate::coalescence::Coalescence;
use crate::context::Context;

use super::render;

type CheckResult = std::result::Result<Option<Coalescence>, Box<dyn std::error::Error>>;

pub fn check(context: &Context) -> CheckResult {
	let lexer = &mut crate::lexer::Lexer::new(&context.expression);
	let coalescence = crate::coalesce::coalesce_root(lexer);
	if let Err(error) = coalescence {
		render::line_error(&error)?;
		return Ok(None);
	}

	let coalescence = coalescence.unwrap();
	coalesce_anchors(&coalescence)?;

	if let Err(error) = coalescence.verify(context) {
		render::line_error(&error)?;
		return Ok(None);
	}

	Ok(Some(coalescence))
}

pub fn coalesce_anchors(coalescence: &Coalescence) -> super::Result {
	render::clear_buffer()?;
	let anchors = coalescence.coalesce_anchors();
	queue!(stdout(), SavePos, Down(1), SetFg(Color::Yellow))?;

	let (_, row) = crossterm::cursor().pos()?;
	anchors.iter().try_for_each(|offset| queue!(stdout(),
		Goto((super::interface::PROMPT.len() + offset) as u16, row),
		Output("^".to_string())))?;

	if let Some(offset) = anchors.last() {
		let byte_end = coalescence.byte_end();
		render::anchor_start(match byte_end == *offset + 1 {
			false => *offset + 1,
			true => *offset,
		})?;

		let coalesce_length = (coalescence.byte_end() - *offset).saturating_sub(2);
		queue!(stdout(), Output("-".repeat(coalesce_length)), Output("^".to_string()))?;
	}

	Ok(queue!(stdout(), SetFg(Color::Reset), ResetPos)?)
}

