use crate::coalescence::Coalescence;
use crate::error::Error;
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
		last_byte_end = token.span.byte_end();
		match token.node {
			Token::ParenthesisClose if expect_parenthesis => match coalesces.is_empty() {
				true => return Err(token.map(Error::EmptyBrackets)),
				false => break,
			},
			Token::ParenthesisClose => return Err(token.map(Error::MismatchedBracket)),
			Token::ParenthesisOpen => {
				coalesces.push(coalesce(lexer, false, true)?);
				last_valued = true;
			}
			Token::Operator(operator) => {
				if !last_valued {
					coalesces.push(match operator {
						Operator::Minus => Coalescence::Terminal(Spanned::new(0.0, token.span)),
						_ => return Err(token.map(Error::ExpectedValued)),
					});
				}

				coalesces.push(Coalescence::Operator(token.map(operator)));
				last_valued = false;
			}
			Token::Terminal(terminal) => match last_valued {
				false => {
					coalesces.push(Coalescence::Terminal(token.map(terminal)));
					last_valued = true;
				}
				true => return Err(token.map(Error::ExpectedOperator)),
			},
			Token::Variable(variable) => match last_valued {
				false => {
					coalesces.push(Coalescence::Variable(Spanned::new(variable, token.span)));
					last_valued = true;
				}
				true => return Err(Spanned::new(Error::ExpectedOperator, token.span)),
			},
			Token::Function(function) => match last_valued {
				false => coalesces.push(Coalescence::Function(Spanned::new(function, token.span))),
				true => return Err(Spanned::new(Error::ExpectedOperator, token.span)),
			},
			Token::Constant(constant) => match last_valued {
				false => {
					let terminal = Spanned::new(constant.value(), token.span);
					coalesces.push(Coalescence::Terminal(terminal));
					last_valued = true;
				}
				true => return Err(Spanned::new(Error::ExpectedOperator, token.span)),
			},
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
