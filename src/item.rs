#[derive(Debug, PartialEq)]
pub enum Function {
	Trigonometric(Trigonometric, AngleUnit),
	AbsoluteValue,
	SquareRoot,
	CubeRoot,
	NaturalLogarithm,
	BinaryLogarithm,
	DecimalLogarithm,
	UnaryMinus,
}

#[derive(Debug, PartialEq)]
pub enum Trigonometric {
	Sine,
	Cosine,
	Tangent,
	InverseSine,
	InverseCosine,
	InverseTangent,
}

#[derive(Debug, PartialEq)]
pub enum AngleUnit {
	Radians,
	Degrees,
}

impl AngleUnit {
	pub fn apply(&self, radians: f64) -> f64 {
		match self {
			AngleUnit::Radians => radians,
			AngleUnit::Degrees => radians.to_degrees(),
		}
	}

	pub fn radians(&self, value: f64) -> f64 {
		match self {
			AngleUnit::Radians => value,
			AngleUnit::Degrees => value.to_radians(),
		}
	}
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
