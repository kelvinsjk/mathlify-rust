use crate::expression::{Expression, SubIn};
use std::convert::Into;
use std::fmt;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn exp_display() {
		// Sec 1a, Page 60, Q7d
		assert_eq!(sum!(exp!("x", 2), exp!("y", 2)).to_string(), "x^2 + y^2");
		// Sec 1a Page 61, Q10b, d
		let exp = exp!(sum!(exp!("x", 2), prod!(-1, "y", "z")), 3);
		assert_eq!(exp.to_string(), "\\left( x^2 - yz \\right)^3");
		let exp = prod!(
			sum!("x", "z"),
			exp!(sum!(exp!("z", 2), prod!(-1, "x", "z"), "z"), 2)
		);
		assert_eq!(
			exp.to_string(),
			"\\left( x + z \\right)\\left( z^2 - xz + z \\right)^2"
		);
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q7d
		let exp = sum!(exp!("x", 2), exp!("y", 2));
		let exp = exp.sub_in("x", &(-4).into());
		let exp = exp.sub_in("y", &(7).into());
		assert_eq!(exp.to_string(), "65");
		// Sec 1a Page 61, Q10b, d
		let exp = exp!(sum!(exp!("x", 2), prod!(-1, "y", "z")), 3);
		let exp = exp.sub_in("x", &Fraction::new(-1, 2).into());
		let exp = exp.sub_in("y", &0.into());
		let exp = exp.sub_in("z", &4.into());
		assert_eq!(exp.to_string(), "\\frac{1}{64}");
		let exp = prod!(
			sum!("x", "z"),
			exp!(sum!(exp!("z", 2), prod!(-1, "x", "z"), "z"), 2)
		);
		let exp = exp.sub_in("x", &Fraction::new(-1, 2).into());
		let exp = exp.sub_in("y", &0.into());
		let exp = exp.sub_in("z", &4.into());
		assert_eq!(exp.to_string(), "1694");
	}
}

#[derive(Debug, Clone)]
pub struct Exponent {
	pub base: Box<Expression>,
	pub exponent: Box<Expression>,
}

impl fmt::Display for Exponent {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let brackets = match self.base.as_ref() {
			Expression::Sum(s) => s.terms.len() > 1,
			Expression::Product(p) => {
				p.factors.len() > 1 || (p.coefficient != 1.into() && p.factors.len() == 1)
			}
			_ => false,
		};
		// handle brackets for base
		let base: String;
		if brackets {
			base = format!("\\left( {} \\right)", self.base);
		} else {
			base = format!("{}", self.base);
		}
		// handle braces for exponent
		let mut exponent = self.exponent.to_string();
		if exponent == "1" {
			exponent = "".to_string();
		} else if exponent.len() > 1 {
			exponent = format!("^{{{}}}", exponent);
		} else {
			exponent = format!("^{}", exponent);
		}
		write!(f, "{}{}", base, exponent)
	}
}

impl SubIn for Exponent {
	fn sub_in(&self, var: &str, exp: &Expression) -> Expression {
		let e = Exponent {
			base: Box::new(self.base.sub_in(var, exp)),
			exponent: Box::new(self.exponent.sub_in(var, exp)),
		};
		let mut exp = Expression::Exponent(e);
		exp.simplify();
		exp
	}
}

impl Into<Expression> for Exponent {
	fn into(self) -> Expression {
		Expression::Exponent(self)
	}
}
