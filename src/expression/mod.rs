pub mod exponent;
pub mod numeral;
pub mod product;
pub mod quotient;
pub mod sum;
pub mod variable;
pub use exponent::Exponent;
pub use numeral::Fraction;
pub use product::Product;
pub use quotient::Quotient;
use std::collections::HashMap;
use std::convert::TryInto;
pub use sum::Sum;

use std::fmt;

#[macro_export]
macro_rules! sum {
	( $( $x:expr ),* ) => {
		{
			let mut _terms = Vec::new();
			$(
				_terms.push(Box::new($x.into()));
			)*
			let mut _s = Sum { terms: _terms };
			let mut _e = Expression::Sum(_s);
			_e.simplify();
			_e
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
			let mut p = Product { coefficient: Fraction::from(1), factors: _factors, };
			p.simplify();
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
			let p = Product { coefficient: Fraction::from(1), factors: _factors,  };
			Expression::Product(p)
		}
	};
}

#[macro_export]
macro_rules! quotient {
	( $a:expr, $b:expr ) => {{
		let q = Quotient {
			numerator: Box::new($a.into()),
			denominator: Box::new($b.into()),
		};
		Expression::Quotient(q)
	}};
}

#[macro_export]
macro_rules! exp {
	( $b:expr, $e:expr ) => {
		Expression::Exponent(Exponent {
			base: Box::new($b.into()),
			exponent: Box::new($e.into()),
		})
	};
}

#[derive(Debug, Clone)]
pub enum Expression {
	Sum(Sum),
	Product(Product),
	Quotient(Quotient),
	Exponent(Exponent),
	Numeral(Fraction),
	Variable(String),
	// TODO: unary functions
}

impl fmt::Display for Expression {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expression::Sum(s) => write!(f, "{}", s),
			Expression::Product(p) => write!(f, "{}", p),
			Expression::Quotient(q) => write!(f, "{}", q),
			Expression::Exponent(e) => write!(f, "{}", e),
			Expression::Variable(v) => write!(f, "{}", v),
			Expression::Numeral(n) => write!(f, "{}", n),
		}
	}
}

pub trait SubIn {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression;
}

impl SubIn for Expression {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		let mut r = match self {
			Expression::Sum(s) => s.sub_in(var, val),
			Expression::Product(p) => p.sub_in(var, val),
			Expression::Quotient(q) => q.sub_in(var, val),
			Expression::Exponent(e) => e.sub_in(var, val),
			Expression::Variable(v) => v.sub_in(var, val),
			Expression::Numeral(n) => n.sub_in(var, val),
		};
		r.simplify();
		r
	}
}

