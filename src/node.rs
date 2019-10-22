use crate::context::Context;
use crate::error::Error;
use crate::item::Function;
use crate::span::Spanned;
use crate::token::Operator;

#[derive(Debug)]
pub enum Node {
	Terminal(f64),
	Variable(String),
	Function(Function, Box<Spanned<Node>>),
	Operator(Spanned<Operator>, Box<Spanned<Node>>, Box<Spanned<Node>>),
}

impl Spanned<Node> {
	pub fn evaluate(&self, context: &Context) -> Result<f64, Spanned<Error>> {
		Ok(match &self.node {
			Node::Terminal(terminal) => *terminal,
			Node::Variable(variable) => context.variable(variable)
				.map_err(|error| Spanned::new(error, self.span))?,
			Node::Function(function, node) => {
				let value = node.evaluate(context)?;
				match function {
					Function::Sine => value.sin(),
					Function::Cosine => value.cos(),
					Function::Tangent => value.tan(),
					Function::InverseSine => value.asin(),
					Function::InverseCosine => value.acos(),
					Function::InverseTangent => value.atan(),
					Function::AbsoluteValue => value.abs(),
					Function::SquareRoot => match value >= 0.0 {
						false => return Err(Spanned::new(Error::NegativeRoot, node.span)),
						true => value.sqrt(),
					},
					Function::CubeRoot => match value >= 0.0 {
						false => return Err(Spanned::new(Error::NegativeRoot, node.span)),
						true => value.cbrt(),
					},
					Function::NaturalLogarithm => value.ln(),
					Function::BinaryLogarithm => value.log2(),
					Function::DecimalLogarithm => value.log10(),
					Function::UnaryMinus => -value,
				}
			}
			Node::Operator(operator, left_node, right_node) => {
				let left = left_node.evaluate(context)?;
				let right = right_node.evaluate(context)?;
				match operator.node {
					Operator::Add => left + right,
					Operator::Minus => left - right,
					Operator::Multiply => left * right,
					Operator::Divide => match right != 0.0 {
						false => return Err(Spanned::new(Error::ZeroDivision, right_node.span)),
						true => left / right
					}
					Operator::Modulo => left % right,
					Operator::Power => left.powf(right),
				}
			}
		})
	}
}
