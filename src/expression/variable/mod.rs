use crate::expression::{Expression, SubIn};
use std::convert::Into;

impl<'a> Into<Expression<'a>> for &'a str {
	fn into(self) -> Expression<'a> {
		Expression::Variable(self)
	}
}

impl SubIn for &str {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		if *self == var {
			val.clone()
		} else {
			(*self).into()
		}
	}
}
