use std::io::{stdout, Write};

use crossterm::*;

use crate::context::Context;

use super::{check, render, Result};

pub const PROMPT: &str = ">> ";

pub fn interface() -> Result {
	let mut reader = crossterm::input().read_sync();
	let _screen = RawScreen::into_raw_mode()?;
	print!("{}", PROMPT.white().bold());
	stdout().flush()?;

	let context = &mut Context::default();
	while let Some(event) = reader.next() {
		match event {
			InputEvent::Keyboard(event) => match event {
				KeyEvent::Enter => evaluate(context)?,
				KeyEvent::Char(character) => {
					execute!(stdout(), Output(character.to_string()))?;
					context.expression.push(character);
					check::check(context)?;
				}
				KeyEvent::Backspace => {
					if context.expression.pop().is_some() {
						execute!(stdout(), Left(1), Output(' '.to_string()), Left(1))?;
					}
					check::check(context)?;
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
					check::check(context)?;
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
	let coalescence = match super::check::check(context)? {
		Some(coalescence) => coalescence,
		None => return Ok(()),
	};

	let expression = &mut context.expression;
	let expression = std::mem::replace(expression, String::new());
	context.push_history(expression);

	let node = crate::parse::parse_root(coalescence);
	match node.evaluate(context) {
		Err(error) => {
			render::line_error(&error)?;
			render::line_break(false)?;
		}
		Ok(evaluation) => {
			render::line_break(true)?;
			let index = context.push_value(evaluation);
			print!("{}{:x}{} ", "[".white().bold(), index, "]".white().bold());

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
	}

	render::line_break(false)?;
	print!("{}", PROMPT.white().bold());
	Ok(())
}

