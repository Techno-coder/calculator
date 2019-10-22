use std::io::{Read, stdin, stdout, Write};

use crossterm::{Colorize, Styler};

use crate::context::Context;
use crate::error::Error;
use crate::span::{Span, Spanned};

pub fn basic() -> super::Result {
	print!("{}", super::PROMPT.white().bold());
	stdout().flush()?;

	let context = &mut Context::default();
	while let Ok(count) = stdin().read_line(&mut context.expression) {
		if count == 0 {
			break;
		}

		match evaluate(context) {
			Ok(evaluation) => {
				let index = context.push_value(evaluation);
				super::render::value_index(index);
				super::render::evaluation(evaluation);
				println!();
			}
			Err(error) => {
				let Span(byte_start, byte_end) = error.span;
				let specific = "^".repeat(byte_end - byte_start).to_owned();
				eprintln!("{}{} {}", " ".repeat(super::PROMPT.len() + byte_start),
					specific, error.node);
			}
		}

		print!("{}", super::PROMPT.white().bold());
		context.expression.clear();
		stdout().flush()?;
	}

	Ok(())
}

pub fn evaluate_direct() -> super::Result {
	let context = &mut Context::default();
	stdin().read_to_string(&mut context.expression)?;
	println!("{}", evaluate(context).map_err(|error| error.node)?);
	Ok(())
}

fn evaluate(context: &mut Context) -> Result<f64, Spanned<Error>> {
	let lexer = &mut crate::lexer::Lexer::new(&context.expression);
	let coalescence = crate::coalesce::coalesce_root(lexer)?;
	let node = crate::parse::parse_root(coalescence);
	node.evaluate(context)
}
