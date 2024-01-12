use crate::expression::numeral::Fraction;
use crate::expression::{Expression, SubIn};
use std::fmt;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn sum_display() {
		assert_eq!(crate::sum!().to_string(), "");
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
		// Sec 1a, Page 60, Q7b
		assert_eq!(
			sum!(prod!(5, "x"), prod!(-3, sum!(prod!(7, "x"), "y"))).to_string(),
			"5x - 3 \\left( 7x + y \\right)"
		);
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = sum!(prod!(4, "x"), prod!(9, "y"));
		let exp = exp.sub_in("x", &(5 as i32).into());
		println!("{}", exp.to_string());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "2");
		let exp = sum!(prod!(4, "x"), prod!(-9, "y"));
		let exp = exp.sub_in("x", &(5).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "38");
		// Sec 1a, Page 60, Q7a,b
		let exp = sum!(prod!(-11, "x"), prod!(-2, "y"));
		let exp = exp.sub_in("x", &(-4).into());
		let exp = exp.sub_in("y", &(7).into());
		assert_eq!(exp.to_string(), "30");
		let exp = sum!(prod!(5, "x"), prod!(-3, sum!(prod!(7, "x"), "y")));
		let exp = exp.sub_in("x", &(-4).into());
		let exp = exp.sub_in("y", &(7).into());
		assert_eq!(exp.to_string(), "43");
	}
}

#[derive(Debug, Clone)]
pub struct Sum<'a> {
	pub terms: Vec<Box<Expression<'a>>>,
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

	pub fn simplify(&mut self) -> () {
		// remove nested sums
		let mut terms: Vec<Box<Expression>> = Vec::new();
		for term in self.terms.iter() {
			match term.as_ref() {
				Expression::Sum(s) => {
					for t in s.terms.iter() {
						terms.push(t.clone());
					}
				}
				_ => {
					terms.push(term.clone());
				}
			}
		}
		self.terms = terms;
		// combine all numbers
		let mut first_number: Option<(usize, Fraction)> = Option::None;
		// (index, val)
		let mut other_indices: Vec<usize> = Vec::new();
		let mut i = 0;
		for term in self.terms.iter_mut() {
			let t = term.as_mut();
			t.simplify();
			match t {
				Expression::Numeral(f) => {
					(first_number, other_indices) = handle_number(first_number, other_indices, i, f);
				}
				Expression::Variable(_v) => (),
				Expression::Product(p) => {
					p.collect_coefficients();
					if p.factors.is_empty() {
						(first_number, other_indices) =
							handle_number(first_number, other_indices, i, &p.coefficient);
					}
				}
				Expression::Sum(s) => s.simplify(),
			}
			i += 1;
		}
		self.terms[first_number.unwrap().0] = Box::new(Expression::Numeral(first_number.unwrap().1));
		for i in other_indices {
			self.terms.remove(i);
		}
	}
}

fn handle_number(
	mut first_number: Option<(usize, Fraction)>,
	mut other_indices: Vec<usize>,
	i: usize,
	f: &Fraction,
) -> (Option<(usize, Fraction)>, Vec<usize>) {
	if first_number.is_none() {
		first_number = Option::Some((i, f.clone()));
	} else {
		let (index, val) = first_number.unwrap();
		first_number = Option::Some((index, val + f.clone()));
		other_indices.push(i);
	}
	return (first_number, other_indices);
}

impl SubIn for Sum<'_> {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		let mut terms: Vec<Box<Expression>> = Vec::new();
		for term in self.terms.iter() {
			terms.push(Box::new(term.sub_in(var, val)));
		}
		let mut sum = Sum { terms };
		sum.simplify();
		Expression::Sum(sum)
	}
}