use crate::expression::{Expression, SubIn};
use std::convert::Into;

impl Into<Expression> for String {
	fn into(self) -> Expression {
		Expression::Variable(self)
	}
}
impl Into<Expression> for &str {
	fn into(self) -> Expression {
		Expression::Variable(self.to_string())
	}
}

impl SubIn for String {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		if *self == var.to_string() {
			val.clone()
		} else {
			Expression::Variable(self.to_string())
		}
	}
}
