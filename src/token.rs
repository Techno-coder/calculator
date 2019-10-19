use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
	Terminal(f64),
	Operator(Operator),
	ParenthesisOpen,
	ParenthesisClose,
	Coalesce(usize),
}

impl Token {
	pub fn is_operator(&self) -> bool {
		match self {
			Token::Operator(_) => true,
			_ => false,
		}
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
	Add,
	Minus,
	Multiply,
	Divide,
}

impl Operator {
	pub fn precedence(&self) -> usize {
		match self {
			Operator::Add | Operator::Minus => 0,
			Operator::Multiply | Operator::Divide => 1,
		}
	}
}

impl fmt::Display for Operator {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Operator::Add => "+",
			Operator::Minus => "-",
			Operator::Multiply => "*",
			Operator::Divide => "/",
		})
	}
}
