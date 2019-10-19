use std::fmt;

#[derive(Debug)]
pub enum Error {
	InvalidCharacter(char),
	InvalidTerminal,
	ExpectedTerminal,
	ExpectedOperator,
	MismatchedBracket,
	EmptyBrackets,
	InvalidCoalesce,
	ZeroDivision,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::InvalidCharacter(character) =>
				write!(f, "Invalid input character: '{}'", character),
			Error::InvalidTerminal =>
				write!(f, "Invalid number"),
			Error::ExpectedTerminal =>
				write!(f, "Expected a number"),
			Error::ExpectedOperator =>
				write!(f, "Expected an operator"),
			Error::MismatchedBracket =>
				write!(f, "Bracket has no matching pair"),
			Error::EmptyBrackets =>
				write!(f, "Bracket pair is empty"),
			Error::InvalidCoalesce =>
				write!(f, "Invalid coalesce"),
			Error::ZeroDivision =>
				write!(f, "Division by zero"),
		}
	}
}
