use crate::coalescence::Coalescence;
use crate::node::Node;
use crate::span::Spanned;
use crate::token::Operator;

pub fn parse_root(coalescence: Coalescence) -> Node {
	let nodes = &mut Vec::new();
	parse(coalescence, &mut Vec::new(), 0, nodes);
	assert_eq!(nodes.len(), 1);
	nodes.pop().unwrap()
}

fn parse(coalescence: Coalescence, operators: &mut Vec<Spanned<Operator>>,
         state: usize, nodes: &mut Vec<Node>) {
	match coalescence {
		Coalescence::Terminal(terminal) => nodes.push(Node::Terminal(terminal)),
		Coalescence::Operator(operator) => {
			while let Some(stack_operator) = operators.last() {
				match stack_operator.node.precedence() > operator.node.precedence() {
					true if operators.len() > state => construct(operators, nodes),
					_ => break,
				}
			}
			operators.push(operator);
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

fn construct(operators: &mut Vec<Spanned<Operator>>, nodes: &mut Vec<Node>) {
	let operator = operators.pop().unwrap();
	let right = nodes.pop().unwrap_or_else(|| panic!("Node stack is empty"));
	let left = nodes.pop().unwrap_or_else(|| panic!("Node stack is empty"));
	nodes.push(Node::Operator(operator, Box::new(left), Box::new(right)));
}
