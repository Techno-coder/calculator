use std::iter::Peekable;
use std::str::CharIndices;

use crate::error::Error;
use crate::span::{Span, Spanned};
use crate::token::{Operator, Token};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
	string: &'a str,
	characters: Peekable<CharIndices<'a>>,
	last_operator: bool,
	last_coalesce: bool,
	byte_end: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(string: &'a str) -> Lexer {
		Lexer {
			string,
			characters: string.char_indices().peekable(),
			last_operator: true,
			last_coalesce: false,
			byte_end: string.len(),
		}
	}

	fn skip_whitespace(&mut self) {
		while let Some((_, character)) = self.characters.peek() {
			match character.is_whitespace() {
				true => self.characters.next(),
				false => return,
			};
		}
	}

	fn number(&mut self) -> usize {
		while let Some((index, character)) = self.characters.peek() {
			match character {
				'.' | 'e' => (),
				_ if character.is_digit(16) => (),
				_ => return *index,
			};
			self.characters.next();
		}
		self.byte_end
	}

	fn parse_number(&mut self, character: char, byte_start: usize)
	                -> Result<Spanned<Token>, Spanned<Error>> {
		let radix = match character {
			'0' => match self.characters.peek() {
				Some((_, prefix)) if prefix == &'x' => 16,
				Some((_, prefix)) if prefix == &'o' => 8,
				Some((_, prefix)) if prefix == &'b' => 2,
				_ => 10,
			}
			_ => 10,
		};

		let number_start = match radix {
			10 => byte_start,
			_ => {
				self.characters.next();
				self.characters.next().map(|(index, _)| index)
					.unwrap_or(self.byte_end)
			}
		};

		let byte_end = self.number();
		let slice = &self.string[number_start..byte_end];
		let span = Span(byte_start, byte_end);

		let error = Spanned::new(Error::InvalidTerminal, span);
		match radix {
			10 => slice.parse::<f64>().map_err(|_| error),
			_ => i64::from_str_radix(slice, radix).map_err(|_| error)
				.map(|terminal| terminal as f64),
		}.map(|terminal| Spanned::new(Token::Terminal(terminal), span))
	}

	fn take_coalesce(&mut self) -> (usize, usize) {
		let mut counter = 1;
		while let Some((index, character)) = self.characters.peek() {
			match character == &';' {
				true => self.characters.next(),
				false => return (*index, counter),
			};
			counter += 1;
		}
		(self.byte_end, counter)
	}
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Result<Spanned<Token>, Spanned<Error>>;

	fn next(&mut self) -> Option<Self::Item> {
		let (byte_start, character) = self.characters.next()?;
		if character.is_whitespace() {
			self.skip_whitespace();
			return self.next();
		}

		let is_negation = character == '-' && self.last_operator;
		self.last_operator = false;

		if character.is_digit(10) || is_negation {
			return Some(self.parse_number(character, byte_start));
		} else if character == ';' {
			let (byte_end, counter) = self.take_coalesce();
			return Some(Ok(Spanned::new(Token::Coalesce(counter),
				Span(byte_start, byte_end))));
		}

		let span = Span(byte_start, self.characters.peek()
			.map(|(index, _)| *index).unwrap_or(self.byte_end));
		let token = Spanned::new(match character {
			'(' => Token::ParenthesisOpen,
			')' => Token::ParenthesisClose,
			'+' => Token::Operator(Operator::Add),
			'-' => Token::Operator(Operator::Minus),
			'*' => Token::Operator(Operator::Multiply),
			'/' => Token::Operator(Operator::Divide),
			_ => return Some(Err(Spanned::new(Error::InvalidCharacter(character), span))),
		}, span);

		self.last_operator = token.node.is_operator();
		Some(Ok(token))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tokenize() {
		let string = "(1 + 2) / 3 * -54; ;";
		let tokens: Result<Vec<_>, _> = Lexer::new(string)
			.map(|token| token.map(|token| token.node)).collect();
		assert_eq!(tokens.unwrap(), &[Token::ParenthesisOpen,
			Token::Terminal(1.0), Token::Operator(Operator::Add), Token::Terminal(2.0),
			Token::ParenthesisClose, Token::Operator(Operator::Divide), Token::Terminal(3.0),
			Token::Operator(Operator::Multiply), Token::Terminal(-54.0), Token::Coalesce(1),
			Token::Coalesce(1)]);
	}

	#[test]
	fn test_numerical_format() {
		let string = "10 + -10.0 0x0a 0b1010 0o12 + -1e1";
		let tokens: Result<Vec<_>, _> = Lexer::new(string)
			.map(|token| token.map(|token| token.node)).collect();
		assert_eq!(tokens.unwrap(), &[Token::Terminal(10.0),
			Token::Operator(Operator::Add), Token::Terminal(-10.0), Token::Terminal(10.0),
			Token::Terminal(10.0), Token::Terminal(10.0), Token::Operator(Operator::Add),
			Token::Terminal(-10.0)]);
	}
}
