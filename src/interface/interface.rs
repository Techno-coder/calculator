use std::io::{stdout, Write};

use crossterm::*;

use crate::context::Context;

use super::{render, Result};

/// Spawns an interface with immediate expression verification.
pub fn interface() -> Result {
	let mut reader = crossterm::input().read_sync();
	let _screen = RawScreen::into_raw_mode()?;
	print!("{}", super::PROMPT.white().bold());
	stdout().flush()?;

	let context = &mut Context::default();
	while let Some(event) = reader.next() {
		match event {
			InputEvent::Keyboard(event) => match event {
				KeyEvent::Enter => evaluate(context, true)?,
				KeyEvent::Char(key_character) => character(context, key_character)?,
				KeyEvent::Backspace => erase(context)?,
				KeyEvent::Ctrl('c') => break,
				KeyEvent::Ctrl('d') => break,
				KeyEvent::Ctrl('l') => {
					let (_, row) = crossterm::cursor().pos()?;
					queue!(stdout(), ScrollUp(row), Up(row))?;
				}
				KeyEvent::Ctrl('u') => {
					context.expression.clear();
					super::render::anchor_start(0)?;
					context.cursor_position = 0;

					evaluate(context, false)?;
					queue!(stdout(), Clear(ClearType::UntilNewLine))?;
				}
				KeyEvent::Ctrl('w') => erase_group(context)?,
				KeyEvent::Up => super::history::history_up(context)?,
				KeyEvent::Down => super::history::history_down(context)?,
				KeyEvent::Left if context.cursor_position >= 1 => {
					queue!(stdout(), Left(1))?;
					context.cursor_position -= 1;
				}
				KeyEvent::Right => {
					let count = context.expression.chars().count();
					if context.cursor_position < count {
						queue!(stdout(), Right(1))?;
						context.cursor_position += 1;
					}
				}
				_ => (),
			}
			_ => (),
		}
		stdout().flush()?;
	}
	Ok(())
}

fn character(context: &mut Context, character: char) -> Result {
	let index = context.expression.char_indices()
		.nth(context.cursor_position).map(|(index, _)| index)
		.unwrap_or(context.expression.len());
	context.expression.insert(index, character);

	let slice = &context.expression[index..];
	execute!(stdout(), SavePos, Output(slice.to_string()),
		ResetPos, Right(1))?;

	context.cursor_position += 1;
	evaluate(context, false)
}

fn erase(context: &mut Context) -> Result {
	match context.cursor_position >= 1 {
		false => Ok(()),
		true => {
			context.cursor_position -= 1;
			let index = context.expression.char_indices()
				.nth(context.cursor_position).map(|(index, _)| index).unwrap();
			context.expression.remove(index);

			execute!(stdout(), Left(1), SavePos, Clear(ClearType::UntilNewLine),
				Output(context.expression[index..].to_string()), ResetPos)?;
			evaluate(context, false)
		}
	}
}

fn erase_group(context: &mut Context) -> Result {
	let end_index = context.expression.char_indices()
		.nth(context.cursor_position).map(|(index, _)| index)
		.unwrap_or(context.expression.len());
	let start_index = context.expression[..end_index].char_indices().rev()
		.skip_while(|(_, character)| character.is_whitespace())
		.skip_while(|(_, character)| !character.is_whitespace())
		.take_while(|(_, character)| character.is_whitespace())
		.last().map(|(index, _)| index).unwrap_or(0);

	let count = context.expression[start_index..end_index].chars().count();
	match count > 0 {
		false => Ok(()),
		true => {
			context.cursor_position -= count;
			context.expression.replace_range(start_index..end_index, "");
			execute!(stdout(), Left(count as u16), SavePos, Clear(ClearType::UntilNewLine),
				Output(context.expression[start_index..].to_string()), ResetPos)?;
			evaluate(context, false)
		}
	}
}

pub fn evaluate(context: &mut Context, store: bool) -> Result {
	let difference = (context.expression.chars().count() - context.cursor_position) as u16;
	queue!(stdout(), Right(difference), Clear(ClearType::UntilNewLine))?;

	let coalescence = match super::check::check(context)? {
		None => return render::anchor_start(context.cursor_position),
		Some(coalescence) => coalescence,
	};

	if store {
		let expression = &mut context.expression;
		let expression = std::mem::replace(expression, String::new());
		context.cursor_position = 0;
		context.push_history(expression);
	}

	let node = crate::parse::parse_root(coalescence);
	match node.evaluate(context) {
		Err(error) => {
			render::line_error(&error)?;
			render::line_break(false)?;
		}
		Ok(evaluation) => match store {
			true => {
				queue!(stdout(), Clear(ClearType::UntilNewLine))?;
				render::line_break(true)?;
				let index = context.push_value(evaluation);
				render::value_index(index);
				render::evaluation(evaluation, None);
			}
			false => {
				print!(" {}= ", Colored::Fg(Color::Green));
				render::evaluation(evaluation, Some(Color::Green));
				return render::anchor_start(context.cursor_position);
			}
		}
	}

	render::line_break(false)?;
	print!("{}", super::PROMPT.white().bold());
	Ok(())
}

