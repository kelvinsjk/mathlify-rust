use crate::expression::numeral::gcd::gcd;
use crate::expression::{Expression, SubIn};
use std::convert::{From, Into};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
pub mod fraction_gcd;
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

	#[test]
	fn ordering() {
		let a = Fraction::new(1, 2);
		let b = Fraction::new(1, 3);
		assert!(a > b);
		let m = cmp::min(b, a);
		println!("{}, {}, {}", a > b, a < b, a == b);
		assert_eq!(m.to_string(), "\\frac{1}{3}");
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
			if self.numerator < 0 {
				return write!(f, "- {}", self.numerator.abs());
			}
			return write!(f, "{}", self.numerator);
		}
		if self.numerator == 0 {
			return write!(f, "0");
		}
		let sign = if self.numerator < 0 { "- " } else { "" };
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

	pub fn is_nonzero(&self) -> bool {
		!self.is_zero()
	}

	pub fn is_negative(&self) -> bool {
		self.numerator < 0
	}

	pub fn is_nonnegative(&self) -> bool {
		!self.is_negative()
	}

	pub fn is_one(&self) -> bool {
		self.numerator == 1 && self.denominator == 1
	}

	pub fn is_negative_one(&self) -> bool {
		self.numerator == -1 && self.denominator == 1
	}

	pub fn negative(&self) -> Fraction {
		Fraction::new(-self.numerator, self.denominator as i32)
	}

	pub fn reciprocal(&self) -> Fraction {
		if &self.denominator == &0 {
			panic!("Denominator cannot be zero");
		}
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
		let sign = if self.numerator < 0 { "- " } else { "" };
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

impl Div<Fraction> for Fraction {
	type Output = Fraction;
	fn div(self, rhs: Fraction) -> Fraction {
		if rhs == Fraction::from(0) {
			panic!("Cannot divide by zero");
		}
		Fraction::new(
			self.numerator * rhs.denominator as i32,
			self.denominator as i32 * rhs.numerator,
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

impl Sub<Fraction> for Fraction {
	type Output = Fraction;
	fn sub(self, rhs: Fraction) -> Fraction {
		self + rhs.negative()
	}
}

impl PartialEq for Fraction {
	fn eq(&self, other: &Self) -> bool {
		self.numerator * other.denominator as i32 == self.denominator as i32 * other.numerator
	}
}

impl Eq for Fraction {}

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let lhs: f32 = self.numerator as f32 / self.denominator as f32;
		let rhs: f32 = other.numerator as f32 / other.denominator as f32;
		lhs.partial_cmp(&rhs)
	}
}

impl Ord for Fraction {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let lhs: f32 = self.numerator as f32 / self.denominator as f32;
		let rhs: f32 = other.numerator as f32 / other.denominator as f32;
		lhs.partial_cmp(&rhs).unwrap()
	}
	fn max(self, other: Self) -> Self {
		if self > other {
			self
		} else {
			other
		}
	}
	fn min(self, other: Self) -> Self {
		if self < other {
			self
		} else {
			other
		}
	}
	fn clamp(self, min: Self, max: Self) -> Self
	where
		Self: Sized,
		Self: PartialOrd,
	{
		if self < min {
			min
		} else if self > max {
			max
		} else {
			self
		}
	}
}

// from and into

impl Into<Expression> for i32 {
	fn into(self) -> Expression {
		Expression::Numeral(Fraction::from(self as i32))
	}
}

impl From<i32> for Fraction {
	fn from(numerator: i32) -> Self {
		Fraction::new(numerator, 1)
	}
}

impl Into<Expression> for Fraction {
	fn into(self) -> Expression {
		Expression::Numeral(self)
	}
}

// custom traits
impl SubIn for Fraction {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		let _ = var;
		let _ = val;
		Expression::Numeral(*self)
	}
}
