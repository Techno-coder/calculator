#[derive(Debug, PartialEq)]
pub enum Function {
	Sine,
	Cosine,
	Tangent,
	InverseSine,
	InverseCosine,
	InverseTangent,
	AbsoluteValue,
	SquareRoot,
	CubeRoot,
	NaturalLogarithm,
	BinaryLogarithm,
	DecimalLogarithm,
}

#[derive(Debug, PartialEq)]
pub enum Constant {
	E,
	Pi,
}

impl Constant {
	pub fn value(&self) -> f64 {
		use std::f64::consts;
		match self {
			Constant::E => consts::E,
			Constant::Pi => consts::PI,
		}
	}
}
