use std::convert::{From, Into};
use std::fmt;
use std::ops::{Add, Mul};

#[macro_export]
macro_rules! sum {
	( $( $x:expr ),* ) => {
		{
			let mut _terms = Vec::new();
			$(
				_terms.push(Box::new($x.into()));
			)*
			let mut _s = Sum { terms: _terms };
			_s.remove_zeros();
			Expression::Sum(_s)
		}
	};
}

#[macro_export]
macro_rules! sum_verbatim {
	( $( $x:expr ),* ) => {
		{
			let mut _terms = Vec::new();
			$(
				_terms.push(Box::new($x.into()));
			)*
			let mut _s = Sum { terms: _terms };
			Expression::Sum(_s)
		}
	};
}

#[macro_export]
macro_rules! prod {
	( $( $x:expr ),* ) => {
		{
			let mut _factors = Vec::new();
			$(
				_factors.push(Box::new($x.into()));
			)*
			let mut p = Product { coefficient: Fraction::from(1), factors: _factors, fraction_mode: false };
			p.collect_coefficients();
			Expression::Product(p)
		}
	};
}

#[macro_export]
macro_rules! prod_verbatim {
	( $( $x:expr ),* ) => {
		{
			let mut _factors = Vec::new();
			$(
				_factors.push(Box::new($x.into()));
			)*
			let p = Product { coefficient: Fraction::from(1), factors: _factors, fraction_mode: false };
			Expression::Product(p)
		}
	};
}

#[macro_export]
macro_rules! prod_fraction {
	( $( $x:expr ),* ) => {
		{
			let mut _factors = Vec::new();
			$(
				_factors.push(Box::new($x.into()));
			)*
			let mut p = Product { coefficient: Fraction::from(1), factors: _factors, fraction_mode: true };
			p.collect_coefficients();
			Expression::Product(p)
		}
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sum_display() {
		assert_eq!(sum!().to_string(), "");
		assert_eq!(sum!(1, 2, "a").to_string(), "1 + 2 + a");
		assert_eq!(sum!(1, 2, 0).to_string(), "1 + 2");
		assert_eq!(sum!(1, 2, prod!(0, "x")).to_string(), "1 + 2");
		assert_eq!(sum_verbatim!(1, 2, 0).to_string(), "1 + 2 + 0");
		assert_eq!(sum_verbatim!(1, prod!("x", 0), 2).to_string(), "1 + 0 + 2");
		assert_eq!(
			sum!(Fraction::new(2, 3), "\\pi").to_string(),
			"\\frac{2}{3} + \\pi"
		);
		// Sec 1a, Page 60, Q6a
		assert_eq!(sum!(prod!(4, "x"), prod!(9, "y")).to_string(), "4x + 9y");
		// Sec 1a, Page 60, Q6b
		assert_eq!(sum!(prod!(4, "x"), prod!(-9, "y")).to_string(), "4x - 9y");
		assert_eq!(sum!(prod!(4, "x"), -9).to_string(), "4x - 9");
	}

	#[test]
	fn product_display() {
		assert_eq!(prod!().to_string(), "");
		assert_eq!(prod!(0, "x").to_string(), "0");
		assert_eq!(prod!(2, "x", Fraction::new(1, 2), "y").to_string(), "xy");
		assert_eq!(
			prod_verbatim!(2, "x", Fraction::new(1, 2), "y").to_string(),
			"2x\\frac{1}{2}y"
		);
		assert_eq!(prod!("x", -1).to_string(), "-x");
		assert_eq!(prod!(-1).to_string(), "-1");
		// Sec 1a, Page 60, Q6c
		assert_eq!(prod!(3, "x", "y").to_string(), "3xy");
		assert_eq!(
			prod!(Fraction::new(1, 3), "x", "y").to_string(),
			"\\frac{1}{3}xy"
		);
		// Sec 1a, Page 60, Q6d
		assert_eq!(
			prod_fraction!(Fraction::new(1, 3), "x", "y").to_string(),
			"\\frac{xy}{3}"
		);
	}
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
	Sum(Sum<'a>),
	Product(Product<'a>),
	Numeral(Fraction),
	//Exponent(Exponent),
	Variable(&'a str),
	// TODO: unary functions
}

impl fmt::Display for Expression<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Sum(s) => write!(f, "{}", s),
			Expression::Product(p) => write!(f, "{}", p),
			Expression::Numeral(n) => write!(f, "{}", n),
			Expression::Variable(v) => write!(f, "{}", v),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Sum<'a> {
	terms: Vec<Box<Expression<'a>>>,
}

impl fmt::Display for Sum<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut terms = self.terms.iter();
		if let Some(first) = terms.next() {
			write!(f, "{}", first)?;
			for term in terms {
				match term.as_ref() {
					Expression::Numeral(n) if n.numerator < 0 => write!(f, " - {}", n.abs())?,
					Expression::Product(p) if p.coefficient.numerator < 0 => write!(f, " - {}", p.abs())?,
					_ => write!(f, " + {}", term)?,
				}
			}
		}
		Ok(())
	}
}

impl Sum<'_> {
	pub fn remove_zeros(&mut self) -> () {
		let mut terms: Vec<Box<Expression>> = Vec::new();
		for term in self.terms.iter_mut() {
			match term.as_mut() {
				Expression::Numeral(n) => {
					if n != &0.into() {
						terms.push(term.clone());
					}
				}
				Expression::Product(p) => {
					if p.coefficient != 0.into() {
						terms.push(term.clone());
					}
				}
				_ => {
					terms.push(term.clone());
				}
			}
		}
		self.terms = terms;
	}
}

