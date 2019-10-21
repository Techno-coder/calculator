use crate::context::Context;
use crate::error::Error;
use crate::span::Spanned;
use crate::token::Operator;

#[derive(Debug)]
pub enum Coalescence {
	Multiple(Vec<Coalescence>),
	Operator(Spanned<Operator>),
	Terminal(Spanned<f64>),
	Variable(Spanned<String>),
}

impl Coalescence {
	pub fn verify(&self, context: &Context) -> Result<(), Spanned<Error>> {
		match self {
			Coalescence::Multiple(coalesces) => coalesces.iter()
				.try_for_each(|coalesce| coalesce.verify(context)),
			Coalescence::Variable(variable) => context.variable(&variable.node)
				.map_err(|error| Spanned::new(error, variable.span)).map(|_| ()),
			_ => Ok(()),
		}
	}

	pub fn coalesce_anchors(&self) -> Vec<usize> {
		match self {
			Coalescence::Multiple(coalesces) => coalesces.iter().step_by(2)
				.map(|coalescence| coalescence.byte_start()).collect(),
			Coalescence::Terminal(_) => vec![self.byte_start()],
			Coalescence::Variable(_) => vec![self.byte_start()],
			Coalescence::Operator(_) => vec![],
		}
	}

	fn byte_start(&self) -> usize {
		match self {
			Coalescence::Multiple(coalesces) => coalesces.first().unwrap().byte_start(),
			Coalescence::Operator(operator) => operator.span.byte_start(),
			Coalescence::Terminal(terminal) => terminal.span.byte_start(),
			Coalescence::Variable(variable) => variable.span.byte_start(),
		}
	}

	pub fn byte_end(&self) -> usize {
		match self {
			Coalescence::Multiple(coalesces) => coalesces.last().unwrap().byte_end(),
			Coalescence::Operator(operator) => operator.span.byte_end(),
			Coalescence::Terminal(terminal) => terminal.span.byte_end(),
			Coalescence::Variable(variable) => variable.span.byte_end(),
		}
	}
}