impl Expression {
	pub fn simplify(&mut self) -> () {
		match self {
			Expression::Sum(s) => {
				s.simplify();
				for t in s.terms.iter_mut() {
					t.simplify();
				}
				// remove singleton sums
				if s.terms.len() == 1 {
					*self = s.terms[0].as_mut().clone();
				} else {
					// combine like terms
					let mut i = 0;
					// variable string: (coefficient, [term index])
					let mut term_map: HashMap<String, (Fraction, Vec<usize>)> =
						std::collections::HashMap::new();
					for t in s.terms.iter() {
						match t.as_ref() {
							Expression::Product(p) => {
								let lexical_string = p.lexical_string();
								if term_map.contains_key(&lexical_string) {
									let (coefficient, indices) = term_map.get(&lexical_string).unwrap();
									let coefficient = *coefficient + p.coefficient.clone();
									let mut indices = indices.clone();
									indices.push(i);
									term_map.insert(lexical_string, (coefficient, indices));
								} else {
									term_map.insert(lexical_string, (p.coefficient.clone(), vec![i]));
								}
							}
							Expression::Exponent(e) => {
								let string = e.to_string();
								if term_map.contains_key(&string) {
									let (coefficient, indices) = term_map.get(&string).unwrap();
									let coefficient = *coefficient + 1.into();
									let mut indices = indices.clone();
									indices.push(i);
									term_map.insert(string, (coefficient, indices));
								} else {
									term_map.insert(string, (1.into(), vec![i]));
								}
							}
							Expression::Variable(v) => {
								if term_map.contains_key(v) {
									let (coefficient, indices) = term_map.get(v).unwrap();
									let coefficient = *coefficient + 1.into();
									let mut indices = indices.clone();
									indices.push(i);
									term_map.insert(v.to_string(), (coefficient, indices));
								} else {
									term_map.insert(v.to_string(), (1.into(), vec![i]));
								}
							}
							_ => (),
						}
						i += 1;
					}
					let mut indices_to_remove: Vec<usize> = Vec::new();
					for (_, (coefficient, indices)) in term_map.iter() {
						if indices.len() > 1 {
							let mut indices = indices.iter();
							let first = indices.next().unwrap();
							let term_to_modify = s.terms[*first].as_mut();
							match term_to_modify {
								Expression::Product(p) => {
									p.coefficient = coefficient.clone();
								}
								Expression::Exponent(e) => {
									if !coefficient.is_one() {
										*term_to_modify = prod!(coefficient.clone(), e.clone());
									}
								}
								Expression::Variable(v) => {
									if !coefficient.is_one() {
										*term_to_modify = prod!(coefficient.clone(), v.clone());
									}
								}
								_ => (),
							};
							for i in indices {
								indices_to_remove.push(*i);
							}
						}
					}
					indices_to_remove.sort();
					for (offset, i) in indices_to_remove.iter().enumerate() {
						s.terms.remove(i - offset);
					}
				}
			}
			Expression::Product(p) => {
				// remove singleton products
				for f in p.factors.iter_mut() {
					f.simplify();
				}
				if p.factors.len() == 0 {
					*self = Expression::Numeral(p.coefficient.clone());
				} else if p.factors.len() == 1 && p.coefficient == 1.into() {
					*self = p.factors[0].as_mut().clone();
				}
			}
			Expression::Exponent(e) => {
				e.base.simplify();
				e.exponent.simplify();
				// number^number -> number
				match (e.base.as_ref(), e.exponent.as_ref()) {
					(Expression::Numeral(b), Expression::Numeral(e)) => {
						if e.is_integer() {
							let n = Box::new(Expression::Numeral(b.pow(e.numerator)));
							*self = *n;
						}
					}
					_ => {
						if let Expression::Numeral(n) = e.exponent.as_ref() {
							if n == &(1 as i32).into() {
								// remove power 1
								*self = e.base.as_mut().clone();
								self.simplify();
							} else if let Expression::Product(p) = e.base.as_ref() {
								// exponent of products become product of exponents
								let mut factors: Vec<Box<Expression>> = Vec::new();
								for factor in p.factors.iter() {
									let f = Expression::Exponent(Exponent {
										base: factor.clone(),
										exponent: e.exponent.clone(),
									});
									factors.push(Box::new(f));
								}
								factors.push(Box::new(Expression::Exponent(Exponent {
									base: Box::new(p.coefficient.into()),
									exponent: e.exponent.clone(),
								})));
								let mut p = Product {
									coefficient: Fraction::from(1),
									factors,
								};
								p.simplify();
								*self = Expression::Product(p);
								self.simplify();
							}
						} else if let Expression::Product(p) = e.base.as_ref() {
							// exponent of products become product of exponents
							let mut factors: Vec<Box<Expression>> = Vec::new();
							for factor in p.factors.iter() {
								let f = Expression::Exponent(Exponent {
									base: factor.clone(),
									exponent: e.exponent.clone(),
								});
								factors.push(Box::new(f));
							}
							factors.push(Box::new(Expression::Exponent(Exponent {
								base: Box::new(p.coefficient.into()),
								exponent: e.exponent.clone(),
							})));
							let mut p = Product {
								coefficient: Fraction::from(1),
								factors,
							};
							p.simplify();
							*self = Expression::Product(p);
							self.simplify();
						}
					}
				}
			}
			// variable, numeral
			Expression::Quotient(q) => {
				if let (Expression::Numeral(n), Expression::Numeral(d)) =
					(q.numerator.as_mut(), q.denominator.as_mut())
				{
					*self = Expression::Numeral(*n / *d);
					self.simplify();
				} else if let Expression::Numeral(n) = q.denominator.as_ref() {
					if n.is_one() {
						*self = q.numerator.as_mut().clone();
						self.simplify();
					}
				}
			}
			_ => (),
		}
	}
}

impl TryInto<Fraction> for Expression {
	type Error = ();
	fn try_into(self) -> Result<Fraction, ()> {
		match self {
			Expression::Numeral(n) => Ok(n),
			_ => Err(()),
		}
	}
}
impl TryInto<String> for Expression {
	type Error = ();
	fn try_into(self) -> Result<String, ()> {
		match self {
			Expression::Variable(v) => Ok(v.to_string()),
			_ => Err(()),
		}
	}
}
impl TryInto<Sum> for Expression {
	type Error = ();
	fn try_into(self) -> Result<Sum, ()> {
		match self {
			Expression::Sum(s) => Ok(s),
			_ => Err(()),
		}
	}
}
impl TryInto<Product> for Expression {
	type Error = ();
	fn try_into(self) -> Result<Product, ()> {
		match self {
			Expression::Product(p) => Ok(p),
			_ => Err(()),
		}
	}
}
impl TryInto<Exponent> for Expression {
	type Error = ();
	fn try_into(self) -> Result<Exponent, ()> {
		match self {
			Expression::Exponent(e) => Ok(e),
			_ => Err(()),
		}
	}
}
impl TryInto<Quotient> for Expression {
	type Error = ();
	fn try_into(self) -> Result<Quotient, ()> {
		match self {
			Expression::Quotient(q) => Ok(q),
			_ => Err(()),
		}
	}
}
