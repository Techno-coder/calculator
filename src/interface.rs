use std::io::{stdout, Write};

use crossterm::*;

use crate::error::Error;
use crate::span::{Span, Spanned};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

const PROMPT: &str = ">> ";

pub fn interface() -> Result {
	let mut string = String::new();
	let mut reader = crossterm::input().read_sync();
	let _screen = RawScreen::into_raw_mode()?;

	print!("{}", PROMPT.white().bold());
	while let Some(event) = reader.next() {
		match event {
			InputEvent::Keyboard(event) => match event {
				KeyEvent::Enter => evaluate(&mut string)?,
				KeyEvent::Char(character) => {
					queue!(stdout(), Output(character.to_string()))?;
					string.push(character);
					coalesce_anchors(&string)?;
				}
				KeyEvent::Backspace => {
					if string.pop().is_some() {
						queue!(stdout(), Left(1), Output(' '.to_string()), Left(1))?;
					}
					coalesce_anchors(&string)?;
				}
				KeyEvent::Ctrl('c') => break,
				KeyEvent::Ctrl('d') => break,
				KeyEvent::Ctrl('l') => {
					let (_, row) = crossterm::cursor().pos()?;
					queue!(stdout(), ScrollUp(row), Up(row))?;
				}
				KeyEvent::Ctrl('u') => {
					string.clear();
					anchor_start(0)?;
					coalesce_anchors(&string)?;
					queue!(stdout(), Clear(ClearType::UntilNewLine))?;
				}
				_ => (),
			}
			_ => (),
		}
		stdout().flush()?;
	}
	Ok(())
}

fn evaluate(string: &mut String) -> Result {
	let lexer = &mut crate::lexer::Lexer::new(string);
	let coalescence = crate::coalesce::coalesce_root(lexer);
	if let Err(error) = coalescence {
		line_error(&error)?;
		return Ok(());
	}

	line_break()?;
	string.clear();

	let coalescence = coalescence.unwrap();
	let node = crate::parse::parse_root(coalescence);
	match node.evaluate() {
		Ok(evaluation) => print!("{}", evaluation),
		Err(error) => print!("{}{}{}", Colored::Fg(Color::Red),
			error, Colored::Fg(Color::Reset)),
	}

	line_break()?;
	print!("{}", PROMPT.white().bold());
	Ok(())
}

fn coalesce_anchors(string: &str) -> Result {
	let lexer = &mut crate::lexer::Lexer::new(string);
	let coalescence = crate::coalesce::coalesce_root(lexer);
	if let Err(error) = coalescence {
		line_error(&error)?;
		return Ok(());
	}

	clear_buffer()?;
	let coalescence = coalescence.unwrap();
	let anchors = coalescence.coalesce_anchors();
	queue!(stdout(), SavePos, Down(1), SetFg(Color::Yellow))?;

	let (_, row) = crossterm::cursor().pos()?;
	anchors.iter().try_for_each(|offset| queue!(stdout(),
		Goto((PROMPT.len() + offset) as u16, row), Output("^".to_string())))?;

	if let Some(offset) = anchors.last() {
		let byte_end = coalescence.byte_end();
		anchor_start(match byte_end == *offset + 1 {
			false => *offset + 1,
			true => *offset,
		})?;

		let coalesce_length = (coalescence.byte_end() - *offset).saturating_sub(2);
		queue!(stdout(), Output("-".repeat(coalesce_length)), Output("^".to_string()))?;
	}

	Ok(queue!(stdout(), SetFg(Color::Reset), ResetPos)?)
}

fn line_error(error: &Spanned<Error>) -> Result {
	let Span(byte_start, byte_end) = error.span;
	let specific = "^".repeat(byte_end - byte_start).to_owned();
	clear_buffer()?;

	let string = error.node.to_string();
	queue!(stdout(), SavePos, Down(1), SetFg(Color::Red))?;
	anchor_start(byte_start)?;

	Ok(queue!(stdout(), Output(specific), Right(1),
		Output(string), SetFg(Color::Reset), ResetPos)?)
}

fn line_break() -> Result {
	clear_buffer()?;
	let (_, row) = crossterm::cursor().pos()?;
	Ok(queue!(stdout(), Goto(0, row + 1))?)
}

fn anchor_start(offset: usize) -> Result {
	let (_, row) = crossterm::cursor().pos()?;
	Ok(queue!(stdout(), Goto((PROMPT.len() + offset) as u16, row))?)
}

fn clear_buffer() -> Result {
	buffer_line()?;
	Ok(queue!(stdout(), Down(1), Clear(ClearType::CurrentLine), Up(1))?)
}

fn buffer_line() -> Result {
	let (_, cursor_row) = crossterm::cursor().pos()?;
	let (_, terminal_rows) = crossterm::terminal().size()?;
	if cursor_row + 1 == terminal_rows {
		queue!(stdout(), ScrollUp(1), Up(1))?;
	}
	Ok(())
}
