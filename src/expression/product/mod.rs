use crate::expression::exponent::Exponent;
use crate::expression::numeral::Fraction;
use crate::expression::{Expression, SubIn};
use std::fmt;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn product_display() {
		assert_eq!(prod!().to_string(), "1");
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
		// Sec 1a, Page 61, Q9c
		assert_eq!(
			sum!(prod!(5, sum!("x", prod!(2, "y"))), prod!(-9, "x")).to_string(),
			"5\\left( x + 2y \\right) - 9x"
		);
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = prod!(3, "x", "y");
		let exp = exp.sub_in("x", &(5 as i32).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "-30");
		let exp = prod!(Fraction::new(1, 3), "x", "y");
		let exp = exp.sub_in("x", &(5).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "-\\frac{10}{3}");
		// Sec 1a, Page 61, Q9c
		let exp = sum!(prod!(5, sum!("x", prod!(2, "y"))), prod!(-9, "x"));
		let exp = exp.sub_in("x", &Fraction::new(1, 3).into());
		let exp = exp.sub_in("y", &Fraction::new(-1, 4).into());
		let f: Fraction = exp.try_into().unwrap();
		assert_eq!(f.to_mixed_fraction(), "-3\\frac{5}{6}");
	}

	#[test]
	fn quotient() {
		let exp = quotient!("x", exp!("y", 20));
		assert_eq!(exp.to_string(), "\\frac{x}{y^{20}}");
		// Sec 1a, Page 60, Q7c
		let exp = sum!(quotient!("x", prod!(5, "y")), quotient!("y", prod!(5, "x")));
		assert_eq!(exp.to_string(), "\\frac{x}{5y} + \\frac{y}{5x}");
		let exp = exp.sub_in("x", &(-4).into());
		let exp = exp.sub_in("y", &(7).into());
		assert_eq!(exp.to_string(), "-\\frac{13}{28}");
		// Sec 1a, Page 61 Q10b,c,
		let exp = sum!(quotient!(1, "y"), quotient!(-1, "x"));
		assert_eq!(exp.to_string(), "\\frac{1}{y} - \\frac{1}{x}");
		let exp = exp.sub_in("x", &(-5).into());
		let exp = exp.sub_in("y", &Fraction::new(1, 4).into());
		assert_eq!(exp.to_string(), "\\frac{21}{5}");
		let exp = quotient!(sum!("x", prod!(-1, "y")), sum!("x", "y"));
		assert_eq!(exp.to_string(), "\\frac{x - y}{x + y}");
		let exp = exp.sub_in("x", &(-5).into());
		let exp = exp.sub_in("y", &Fraction::new(1, 4).into());
		assert_eq!(exp.to_string(), "\\frac{21}{19}");
		// 9b, 10c
	}
}

#[derive(Debug, Clone)]
pub struct Product<'a> {
	pub coefficient: Fraction,
	pub factors: Vec<Box<Expression<'a>>>,
	pub fraction_mode: bool,
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
				match factor.as_ref() {
					Expression::Exponent(e) => match e.exponent.as_ref() {
						Expression::Numeral(n) => {
							if n.is_negative() {
								let mut exp = Expression::Exponent(Exponent {
									base: e.base.clone(),
									exponent: Box::new(Expression::Numeral(n.negative())),
								});
								exp.simplify();
								den_factors.push(Box::new(exp));
							} else {
								num_factors.push(factor.clone());
							}
						}
						Expression::Product(p) => {
							if p.coefficient.is_negative() {
								let mut exp = Expression::Exponent(Exponent {
									base: e.base.clone(),
									exponent: Box::new(Expression::Product(p.negative())),
								});
								exp.simplify();
								den_factors.push(Box::new(exp));
							} else {
								num_factors.push(factor.clone());
							}
						}
						_ => {
							num_factors.push(factor.clone());
						}
					},
					_ => {
						num_factors.push(factor.clone());
					}
				}
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
				//println!("den: {:?}", den);
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
		} else {
			if self.factors.is_empty() {
				return write!(f, "1");
			}
		}
		for factor in self.factors.iter() {
			match factor.as_ref() {
				Expression::Sum(s) => {
					if self.coefficient.is_one() && self.factors.len() == 1 {
						write!(f, "{}", s)?;
					} else {
						write!(f, "\\left( {} \\right)", s)?;
					}
				}
				_ => write!(f, "{}", factor)?,
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

	pub fn simplify(&mut self) -> () {
		self.collect_coefficients();
		// loop 1: simplify factors
		for factor in self.factors.iter_mut() {
			factor.simplify();
		}
		let mut factors: Vec<Box<Expression>> = Vec::new();
		let mut coefficient = self.coefficient.clone();
		let mut product_found = false;
		// loop 2: remove nested products
		for factor in self.factors.iter_mut() {
			match factor.as_mut() {
				Expression::Product(p) => {
					product_found = true;
					p.simplify();
					for f in p.factors.iter() {
						factors.push(f.clone());
						coefficient = self.coefficient * p.coefficient;
					}
				}
				_ => {
					let mut f = factor.clone();
					f.simplify();
					factors.push(f);
				}
			}
		}
		if product_found {
			let mut prod = Product {
				coefficient,
				factors,
				fraction_mode: self.fraction_mode,
			};
			prod.simplify();
			*self = prod;
		}
	}

	pub fn abs(&self) -> Product {
		Product {
			coefficient: self.coefficient.abs(),
			factors: self.factors.clone(),
			fraction_mode: self.fraction_mode,
		}
	}

	pub fn negative(&self) -> Product {
		Product {
			coefficient: self.coefficient.negative(),
			factors: self.factors.clone(),
			fraction_mode: self.fraction_mode,
		}
	}
}

impl SubIn for Product<'_> {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		let mut factors: Vec<Box<Expression>> = Vec::new();
		for factor in self.factors.iter() {
			let f = factor.sub_in(var, val);
			match f {
				Expression::Product(mut p) => {
					p.simplify();
					if p.factors.is_empty() {
						factors.push(Box::new(Expression::Numeral(p.coefficient)));
					} else {
						factors.push(Box::new(Expression::Product(p)));
					}
				}
				Expression::Sum(mut s) => {
					s.simplify();
					factors.push(Box::new(Expression::Sum(s)));
				}
				_ => {
					factors.push(Box::new(f));
				}
			}
		}
		let mut prod = Product {
			coefficient: self.coefficient.clone(),
			factors,
			fraction_mode: self.fraction_mode,
		};
		prod.simplify();
		Expression::Product(prod)
	}
}