#[derive(Debug, Clone)]
pub struct Product<'a> {
	coefficient: Fraction,
	factors: Vec<Box<Expression<'a>>>,
	fraction_mode: bool,
}

impl fmt::Display for Product<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.coefficient == 0.into() {
			return write!(f, "0");
		}
		// fraction mode: \\frac{x}{3y}
		if self.fraction_mode {
			let mut num_factors: Vec<Box<Expression>> = Vec::new();
			let mut den_factors: Vec<Box<Expression>> = Vec::new();
			for factor in self.factors.iter() {
				num_factors.push(factor.clone());
			}
			if !(self.coefficient.denominator == 1 && den_factors.is_empty()) {
				let den = Product {
					coefficient: (self.coefficient.denominator as i32).into(),
					factors: den_factors,
					fraction_mode: false,
				};
				let num = Product {
					coefficient: self.coefficient.numerator.into(),
					factors: num_factors,
					fraction_mode: false,
				};
				return write!(f, "\\frac{{{}}}{{{}}}", num, den);
			}
		}
		// auto mode: 1/3 xy^-1
		if self.coefficient == (-1).into() {
			if self.factors.is_empty() {
				return write!(f, "-1");
			}
			write!(f, "-")?;
		} else if self.coefficient != 1.into() {
			write!(f, "{}", self.coefficient)?;
		}
		let mut factors = self.factors.iter();
		if let Some(first) = factors.next() {
			write!(f, "{}", first)?;
			for factor in factors {
				write!(f, "{}", factor)?;
			}
		}
		Ok(())
	}
}

impl Product<'_> {
	pub fn collect_coefficients(&mut self) -> () {
		let mut factors: Vec<Box<Expression>> = Vec::new();
		for factor in self.factors.iter_mut() {
			match factor.as_mut() {
				Expression::Numeral(n) => {
					self.coefficient = self.coefficient * *n;
				}
				_ => {
					factors.push(factor.clone());
				}
			}
		}
		self.factors = factors;
	}

	pub fn abs(&self) -> Product {
		Product {
			coefficient: self.coefficient.abs(),
			factors: self.factors.clone(),
			fraction_mode: self.fraction_mode,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
	numerator: i32,
	denominator: u32,
}

impl fmt::Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.denominator == 1 {
			return write!(f, "{}", self.numerator);
		}
		write!(f, "\\frac{{{}}}{{{}}}", self.numerator, self.denominator)
	}
}

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

impl Mul<Fraction> for Fraction {
	type Output = Fraction;
	fn mul(self, rhs: Fraction) -> Fraction {
		Fraction::new(
			self.numerator * rhs.numerator,
			self.denominator as i32 * rhs.denominator as i32,
		)
	}
}

impl From<i32> for Fraction {
	fn from(numerator: i32) -> Self {
		Fraction::new(numerator, 1)
	}
}

impl PartialEq for Fraction {
	fn eq(&self, other: &Self) -> bool {
		self.numerator * other.denominator as i32 == self.denominator as i32 * other.numerator
	}
}

impl<'a> Into<Expression<'a>> for i32 {
	fn into(self) -> Expression<'a> {
		Expression::Numeral(Fraction::from(self))
	}
}

impl<'a> Into<Expression<'a>> for &'a str {
	fn into(self) -> Expression<'a> {
		Expression::Variable(self)
	}
}

impl<'a> Into<Expression<'a>> for Fraction {
	fn into(self) -> Expression<'a> {
		Expression::Numeral(self)
	}
}
