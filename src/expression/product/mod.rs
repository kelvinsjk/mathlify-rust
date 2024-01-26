use crate::expression::numeral::Fraction;
use crate::expression::{Expression, SubIn};
use std::collections::HashMap;
use std::fmt;
pub mod product_lcm;

use super::Exponent;

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
		assert_eq!(prod!("x", -1).to_string(), "- x");
		assert_eq!(prod!(-1).to_string(), "- 1");
		// Sec 1a, Page 60, Q6c
		assert_eq!(prod!(3, "x", "y").to_string(), "3xy");
		assert_eq!(
			prod!(Fraction::new(1, 3), "x", "y").to_string(),
			"\\frac{1}{3}xy"
		);
		// Sec 1a, Page 60, Q6d
		assert_eq!(quotient!(prod!("x", "y"), 3).to_string(), "\\frac{xy}{3}");
		// Sec 1a, Page 61, Q9c
		assert_eq!(
			sum!(prod!(5, sum!("x", prod!(2, "y"))), prod!(-9, "x")).to_string(),
			"5\\left( x + 2y \\right) - 9x"
		);
	}

	#[test]
	fn nested_products() {
		let mut p = prod!(prod!(2, "x"), prod!(3, "y"));
		p.simplify();
		assert_eq!(p.to_string(), "6xy");
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = prod!(3, "x", "y");
		let exp = exp.sub_in("x", &(5 as i32).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "- 30");
		let exp = prod!(Fraction::new(1, 3), "x", "y");
		let exp = exp.sub_in("x", &(5).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "- \\frac{10}{3}");
		// Sec 1a, Page 61, Q9c
		let exp = sum!(prod!(5, sum!("x", prod!(2, "y"))), prod!(-9, "x"));
		let exp = exp.sub_in("x", &Fraction::new(1, 3).into());
		let exp = exp.sub_in("y", &Fraction::new(-1, 4).into());
		let f: Fraction = exp.try_into().unwrap();
		assert_eq!(f.to_mixed_fraction(), "- 3\\frac{5}{6}");
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
		assert_eq!(exp.to_string(), "- \\frac{13}{28}");
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

	#[test]
	fn lexical_string() {
		let p = Product {
			coefficient: 1.into(),
			factors: vec![
				Box::new(Expression::Variable("z".to_string())),
				Box::new(sum!(prod!(2, "a"), prod!(3, "b"))),
				Box::new(Expression::Variable("x".to_string())),
				Box::new(exp!("y", 2)),
			],
		};
		assert_eq!(p.lexical_string(), "2a+3by^2xz");
	}
}

#[derive(Debug, Clone)]
pub struct Product {
	pub coefficient: Fraction,
	pub factors: Vec<Box<Expression>>,
}

