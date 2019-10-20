use crate::coalescence::Coalescence;
use crate::error::Error;
use crate::lexer::Lexer;
use crate::span::{Span, Spanned};
use crate::token::Token;

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
			Token::Operator(operator) => match last_valued {
				true => {
					coalesces.push(Coalescence::Operator(token.map(operator)));
					last_valued = false;
				}
				false => return Err(token.map(Error::ExpectedValued)),
			},
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
			Token::Coalesce(count) => {
				let coalesce_length = 1 + count * 2;
				if coalesce_length > coalesces.len() {
					return Err(token.map(Error::InvalidCoalesce));
				}

				let coalesce_start = coalesces.len() - coalesce_length;
				let coalescence = coalesces.split_off(coalesce_start);
				coalesces.push(Coalescence::Multiple(coalescence));
			}
		}
	}

	let last_span = Span(last_byte_end, last_byte_end + 1);
	match last_valued {
		true => Ok(Coalescence::Multiple(coalesces)),
		false => Err(Spanned::new(Error::ExpectedValued, last_span))
	}
}
