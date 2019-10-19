use crate::error::Error;
use crate::span::Spanned;
use crate::token::Operator;

#[derive(Debug)]
pub enum Node {
	Terminal(Spanned<f64>),
	Operator(Spanned<Operator>, Box<Node>, Box<Node>),
}

impl Node {
	pub fn evaluate(&self) -> Result<f64, Error> {
		Ok(match self {
			Node::Terminal(terminal) => terminal.node,
			Node::Operator(operator, left, right) => {
				let left = left.evaluate()?;
				let right = right.evaluate()?;
				match operator.node {
					Operator::Add => left + right,
					Operator::Minus => left - right,
					Operator::Multiply => left * right,
					Operator::Divide => {
						if right != 0.0 {
							left / right
						} else {
							return Err(Error::ZeroDivision);
						}
					}
				}
			}
		})
	}
}