impl fmt::Display for Product {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.coefficient == 0.into() {
			return write!(f, "0");
		}
		// auto mode: 1/3 xy^-1
		if self.coefficient == (-1).into() {
			if self.factors.is_empty() {
				return write!(f, "- 1");
			}
			write!(f, "- ")?;
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

impl Product {
	// combine numerals
	// combine variables x, and exponents x^n where n is a number and x is a variable
	pub fn collect_coefficients(&mut self) -> () {
		let mut factors: Vec<Box<Expression>> = Vec::new();
		let mut i = 0;
		// variable string: (coefficient, term index)
		let mut term_map: HashMap<String, (Fraction, usize)> = std::collections::HashMap::new();
		for factor in self.factors.iter_mut() {
			match factor.as_mut() {
				Expression::Numeral(n) => {
					self.coefficient = self.coefficient * *n;
				}
				Expression::Variable(v) => {
					if term_map.contains_key(v) {
						let (power, index) = term_map.get(v).unwrap();
						term_map.insert(v.clone(), (*power + 1.into(), *index));
					} else {
						term_map.insert(v.clone(), (1.into(), i));
						factors.push(factor.clone());
						i += 1;
					}
				}
				Expression::Exponent(e) => {
					if let (Expression::Variable(v), Expression::Numeral(n)) =
						(e.base.as_ref(), e.exponent.as_ref())
					{
						if term_map.contains_key(v) {
							let (power, index) = term_map.get(v).unwrap();
							term_map.insert(v.clone(), (*power + *n, *index));
						} else {
							term_map.insert(v.clone(), (n.clone(), i));
							factors.push(factor.clone());
							i += 1;
						}
					} else {
						factors.push(factor.clone());
						i += 1;
					}
				}
				_ => {
					factors.push(factor.clone());
					i += 1;
				}
			}
		}
		// mutate final factor
		let mut offset = 0;
		for (var, (power, index)) in term_map.iter() {
			if power.is_zero() {
				factors.remove(*index - offset);
				offset += 1;
			} else if power.is_one() {
				factors[*index - offset] = Box::new(Expression::Variable(var.clone()));
			} else {
				factors[*index - offset] = Box::new(Expression::Exponent(Exponent {
					base: Box::new(Expression::Variable(var.clone())),
					exponent: Box::new(Expression::Numeral(power.clone())),
				}));
			}
		}
		self.factors = factors;
	}

	pub fn remove_nested_products(&mut self) -> () {
		let mut factors: Vec<Box<Expression>> = Vec::new();
		let mut coefficient = self.coefficient.clone();
		let mut product_found = false;
		for factor in self.factors.iter_mut() {
			match factor.as_mut() {
				Expression::Product(p) => {
					product_found = true;
					p.simplify();
					coefficient = coefficient * p.coefficient;
					for f in p.factors.iter() {
						factors.push(f.clone());
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
			};
			prod.simplify();
			*self = prod;
		}
	}

	pub fn simplify(&mut self) -> () {
		self.collect_coefficients();
		for factor in self.factors.iter_mut() {
			factor.simplify();
		}
		self.remove_nested_products();
	}

	pub fn abs(&self) -> Product {
		Product {
			coefficient: self.coefficient.abs(),
			factors: self.factors.clone(),
		}
	}

	pub fn negative(&self) -> Product {
		Product {
			coefficient: self.coefficient.negative(),
			factors: self.factors.clone(),
		}
	}

	// ignores the coefficient, and returns the
	// factors as a string in the following order:
	// sums (with to_string(), sorted by default order)
	// exps (with to_string(), sorted by default order)
	// variables (sorted by default order)
	// known issue: (x+y)z and and x+yz both return x+yz
	pub fn lexical_string(&self) -> String {
		let mut sums: Vec<String> = Vec::new();
		let mut exps: Vec<String> = Vec::new();
		let mut vars: Vec<String> = Vec::new();
		let mut quotients: Vec<String> = Vec::new();
		let mut numerals: Vec<String> = Vec::new();
		let mut products: Vec<String> = Vec::new();
		for factor in self.factors.iter() {
			match factor.as_ref() {
				Expression::Sum(s) => {
					sums.push(s.lexical_string());
				}
				Expression::Exponent(e) => {
					exps.push(e.to_string());
				}
				Expression::Quotient(q) => {
					quotients.push(q.to_string());
				}
				Expression::Variable(v) => {
					vars.push(v.to_string());
				}
				Expression::Numeral(n) => {
					numerals.push(n.to_string());
				}
				Expression::Product(p) => {
					products.push(p.coefficient.to_string() + &p.lexical_string());
				}
			}
		}
		sums.sort();
		exps.sort();
		quotients.sort();
		vars.sort();
		numerals.sort();
		products.sort();
		let mut terms: Vec<String> = Vec::new();
		terms.extend(sums);
		terms.extend(exps);
		terms.extend(quotients);
		terms.extend(vars);
		terms.extend(numerals);
		terms.extend(products);
		terms.join("")
	}

	pub fn has_variable(&self, x: &str) -> bool {
		for factor in self.factors.iter() {
			match factor.as_ref() {
				Expression::Variable(v) => {
					if v == x {
						return true;
					}
				}
				Expression::Exponent(e) => {
					if e.has_variable(x) {
						return true;
					}
				}
				_ => {}
			}
		}
		false
	}

	pub fn variable_pow(&self, x: &str) -> Option<Fraction> {
		for factor in self.factors.iter() {
			match factor.as_ref() {
				Expression::Exponent(e) => {
					if e.base.to_string() == x {
						match e.exponent.as_ref() {
							Expression::Numeral(n) => {
								return Some(n.clone());
							}
							_ => {}
						}
					}
				}
				Expression::Variable(v) => {
					if v == x {
						return Some(1.into());
					}
				}
				_ => {}
			}
		}
		None
	}

	// returns without coefficient
	pub fn variable_decrement(&self, var: &str, pow: &Fraction) -> Vec<Box<Expression>> {
		let mut factors: Vec<Box<Expression>> = Vec::new();
		for factor in self.factors.iter() {
			match factor.as_ref() {
				Expression::Variable(v) => {
					if v != var {
						factors.push(factor.clone());
					}
				}
				Expression::Exponent(e) => {
					if let (Expression::Variable(v), Expression::Numeral(n)) =
						(e.base.as_ref(), e.exponent.as_ref())
					{
						if v == var {
							let mut exp = Expression::Exponent(Exponent {
								base: e.base.clone(),
								exponent: Box::new(Expression::Numeral(n.clone() - pow.clone())),
							});
							exp.simplify();
							factors.push(Box::new(exp));
						} else {
							factors.push(factor.clone());
						}
					} else {
						factors.push(factor.clone());
					}
				}
				_ => {
					factors.push(factor.clone());
				}
			}
		}
		factors
	}
}

impl SubIn for Product {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
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
		};
		prod.simplify();
		Expression::Product(prod)
	}
}
