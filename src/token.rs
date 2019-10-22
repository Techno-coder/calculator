use crate::item::{Constant, Function};

#[derive(Debug, PartialEq)]
pub enum Token {
	Terminal(f64),
	Variable(String),
	Operator(Operator),
	Function(Function),
	Constant(Constant),
	ParenthesisOpen,
	ParenthesisClose,
	Coalesce(usize),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
	Add,
	Minus,
	Multiply,
	Divide,
	Modulo,
	Power,
}

impl Operator {
	pub fn precedence(&self) -> usize {
		match self {
			Operator::Add | Operator::Minus => 0,
			Operator::Multiply | Operator::Divide | Operator::Modulo => 1,
			Operator::Power => 2,
		}
	}
}
