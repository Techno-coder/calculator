use crate::context::Context;
use crate::error::Error;
use crate::span::Spanned;
use crate::token::Operator;

#[derive(Debug)]
pub enum Node {
	Terminal(f64),
	Variable(String),
	Operator(Spanned<Operator>, Box<Spanned<Node>>, Box<Spanned<Node>>),
}

impl Spanned<Node> {
	pub fn evaluate(&self, context: &Context) -> Result<f64, Spanned<Error>> {
		Ok(match &self.node {
			Node::Terminal(terminal) => *terminal,
			Node::Variable(variable) => context.variable(variable)
				.map_err(|error| Spanned::new(error, self.span))?,
			Node::Operator(operator, left_node, right_node) => {
				let left = left_node.evaluate(context)?;
				let right = right_node.evaluate(context)?;
				match operator.node {
					Operator::Add => left + right,
					Operator::Minus => left - right,
					Operator::Multiply => left * right,
					Operator::Divide => {
						if right != 0.0 {
							left / right
						} else {
							return Err(Spanned::new(Error::ZeroDivision, right_node.span));
						}
					}
					Operator::Modulo => left % right,
					Operator::Power => left.powf(right),
				}
			}
		})
	}
}
