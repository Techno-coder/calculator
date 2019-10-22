use crate::context::Context;
use crate::error::Error;

#[test]
fn test_arithmetic() {
	assert_eq!(evaluate("1 + 2"), Ok(3.0));
	assert_eq!(evaluate("1 - 2"), Ok(-1.0));
	assert_eq!(evaluate("1 * 2"), Ok(2.0));
	assert_eq!(evaluate("1 / 2"), Ok(0.5));
	assert_eq!(evaluate("10 ^ -2"), Ok(0.01));
}

#[test]
fn test_unary_minus() {
	assert_eq!(evaluate("-1"), Ok(-1.0));
	assert_eq!(evaluate("1 - -1"), Ok(2.0));
	assert_eq!(evaluate("1 --- 1"), Ok(0.0));
	assert_eq!(evaluate("-1 -- -1"), Ok(-2.0));
}

#[test]
fn test_functions() {
	assert_eq!(evaluate("abs -1"), Ok(1.0));
	assert_eq!(evaluate("sin asin 1"), Ok(1.0));
	assert_eq!(evaluate("sin' asin' 1"), Ok(1.0));
	assert_eq!(evaluate("sqrt -1 + 1;;"), Ok(0.0));
}

fn evaluate(expression: &str) -> Result<f64, Error> {
	let mut context = &mut Context::default();
	context.expression = expression.to_owned();
	crate::interface::evaluate(context)
		.map_err(|error| error.node)
}
