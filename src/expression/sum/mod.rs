use crate::expression::numeral::Fraction;
use crate::expression::{Expression, Product, SubIn};
use crate::prod;
use std::collections::HashMap;
use std::fmt;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn sum_display() {
		assert_eq!(crate::sum!().to_string(), "0");
		assert_eq!(sum!(1, "a").to_string(), "1 + a");
		assert_eq!(sum_verbatim!(1, 2, "a").to_string(), "1 + 2 + a");
		assert_eq!(sum!(1, "x", 0).to_string(), "1 + x");
		assert_eq!(sum!(1, "y", prod!(0, "x")).to_string(), "1 + y");
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
			"5x - 3\\left( 7x + y \\right)"
		);
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = sum!(prod!(4, "x"), prod!(9, "y"));
		let exp = exp.sub_in("x", &(5 as i32).into());
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
pub struct Sum {
	pub terms: Vec<Box<Expression>>,
}

impl fmt::Display for Sum {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.terms.is_empty() {
			return write!(f, "0");
		}
		let mut terms = self.terms.iter();
		if let Some(first) = terms.next() {
			write!(f, "{}", first)?;
			for term in terms {
				match term.as_ref() {
					Expression::Numeral(n) if n.numerator < 0 => write!(f, " - {}", n.abs())?,
					Expression::Product(p) if p.coefficient.numerator < 0 => write!(f, " {}", p)?,
					Expression::Quotient(q) => match q.numerator.as_ref() {
						Expression::Numeral(n) if n.numerator < 0 => write!(f, " - {}", q.abs())?,
						Expression::Product(p) if p.coefficient.numerator < 0 => write!(f, " - {}", p.abs())?,
						_ => write!(f, " + {}", term)?,
					},
					_ => write!(f, " + {}", term)?,
				}
			}
		}
		Ok(())
	}
}

impl Sum {
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

	pub fn remove_nested_sums(&mut self) -> () {
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
	}

	pub fn combine_numbers(&mut self) -> () {
		// (index, value)
		let mut first_number: Option<(usize, Fraction)> = Option::None;
		let mut other_indices: Vec<usize> = Vec::new();
		let mut i = 0;
		for t in self.terms.iter_mut() {
			match t.as_mut() {
				Expression::Numeral(f) => {
					(first_number, other_indices) = handle_number(first_number, other_indices, i, f);
				}
				Expression::Product(p) => {
					p.simplify();
					if p.factors.is_empty() {
						(first_number, other_indices) =
							handle_number(first_number, other_indices, i, &p.coefficient);
					}
				}
				_ => (),
			}
			i += 1;
		}
		if let Some((index, val)) = first_number {
			self.terms[index] = Box::new(Expression::Numeral(val));
		}
		for (offset, i) in other_indices.iter().enumerate() {
			self.terms.remove(i - offset);
		}
	}

	pub fn combine_like_terms(&mut self) -> () {
		let mut i = 0;
		// get hashmap of terms
		// variable string: (coefficient, [term indices])
		let mut term_map: HashMap<String, (Fraction, Vec<usize>)> = std::collections::HashMap::new();
		for t in self.terms.iter() {
			match t.as_ref() {
				Expression::Product(p) => {
					let key = p.lexical_string();
					let val = term_map.get_mut(&key);
					if let Some((ref mut coeff, ref mut indices)) = val {
						*coeff = *coeff + p.coefficient.clone();
						indices.push(i);
					} else {
						term_map.insert(key, (p.coefficient.clone(), vec![i]));
					}
				}
				Expression::Numeral(n) => {
					let key = "numeral";
					let val = term_map.get_mut(key);
					if let Some((ref mut coeff, ref mut indices)) = val {
						*coeff = *coeff + n.clone();
						indices.push(i);
					} else {
						term_map.insert(key.to_string(), (n.clone(), vec![i]));
					}
				}
				_ => {
					let key = if let Expression::Sum(s) = t.as_ref() {
						s.lexical_string()
					} else {
						t.to_string()
					};
					let val = term_map.get_mut(&key);
					if let Some((ref mut coeff, ref mut indices)) = val {
						*coeff = *coeff + 1.into();
						indices.push(i);
					} else {
						term_map.insert(key, (1.into(), vec![i]));
					}
				}
			}
			i += 1;
		}
		// modify affected term
		let mut indices_to_remove: Vec<usize> = Vec::new();
		for (_, (coefficient, indices)) in term_map.iter() {
			if indices.len() > 1 {
				let mut indices = indices.iter();
				let first = indices.next().unwrap();
				let term_to_modify = self.terms[*first].as_mut();
				match term_to_modify {
					Expression::Product(p) => {
						p.coefficient = coefficient.clone();
					}
					Expression::Numeral(n) => {
						*n = coefficient.clone();
					}
					_ => {
						if !coefficient.is_one() {
							*term_to_modify = prod!(coefficient.clone(), term_to_modify.clone());
						}
					}
				};
				for i in indices {
					indices_to_remove.push(*i);
				}
			}
		}
		indices_to_remove.sort();
		let mutated = indices_to_remove.len() > 0;
		for (offset, i) in indices_to_remove.iter().enumerate() {
			self.terms.remove(i - offset);
		}
		if mutated {
			self.simplify();
		}
	}

	pub fn simplify(&mut self) -> () {
		self.remove_zeros();
		self.remove_nested_sums();
		for term in self.terms.iter_mut() {
			term.simplify();
		}
		self.combine_like_terms();
	}

	pub fn lexical_string(&self) -> String {
		let mut sums: Vec<String> = Vec::new();
		let mut exps: Vec<String> = Vec::new();
		let mut vars: Vec<String> = Vec::new();
		let mut quotients: Vec<String> = Vec::new();
		let mut numerals: Vec<String> = Vec::new();
		let mut products: Vec<String> = Vec::new();
		for term in self.terms.iter() {
			match term.as_ref() {
				Expression::Sum(s) => {
					sums.push(s.to_string());
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
		terms.join("+")
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

impl SubIn for Sum {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		let mut terms: Vec<Box<Expression>> = Vec::new();
		for term in self.terms.iter() {
			terms.push(Box::new(term.sub_in(var, val)));
		}
		let mut sum = Sum { terms };
		sum.simplify();
		Expression::Sum(sum)
	}
}
