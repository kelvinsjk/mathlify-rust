use crate::expression::{Expression, SubIn};
use std::convert::{From, Into};
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
	pub numerator: i32,
	pub denominator: u32,
}

// display
impl fmt::Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.denominator == 1 {
			return write!(f, "{}", self.numerator);
		}
		if self.numerator == 0 {
			return write!(f, "0");
		}
		let sign = if self.numerator < 0 { "-" } else { "" };
		write!(
			f,
			"{}\\frac{{{}}}{{{}}}",
			sign,
			self.numerator.abs(),
			self.denominator
		)
	}
}

// constructor and methods
impl Fraction {
	pub fn new(numerator: i32, denominator: i32) -> Fraction {
		//TODO: GCD simplification
		if denominator == 0 {
			panic!("Denominator cannot be zero");
		}
		Fraction {
			numerator: numerator * denominator.signum(),
			denominator: denominator.abs() as u32,
		}
	}

	pub fn abs(&self) -> Fraction {
		Fraction {
			numerator: self.numerator.abs(),
			denominator: self.denominator,
		}
	}
}

// built in operators
impl Mul<Fraction> for Fraction {
	type Output = Fraction;
	fn mul(self, rhs: Fraction) -> Fraction {
		Fraction::new(
			self.numerator * rhs.numerator,
			self.denominator as i32 * rhs.denominator as i32,
		)
	}
}

impl Add<Fraction> for Fraction {
	type Output = Fraction;
	fn add(self, rhs: Fraction) -> Fraction {
		Fraction::new(
			self.numerator * rhs.denominator as i32 + rhs.numerator * self.denominator as i32,
			self.denominator as i32 * rhs.denominator as i32,
		)
	}
}

impl PartialEq for Fraction {
	fn eq(&self, other: &Self) -> bool {
		self.numerator * other.denominator as i32 == self.denominator as i32 * other.numerator
	}
}

// from and into

impl<'a> Into<Expression<'a>> for i32 {
	fn into(self) -> Expression<'a> {
		Expression::Numeral(Fraction::from(self as i32))
	}
}

impl From<i32> for Fraction {
	fn from(numerator: i32) -> Self {
		Fraction::new(numerator, 1)
	}
}

impl<'a> Into<Expression<'a>> for Fraction {
	fn into(self) -> Expression<'a> {
		Expression::Numeral(self)
	}
}

// custom traits
impl SubIn for Fraction {
	fn sub_in<'a>(&self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		let _ = var;
		let _ = val;
		Expression::Numeral(*self)
	}
}
