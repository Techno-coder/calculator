use std::iter::Peekable;
use std::str::CharIndices;

use crate::error::Error;
use crate::item::{Constant, Function};
use crate::span::{Span, Spanned};
use crate::token::{Operator, Token};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
	string: &'a str,
	characters: Peekable<CharIndices<'a>>,
	last_coalesce: bool,
	byte_end: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(string: &'a str) -> Lexer {
		Lexer {
			string,
			characters: string.char_indices().peekable(),
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
		let mut exponent_divider = false;
		while let Some((index, character)) = self.characters.peek() {
			match character {
				'.' => (),
				'e' => {
					exponent_divider = true;
					self.characters.next();
					continue;
				}
				'-' if exponent_divider => (),
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

	fn take_identifier(&mut self) -> usize {
		while let Some((index, character)) = self.characters.peek() {
			let invalid_character = character.is_whitespace() ||
				character.is_ascii_punctuation();
			match character != &'$' && invalid_character {
				false => self.characters.next(),
				true => return *index,
			};
		}
		self.byte_end
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

		if character.is_digit(10) {
			return Some(self.parse_number(character, byte_start));
		} else if character == ';' {
			let (byte_end, counter) = self.take_coalesce();
			return Some(Ok(Spanned::new(Token::Coalesce(counter),
				Span(byte_start, byte_end))));
		} else if character == '$' {
			let byte_end = self.take_identifier();
			let token = Token::Variable(self.string[byte_start + 1..byte_end].to_owned());
			return Some(Ok(Spanned::new(token, Span(byte_start, byte_end))));
		}

		if !character.is_ascii_punctuation() {
			let byte_end = self.take_identifier();
			let span = Span(byte_start, byte_end);
			let token = Spanned::new(match &self.string[byte_start..byte_end] {
				"sin" => Token::Function(Function::Sine),
				"cos" => Token::Function(Function::Cosine),
				"tan" => Token::Function(Function::Tangent),
				"asin" => Token::Function(Function::InverseSine),
				"acos" => Token::Function(Function::InverseCosine),
				"atan" => Token::Function(Function::InverseTangent),
				"abs" => Token::Function(Function::AbsoluteValue),
				"sqrt" => Token::Function(Function::SquareRoot),
				"cbrt" => Token::Function(Function::CubeRoot),
				"ln" => Token::Function(Function::NaturalLogarithm),
				"log2" => Token::Function(Function::BinaryLogarithm),
				"log10" => Token::Function(Function::DecimalLogarithm),
				"e" => Token::Constant(Constant::E),
				"pi" => Token::Constant(Constant::Pi),
				_ => return Some(Err(Spanned::new(Error::InvalidItem, span))),
			}, span);
			return Some(Ok(token));
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
			'%' => Token::Operator(Operator::Modulo),
			'^' => Token::Operator(Operator::Power),
			_ => return Some(Err(Spanned::new(Error::InvalidCharacter(character), span))),
		}, span);
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
			Token::Operator(Operator::Multiply), Token::Operator(Operator::Minus),
			Token::Terminal(54.0), Token::Coalesce(1), Token::Coalesce(1)]);
	}

	#[test]
	fn test_numerical_format() {
		let string = "10 + -10.0 0x0a 0b1010 0o12 + -1e1";
		let tokens: Result<Vec<_>, _> = Lexer::new(string)
			.map(|token| token.map(|token| token.node)).collect();
		assert_eq!(tokens.unwrap(), &[Token::Terminal(10.0), Token::Operator(Operator::Add),
			Token::Operator(Operator::Minus), Token::Terminal(10.0), Token::Terminal(10.0),
			Token::Terminal(10.0), Token::Terminal(10.0), Token::Operator(Operator::Add),
			Token::Operator(Operator::Minus), Token::Terminal(10.0)]);
	}

	#[test]
	fn test_identifier() {
		let string = "$ $0 $$ $identifier";
		let tokens: Result<Vec<_>, _> = Lexer::new(string)
			.map(|token| token.map(|token| token.node)).collect();
		assert_eq!(tokens.unwrap(), &[Token::Variable("".to_owned()),
			Token::Variable("0".to_owned()), Token::Variable("$".to_owned()),
			Token::Variable("identifier".to_owned())]);
	}
}
