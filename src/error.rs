use std::fmt;

#[derive(Debug)]
pub enum Error {
	UndefinedVariable(String),
	InvalidCharacter(char),
	InvalidEvaluationOffset,
	InvalidTerminal,
	InvalidItem,
	ExpectedValued,
	ExpectedOperator,
	MismatchedBracket,
	EmptyBrackets,
	InvalidCoalesce,
	ZeroDivision,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Error::UndefinedVariable(variable) =>
				write!(f, "Undefined variable: {}", variable),
			Error::InvalidCharacter(character) =>
				write!(f, "Invalid input character: '{}'", character),
			Error::InvalidEvaluationOffset =>
				write!(f, "Invalid evaluation offset"),
			Error::InvalidTerminal =>
				write!(f, "Invalid number"),
			Error::InvalidItem =>
				write!(f, "Invalid function or constant"),
			Error::ExpectedValued =>
				write!(f, "Expected a number, variable or constant"),
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
