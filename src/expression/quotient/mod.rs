use crate::expression::{Expression, SubIn};
use std::fmt;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn display() {
		assert_eq!(prod!().to_string(), "1");
		assert_eq!(prod!(0, "x").to_string(), "0");
		assert_eq!(
			prod_verbatim!(2, "x", Fraction::new(1, 2), "y").to_string(),
			"2x\\frac{1}{2}y"
		);
		assert_eq!(
			prod!(Fraction::new(1, 3), "x", "y").to_string(),
			"\\frac{1}{3}xy"
		);
		// Sec 1a, Page 60, Q6d
		assert_eq!(quotient!(prod!("x", "y"), 3).to_string(), "\\frac{xy}{3}");
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = prod!(3, "x", "y");
		let exp = exp.sub_in("x", &(5 as i32).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "-30");
	}
}

#[derive(Debug, Clone)]
pub struct Quotient {
	pub numerator: Box<Expression>,
	pub denominator: Box<Expression>,
}

impl fmt::Display for Quotient {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Expression::Numeral(n) = self.denominator.as_ref() {
			if n.is_one() {
				return write!(f, "{}", self.numerator);
			}
		}
		write!(f, "\\frac{{{}}}{{{}}}", self.numerator, self.denominator)
	}
}

impl SubIn for Quotient {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		let mut e = Expression::Quotient(Quotient {
			numerator: Box::new(self.numerator.sub_in(var, val)),
			denominator: Box::new(self.denominator.sub_in(var, val)),
		});
		e.simplify();
		e
	}
}

// only works for numeral numerator
impl Quotient {
	pub fn abs(&self) -> Quotient {
		if let Expression::Numeral(n) = self.numerator.as_ref() {
			if n.is_negative() {
				return Quotient {
					numerator: Box::new(n.abs().into()),
					denominator: Box::new(*self.denominator.clone()),
				};
			}
		}
		self.clone()
	}
}
