use crate::expression::numeral::gcd::gcd;
use crate::expression::{Expression, SubIn};
use std::convert::{From, Into};
use std::fmt;
use std::ops::{Add, Mul};
mod gcd;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn mixed_fraction() {
		// Sec 1a, Page 61, Q8a
		let exp = sum!(prod!(3, "y"), prod!(-2, "x"));
		let exp = exp.sub_in("x", &(-5).into());
		let exp = exp.sub_in("y", &Fraction::new(1, 4).into());
		if let Expression::Numeral(f) = exp {
			assert_eq!(f.to_mixed_fraction(), "10\\frac{3}{4}");
		} else {
			panic!("Expected fraction after subbing in");
		}
	}
}

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
		if denominator == 0 {
			panic!("Denominator cannot be zero");
		}
		let gcd = gcd(numerator, denominator);
		let numerator = numerator / gcd;
		let denominator = denominator / gcd;
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

	pub fn is_integer(&self) -> bool {
		self.denominator == 1
	}

	pub fn is_positive(&self) -> bool {
		self.numerator > 0
	}

	pub fn is_zero(&self) -> bool {
		self.numerator == 0
	}

	pub fn is_negative(&self) -> bool {
		self.numerator < 0
	}

	pub fn is_one(&self) -> bool {
		self.numerator == 1 && self.denominator == 1
	}

	pub fn negative(&self) -> Fraction {
		Fraction::new(-self.numerator, self.denominator as i32)
	}

	pub fn reciprocal(&self) -> Fraction {
		Fraction::new(self.denominator.clone() as i32, self.numerator.clone())
	}

	pub fn pow(&self, power: i32) -> Fraction {
		if power == 0 {
			return Fraction::from(1);
		} else if power > 0 {
			return Fraction::new(
				self.numerator.pow(power as u32),
				self.denominator.pow(power as u32) as i32,
			);
		} else {
			let power = power.abs() as u32;
			return Fraction::new(
				self.denominator.pow(power) as i32,
				self.numerator.pow(power),
			);
		}
	}

	pub fn to_mixed_fraction(&self) -> String {
		if self.is_integer() {
			return format!("{}", self.numerator);
		}
		let sign = if self.numerator < 0 { "-" } else { "" };
		let numerator = self.numerator.abs() as u32;
		let whole = numerator / self.denominator;
		let numerator = numerator % self.denominator;
		let denominator = self.denominator;
		if whole == 0 {
			return format!("{}\\frac{{{}}}{{{}}}", sign, numerator, denominator);
		}
		format!(
			"{}{}\\frac{{{}}}{{{}}}",
			sign, whole, numerator, denominator
		)
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
