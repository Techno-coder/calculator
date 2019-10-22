use std::io::{stdout, Write};

use crossterm::*;

use crate::error::Error;
use crate::span::{Span, Spanned};

use super::Result;

pub fn value_index(index: usize)  {
	print!("{}{:x}{} ", "[".white().bold(), index, "]".white().bold());
}

pub fn evaluation(evaluation: f64) {
	let exponentiation_range = 1e-3 < evaluation.abs() && evaluation.abs() < 1e9;
	match exponentiation_range || !evaluation.is_normal() {
		true => print!("{}", evaluation),
		false => {
			let string = format!("{:e}", evaluation);
			print!("{}{}{}", &string[..string.find('e').unwrap()],
				"e".white().bold(), &string[string.find('e').unwrap() + 1..]);
		}
	}
}

pub fn line_error(error: &Spanned<Error>) -> Result {
	let Span(byte_start, byte_end) = error.span;
	let specific = "^".repeat(byte_end - byte_start).to_owned();
	clear_buffer()?;

	let string = error.node.to_string();
	queue!(stdout(), SavePos, Down(1), SetFg(Color::Red))?;
	anchor_start(byte_start)?;

	Ok(queue!(stdout(), Output(specific), Right(1),
		Output(string), SetFg(Color::Reset), ResetPos)?)
}

pub fn line_break(clear: bool) -> Result {
	match clear {
		true => clear_buffer(),
		false => buffer_line(),
	}?;

	let (_, row) = crossterm::cursor().pos()?;
	Ok(queue!(stdout(), Goto(0, row + 1))?)
}

pub fn anchor_start(offset: usize) -> Result {
	let (_, row) = crossterm::cursor().pos()?;
	Ok(queue!(stdout(), Goto((super::PROMPT.len() + offset) as u16, row))?)
}

pub fn clear_buffer() -> Result {
	buffer_line()?;
	Ok(queue!(stdout(), Down(1), Clear(ClearType::CurrentLine), Up(1))?)
}

pub fn buffer_line() -> Result {
	let (_, cursor_row) = crossterm::cursor().pos()?;
	let (_, terminal_rows) = crossterm::terminal().size()?;
	if cursor_row + 1 == terminal_rows {
		queue!(stdout(), ScrollUp(1), Up(1))?;
	}
	Ok(())
}
