use crate::expression::{Expression, SubIn};

use std::fmt;

#[derive(Debug, Clone)]
pub struct Brackets {
	pub expression: Box<Expression>,
}

impl fmt::Display for Brackets {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "\\left( {} \\right)", self.expression);
	}
}

impl SubIn for Brackets {
	fn sub_in(&self, variable: &str, value: &Expression) -> Expression {
		self.expression.sub_in(variable, value)
	}
}
