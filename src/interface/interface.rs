use std::io::{stdout, Write};

use crossterm::*;

use crate::context::Context;

use super::{render, Result};

pub const PROMPT: &str = ">> ";

pub fn interface() -> Result {
	let mut reader = crossterm::input().read_sync();
	let _screen = RawScreen::into_raw_mode()?;

	print!("{}", PROMPT.white().bold());
	let context = &mut Context::default();
	while let Some(event) = reader.next() {
		match event {
			InputEvent::Keyboard(event) => match event {
				KeyEvent::Enter => evaluate(context)?,
				KeyEvent::Char(character) => {
					queue!(stdout(), Output(character.to_string()))?;
					context.expression.push(character);
					coalesce_anchors(&context.expression)?;
				}
				KeyEvent::Backspace => {
					if context.expression.pop().is_some() {
						queue!(stdout(), Left(1), Output(' '.to_string()), Left(1))?;
					}
					coalesce_anchors(&context.expression)?;
				}
				KeyEvent::Ctrl('c') => break,
				KeyEvent::Ctrl('d') => break,
				KeyEvent::Ctrl('l') => {
					let (_, row) = crossterm::cursor().pos()?;
					queue!(stdout(), ScrollUp(row), Up(row))?;
				}
				KeyEvent::Ctrl('u') => {
					context.expression.clear();
					super::render::anchor_start(0)?;
					coalesce_anchors(&context.expression)?;
					queue!(stdout(), Clear(ClearType::UntilNewLine))?;
				}
				KeyEvent::Up => super::history::history_up(context)?,
				KeyEvent::Down => super::history::history_down(context)?,
				_ => (),
			}
			_ => (),
		}
		stdout().flush()?;
	}
	Ok(())
}

fn evaluate(context: &mut Context) -> Result {
	let expression = &mut context.expression;
	let lexer = &mut crate::lexer::Lexer::new(expression);
	let coalescence = crate::coalesce::coalesce_root(lexer);
	if let Err(error) = coalescence {
		render::line_error(&error)?;
		return Ok(());
	}

	render::line_break()?;
	let expression = std::mem::replace(expression, String::new());
	context.push_history(expression);

	let coalescence = coalescence.unwrap();
	let node = crate::parse::parse_root(coalescence);
	match node.evaluate() {
		Err(error) => print!("{}{}{}", Colored::Fg(Color::Red),
			error, Colored::Fg(Color::Reset)),
		Ok(evaluation) => {
			let index = context.push_value(evaluation);
			print!("[{:2x}]: {}", index, evaluation);
		}
	}

	render::line_break()?;
	print!("{}", PROMPT.white().bold());
	Ok(())
}

pub fn coalesce_anchors(string: &str) -> Result {
	let lexer = &mut crate::lexer::Lexer::new(string);
	let coalescence = crate::coalesce::coalesce_root(lexer);
	if let Err(error) = coalescence {
		render::line_error(&error)?;
		return Ok(());
	}

	render::clear_buffer()?;
	let coalescence = coalescence.unwrap();
	let anchors = coalescence.coalesce_anchors();
	queue!(stdout(), SavePos, Down(1), SetFg(Color::Yellow))?;

	let (_, row) = crossterm::cursor().pos()?;
	anchors.iter().try_for_each(|offset| queue!(stdout(),
		Goto((PROMPT.len() + offset) as u16, row), Output("^".to_string())))?;

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

