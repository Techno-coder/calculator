use crate::coalescence::Coalescence;
use crate::item::Function;
use crate::node::Node;
use crate::span::{Span, Spanned};
use crate::token::Operator;

#[derive(Debug)]
enum ParserOperator {
	Operator(Spanned<Operator>),
	Function(Spanned<Function>),
}

impl ParserOperator {
	pub fn precedence(&self) -> usize {
		match self {
			ParserOperator::Operator(operator) => operator.node.precedence(),
			ParserOperator::Function(_) => usize::max_value(),
		}
	}
}

pub fn parse_root(coalescence: Coalescence) -> Spanned<Node> {
	let nodes = &mut Vec::new();
	parse(coalescence, &mut Vec::new(), 0, nodes);
	assert_eq!(nodes.len(), 1);
	nodes.pop().unwrap()
}

fn parse(coalescence: Coalescence, operators: &mut Vec<ParserOperator>,
         state: usize, nodes: &mut Vec<Spanned<Node>>) {
	match coalescence {
		Coalescence::Terminal(terminal) =>
			nodes.push(Spanned::new(Node::Terminal(terminal.node), terminal.span)),
		Coalescence::Variable(variable) =>
			nodes.push(Spanned::new(Node::Variable(variable.node), variable.span)),
		Coalescence::Function(function) =>
			operators.push(ParserOperator::Function(function)),
		Coalescence::Operator(operator) => {
			while let Some(stack_operator) = operators.last() {
				match stack_operator.precedence() >= operator.node.precedence() {
					true if operators.len() > state => construct(operators, nodes),
					_ => break,
				}
			}
			operators.push(ParserOperator::Operator(operator));
		}
		Coalescence::Multiple(coalesces) => {
			let operator_state = operators.len();
			coalesces.into_iter().for_each(|coalescence|
				parse(coalescence, operators, operator_state, nodes));

			assert!(operators.len() >= operator_state);
			while operators.len() > operator_state {
				construct(operators, nodes);
			}
		}
	}
}

fn construct(operators: &mut Vec<ParserOperator>, nodes: &mut Vec<Spanned<Node>>) {
	let operator = operators.pop().unwrap();
	match operator {
		ParserOperator::Operator(operator) => {
			let right = nodes.pop().unwrap();
			let left = nodes.pop().unwrap();
			let span = Span(left.span.byte_start(), right.span.byte_end());
			let node = Node::Operator(operator, Box::new(left), Box::new(right));
			nodes.push(Spanned::new(node, span))
		}
		ParserOperator::Function(function) => {
			let node = nodes.pop().unwrap();
			let span = Span(function.span.byte_start(), node.span.byte_end());
			let node = Node::Function(function.node, Box::new(node));
			nodes.push(Spanned::new(node, span))
		}
	}
}
