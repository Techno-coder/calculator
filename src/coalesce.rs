use crate::coalescence::Coalescence;
use crate::error::Error;
use crate::item::Function;
use crate::lexer::Lexer;
use crate::span::{Span, Spanned};
use crate::token::{Operator, Token};

pub fn coalesce_root(lexer: &mut Lexer) -> Result<Coalescence, Spanned<Error>> {
	coalesce(lexer, false, false)
}

fn coalesce(lexer: &mut Lexer, mut last_valued: bool, expect_parenthesis: bool)
            -> Result<Coalescence, Spanned<Error>> {
	let mut last_byte_end = 0;
	let mut coalesces = Vec::new();
	while let Some(token) = lexer.next() {
		let token = token?;
		let span = token.span;
		last_byte_end = token.span.byte_end();
		match token.node {
			Token::ParenthesisClose if expect_parenthesis => match coalesces.is_empty() {
				true => return Err(token.map(Error::EmptyBrackets)),
				false => break,
			},
			Token::ParenthesisClose => return Err(token.map(Error::MismatchedBracket)),
			Token::ParenthesisOpen => {
				if last_valued {
					let operator = Spanned::new(Operator::Multiply, span);
					coalesces.push(Coalescence::Operator(operator));
				}

				coalesces.push(coalesce(lexer, false, true)?);
				last_valued = true;
			}
			Token::Operator(operator) => {
				if !last_valued {
					match operator {
						Operator::Minus => {
							let function = Spanned::new(Function::UnaryMinus, span);
							coalesces.push(Coalescence::Function(function));
							continue;
						}
						_ => return Err(token.map(Error::ExpectedValued)),
					}
				}

				coalesces.push(Coalescence::Operator(token.map(operator)));
				last_valued = false;
			}
			Token::Terminal(terminal) => value(&mut coalesces, &mut last_valued,
				Coalescence::Terminal(token.map(terminal)), span)?,
			Token::Variable(variable) => value(&mut coalesces, &mut last_valued,
				Coalescence::Variable(Spanned::new(variable, span)), span)?,
			Token::Function(function) => match last_valued {
				false => coalesces.push(Coalescence::Function(Spanned::new(function, span))),
				true => return Err(Spanned::new(Error::ExpectedOperator, span)),
			},
			Token::Constant(constant) => value(&mut coalesces, &mut last_valued,
				Coalescence::Terminal(Spanned::new(constant.value(), span)), span)?,
			Token::Coalesce(mut count) => {
				count += 1;
				let mut iterator = coalesces.iter().enumerate().rev();
				while let Some((index, coalesce)) = iterator.next() {
					match coalesce {
						Coalescence::Operator(_) => continue,
						_ => count -= 1,
					}

					if count == 0 {
						let coalescence = coalesces.split_off(index);
						coalesces.push(Coalescence::Multiple(coalescence));
						break;
					}
				}

				if count > 0 {
					return Err(token.map(Error::InvalidCoalesce));
				}
			}
		}
	}

	let last_span = Span(last_byte_end, last_byte_end + 1);
	match last_valued {
		true => Ok(Coalescence::Multiple(coalesces)),
		false => Err(Spanned::new(Error::ExpectedValued, last_span))
	}
}

fn value(coalesces: &mut Vec<Coalescence>, last_valued: &mut bool,
         value: Coalescence, span: Span) -> Result<(), Spanned<Error>> {
	if *last_valued {
		match coalesces.last() {
			Some(Coalescence::Multiple(_)) => {
				let byte_start = value.byte_start();
				let span = Span(byte_start, byte_start + 1);
				let operator = Spanned::new(Operator::Multiply, span);
				coalesces.push(Coalescence::Operator(operator));
			}
			_ => return Err(Spanned::new(Error::ExpectedOperator, span)),
		}
	}

	*last_valued = true;
	Ok(coalesces.push(value))
}
