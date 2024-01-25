pub mod exponent;
pub mod numeral;
pub mod product;
pub mod quotient;
pub mod sum;
pub mod variable;
pub use exponent::Exponent;
pub use numeral::fraction_gcd::{fraction_gcd, fraction_lcm};
pub use numeral::Fraction;
pub use product::Product;
pub use quotient::Quotient;
use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;
use std::{cmp, fmt};
pub use sum::Sum;

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
	pub fn remove_singletons(&mut self) -> () {
		match self {
			Expression::Sum(s) => {
				for t in s.terms.iter_mut() {
					t.remove_singletons();
				}
				if s.terms.len() == 1 {
					*self = s.terms[0].as_mut().clone();
				}
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					f.remove_singletons();
				}
				if p.coefficient.is_zero() {
					*self = Expression::Numeral(Fraction::from(0));
					return;
				}
				if p.factors.len() == 0 {
					*self = Expression::Numeral(p.coefficient);
				} else if p.factors.len() == 1 && p.coefficient == 1.into() {
					*self = p.factors[0].as_mut().clone();
				}
			}
			Expression::Quotient(q) => {
				q.numerator.remove_singletons();
				q.denominator.remove_singletons();
			}
			Expression::Exponent(e) => {
				e.base.remove_singletons();
				e.exponent.remove_singletons();
			}
			_ => (),
		}
	}

	pub fn simplify(&mut self) -> () {
		self.remove_singletons();
		match self {
			Expression::Sum(s) => {
				s.simplify();
				for t in s.terms.iter_mut() {
					t.simplify();
				}
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
				let mutated = indices_to_remove.len() > 0;
				for (offset, i) in indices_to_remove.iter().enumerate() {
					s.terms.remove(i - offset);
				}
				if mutated {
					self.simplify();
				}
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					f.simplify();
				}
				p.simplify();
			}
			Expression::Exponent(e) => {
				e.base.simplify();
				e.exponent.simplify();
				// TODO: refactor
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
							} else if n == &(0 as i32).into() {
								// remove power 0
								*self = Expression::Numeral(Fraction::from(1));
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
				q.simplify();
				// numeral/numeral -> fraction
				if let (Expression::Numeral(n), Expression::Numeral(d)) =
					(q.numerator.as_mut(), q.denominator.as_mut())
				{
					*self = Expression::Numeral(*n / *d);
					self.simplify();
				} else if let Expression::Numeral(n) = q.denominator.as_ref() {
					// denominator 1 -> only take numerator
					if n.is_one() {
						*self = q.numerator.as_mut().clone();
						self.simplify();
					}
				}
			}
			_ => (),
		}
	}

	pub fn expand(&mut self) -> () {
		match self {
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					f.expand();
				}
				// find sums
				let mut sums: VecDeque<Sum> = VecDeque::new();
				let mut others: Vec<Box<Expression>> = Vec::new();
				for f in p.factors.iter() {
					if let Expression::Sum(s) = f.as_ref() {
						sums.push_back(s.clone());
					} else {
						others.push(f.clone());
					}
				}
				if sums.len() > 0 {
					let mut new_terms: Vec<Box<Expression>> = sums.pop_front().unwrap().terms;
					while sums.len() > 0 {
						let mut terms_to_multiply = sums.pop_front().unwrap().terms;
						for t1 in new_terms.iter_mut() {
							for t2 in terms_to_multiply.iter_mut() {
								let mut t = prod!(t1.as_ref().clone(), t2.as_ref().clone());
								t.simplify();
								*t1 = Box::new(t);
							}
						}
					}
					let mut terms: Vec<Box<Expression>> = Vec::new();
					for t in new_terms.iter() {
						let mut factors = others.clone();
						factors.push(t.clone());
						terms.push(Box::new(Expression::Product(Product {
							coefficient: p.coefficient.clone(),
							factors,
						})));
					}
					let mut s = Sum { terms };
					s.simplify();
					*self = Expression::Sum(s);
				}
			}
			Expression::Sum(s) => {
				for t in s.terms.iter_mut() {
					t.expand();
				}
			}
			Expression::Quotient(q) => {
				q.numerator.expand();
				q.denominator.expand();
			}
			Expression::Exponent(e) => {
				e.base.expand();
				e.exponent.expand();
			}
			_ => (),
		}
		self.remove_nested_sums();
	}

	pub fn expand_and_simplify(&mut self) -> () {
		self.expand();
		self.simplify();
	}

	pub fn remove_nested_sums(&mut self) -> () {
		match self {
			Expression::Sum(s) => {
				s.remove_nested_sums();
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					f.remove_nested_sums();
				}
			}
			Expression::Quotient(q) => {
				q.numerator.remove_nested_sums();
				q.denominator.remove_nested_sums();
			}
			Expression::Exponent(e) => {
				e.base.remove_nested_sums();
				e.exponent.remove_nested_sums();
			}
			_ => (),
		}
	}

	// take out common numeric/variable/exponent factors
	// does not work for sum factors at the moment
	// only work if outer-most expression type is a sum
	// for exponents only work for x^n where n is a numeral at the moment
	// doesn't work for (xy)^n
	pub fn factorize(&mut self) -> () {
		if let Expression::Sum(s) = self {
			if s.terms.len() < 2 {
				return;
			}
			let mut terms = s.terms.iter();
			let first_term = terms.next().unwrap();
			match first_term.as_ref() {
				Expression::Numeral(f) => {
					let mut factor = f.clone();
					for t in terms {
						match t.as_ref() {
							Expression::Product(p) => {
								factor = fraction_gcd(&factor, &p.coefficient);
							}
							Expression::Numeral(f) => {
								factor = fraction_gcd(&factor, &f);
							}
							_ => (),
						}
					}
					// factorize
					if factor.is_nonzero() && factor != 1.into() {
						for t in s.terms.iter_mut() {
							match t.as_mut() {
								Expression::Product(p) => {
									p.coefficient = p.coefficient / factor.clone();
								}
								Expression::Numeral(f) => {
									*f = *f / factor.clone();
								}
								_ => (),
							}
						}
						let mut sum = Expression::Sum(s.clone());
						sum.expand_and_simplify();
						*self = Expression::Product(Product {
							coefficient: factor,
							factors: vec![Box::new(sum)],
						});
					}
					return;
				}
				Expression::Product(p) => {
					let mut variable_exponent_map: HashMap<String, Fraction> = HashMap::new();
					let mut factor = p.coefficient.clone();
					let mut variable_vec: Vec<String> = Vec::new();
					// collate variables and exponents
					for f in &p.factors {
						match f.as_ref() {
							// assumes variable and exponents are mutually exclusive
							Expression::Variable(v) => {
								variable_exponent_map.insert(v.clone(), 1.into());
								variable_vec.push(v.clone());
							}
							Expression::Exponent(e) => {
								if let (Expression::Variable(v), Expression::Numeral(n)) =
									(e.base.as_ref(), e.exponent.as_ref())
								{
									variable_exponent_map.insert(v.clone(), n.clone());
									variable_vec.push(v.clone());
								}
							}
							_ => (),
						}
					}
					// get common factors
					for t in terms {
						match t.as_ref() {
							Expression::Product(p) => {
								factor = fraction_gcd(&factor, &p.coefficient);
								for v in variable_exponent_map.clone().keys() {
									let mut v_found = false;
									for f in &p.factors {
										match f.as_ref() {
											Expression::Variable(v2) => {
												if v == v2 {
													v_found = true;
													let power = variable_exponent_map.get_mut(v).unwrap();
													if power > &mut 1.into() {
														*power = 1.into();
													}
													break;
												}
											}
											Expression::Exponent(e) => {
												if let (Expression::Variable(v2), Expression::Numeral(n)) =
													(e.base.as_ref(), e.exponent.as_ref())
												{
													if v == v2 {
														v_found = true;
														let power = variable_exponent_map.get_mut(v).unwrap();
														if power > &mut n.clone() {
															*power = n.clone();
														}
														break;
													}
												}
											}
											_ => (),
										}
									}
									if !v_found {
										variable_exponent_map.remove(v);
									}
								}
							}
							Expression::Exponent(e) => {
								factor = Fraction::from(1);
								if let (Expression::Variable(v), Expression::Numeral(n)) =
									(e.base.as_ref(), e.exponent.as_ref())
								{
									let power = if let Some(n2) = variable_exponent_map.get_mut(v) {
										Some(cmp::min(*n2, n.clone()))
									} else {
										None
									};
									variable_exponent_map.clear();
									if let Some(n) = power {
										variable_exponent_map.insert(v.clone(), n);
									}
								} else {
									variable_exponent_map.clear();
								}
							}
							Expression::Variable(v) => {
								let power = if let Some(n) = variable_exponent_map.get_mut(v) {
									Some(cmp::min(*n, 1.into()))
								} else {
									None
								};
								variable_exponent_map.clear();
								if let Some(n) = power {
									variable_exponent_map.insert(v.clone(), n);
								}
							}
							Expression::Numeral(f) => {
								factor = fraction_gcd(&factor, &f);
								variable_exponent_map.clear();
							}
							_ => {
								factor = Fraction::from(1);
								variable_exponent_map.clear();
							}
						}
					}
					// factorize
					if factor != 1.into() || variable_exponent_map.keys().len() > 0 {
						for t in s.terms.iter_mut() {
							match t.as_mut() {
								Expression::Product(p) => {
									p.coefficient = p.coefficient / factor.clone();
									for var in variable_vec.clone() {
										if let Some(power) = variable_exponent_map.get_mut(&var) {
											p.factors = p.variable_decrement(&var, &power);
										}
									}
									t.simplify();
								}
								Expression::Exponent(e) => {
									if let (Expression::Variable(v), Expression::Numeral(n)) =
										(e.base.as_ref(), e.exponent.as_ref())
									{
										if let Some(power) = variable_exponent_map.get_mut(v) {
											let new_power = *power - n.clone();
											if new_power.is_zero() {
												*t = Box::new(Expression::Numeral(Fraction::from(1)));
											} else if new_power.is_one() {
												*t = Box::new(Expression::Variable(v.clone()));
											} else {
												*power = new_power;
											}
										}
									}
								}
								Expression::Variable(v) => {
									let pow: Fraction;
									if let Some(power) = variable_exponent_map.get_mut(v) {
										pow = *power - 1.into();
									} else {
										panic!("Unexpected factorization of variable encountered in product-variable")
									}
									if pow.is_zero() {
										*t = Box::new(Expression::Numeral(Fraction::from(1)));
									} else {
										let mut exp = Expression::Exponent(Exponent {
											base: Box::new(Expression::Variable(v.clone())),
											exponent: Box::new(Expression::Numeral(pow)),
										});
										exp.simplify();
										*t = Box::new(exp);
									}
								}
								Expression::Numeral(f) => {
									*f = *f / factor.clone();
								}
								_ => {
									panic!("Unexpected factorization of variable encountered in product")
								}
							}
						}
						// collect factorized variables
						let mut factors: Vec<Box<Expression>> = Vec::new();
						for var in variable_vec {
							if let Some(pow) = variable_exponent_map.get(&var) {
								if pow.is_one() {
									factors.push(Box::new(Expression::Variable(var)));
								} else {
									let exp = Expression::Exponent(Exponent {
										base: Box::new(Expression::Variable(var)),
										exponent: Box::new(Expression::Numeral(pow.clone())),
									});
									factors.push(Box::new(exp));
								}
							}
						}
						let mut sum = Expression::Sum(s.clone());
						sum.expand_and_simplify();
						factors.push(Box::new(sum));
						*self = Expression::Product(Product {
							coefficient: factor,
							factors,
						});
					}
				}
				Expression::Variable(v) => {
					// check if variable in remaining terms
					for t in terms {
						match t.as_ref() {
							Expression::Variable(_) => {
								// simplification should have prevented two of the same variables in a sum
								return;
							}
							Expression::Exponent(e) => {
								// only work for x^n where n is a numeral at the moment
								// doesn't work for (xy)^n
								if let (Expression::Variable(v2), Expression::Numeral(n)) =
									(e.base.as_ref(), e.exponent.as_ref())
								{
									if v != v2 || n <= &1.into() {
										return;
									}
								}
							}
							Expression::Product(p) => {
								if !p.has_variable(v) {
									return;
								}
								let power = p.variable_pow(v).unwrap();
								if power < 1.into() {
									return;
								}
							}
							_ => return,
						}
					}
					// factorize
					let mut terms: Vec<Box<Expression>> =
						vec![Box::new(Expression::Numeral(Fraction::from(1)))];
					let mut terms_iter = s.terms.iter();
					terms_iter.next(); // can skip first term
					for t in terms_iter {
						match t.as_ref() {
							Expression::Exponent(e) => {
								if let Expression::Numeral(f) = e.exponent.as_ref() {
									let mut exp = Expression::Exponent(Exponent {
										base: Box::new(Expression::Variable(v.clone())),
										exponent: Box::new(Expression::Numeral(*f - 1.into())),
									});
									exp.simplify();
									terms.push(Box::new(exp))
								} else {
									panic!(
										"Unexpected factorization of variable encountered: exponent is not a numeral"
									)
								}
							}
							Expression::Product(p) => {
								let factors = p.variable_decrement(&v, &1.into());
								let mut exp = Expression::Product(Product {
									coefficient: p.coefficient.clone(),
									factors,
								});
								exp.simplify();
								terms.push(Box::new(exp))
							}
							_ => {
								panic!("Unexpected factorization of variable encountered")
							}
						}
					}
					let mut sum = Expression::Sum(Sum { terms });
					sum.expand_and_simplify();
					let mut p = Product {
						coefficient: Fraction::from(1),
						factors: vec![Box::new(Expression::Variable(v.clone())), Box::new(sum)],
					};
					p.simplify();
					*self = Expression::Product(p);
				}
				Expression::Exponent(e) => {
					if let (Expression::Variable(v), Expression::Numeral(n)) =
						(e.base.as_ref(), e.exponent.as_ref())
					{
						if n.is_negative() {
							return;
						}
						let mut power = n.clone();
						for t in terms {
							match t.as_ref() {
								Expression::Exponent(e2) => {
									if let (Expression::Variable(v2), Expression::Numeral(n2)) =
										(e2.base.as_ref(), e2.exponent.as_ref())
									{
										if v != v2 || n2.is_negative() {
											return;
										}
										power = cmp::min(power, n2.clone());
									}
								}
								Expression::Variable(v2) => {
									if v != v2 {
										return;
									}
									if power < 1.into() {
										return;
									}
									power = cmp::min(power, 1.into());
								}
								Expression::Product(p) => {
									if !p.has_variable(v) {
										return;
									}
									let p_power = p.variable_pow(v).unwrap();
									if p_power.is_negative() {
										return;
									}
									power = cmp::min(power, p_power);
								}
								_ => return,
							}
						}
						let mut terms: Vec<Box<Expression>> = Vec::new();
						for t in s.terms.iter() {
							match t.as_ref() {
								Expression::Exponent(e) => {
									if let Expression::Numeral(f) = e.exponent.as_ref() {
										let new_power = *f - power.clone();
										if new_power.is_zero() {
											terms.push(Box::new(Expression::Numeral(Fraction::from(1))));
										} else if new_power.is_one() {
											terms.push(Box::new(Expression::Variable(v.clone())));
										} else {
											let mut exp = Expression::Exponent(Exponent {
												base: Box::new(Expression::Variable(v.clone())),
												exponent: Box::new(Expression::Numeral(new_power)),
											});
											exp.simplify();
											terms.push(Box::new(exp))
										}
									} else {
										panic!(
											"Unexpected factorization of variable encountered: exponent is not a numeral"
										)
									}
								}
								Expression::Variable(v2) => {
									assert!(v == v2);
									if power > 1.into() {
										panic!("Unexpected factorization of variable encountered: power should be at least 1 when we get here")
									}
									let new_power: Fraction = Fraction::from(1) - power;
									if new_power.is_zero() {
										terms.push(Box::new(Expression::Numeral(Fraction::from(1))));
									} else if new_power.is_one() {
										terms.push(Box::new(Expression::Variable(v.clone())));
									} else {
										let mut exp = Expression::Exponent(Exponent {
											base: Box::new(Expression::Variable(v.clone())),
											exponent: Box::new(Expression::Numeral(new_power)),
										});
										exp.simplify();
										terms.push(Box::new(exp))
									}
								}
								Expression::Product(p) => {
									let factors = p.variable_decrement(&v, &power);
									let mut exp = Expression::Product(Product {
										coefficient: p.coefficient.clone(),
										factors,
									});
									exp.simplify();
									terms.push(Box::new(exp))
								}
								_ => {
									panic!("Unexpected factorization of variable encountered")
								}
							}
						}
						let mut factor = Expression::Exponent(Exponent {
							base: Box::new(Expression::Variable(v.clone())),
							exponent: Box::new(Expression::Numeral(power.clone())),
						});
						factor.simplify();
						let mut sum = Expression::Sum(Sum { terms });
						sum.expand_and_simplify();
						let mut p = Product {
							coefficient: Fraction::from(1),
							factors: vec![Box::new(factor), Box::new(sum)],
						};
						p.simplify();
						*self = Expression::Product(p);
					}
				}
				_ => (),
			}
		}
	}

	// combine sums of quotients
	// only work for numbers at the moment
	pub fn combine_fraction(&mut self) -> () {
		if let Expression::Sum(s) = self {
			let mut denominator = Fraction::from(1);
			// get lcm
			for t in &s.terms {
				match t.as_ref() {
					Expression::Quotient(q) => {
						if let Expression::Numeral(n) = q.denominator.as_ref() {
							println!("denominator-pre: {}, {}", denominator, n);
							denominator = fraction_lcm(&denominator, &n);
							println!("denominator-post: {}", denominator);
						} else {
							panic!("Unexpected denominator in quotient")
						}
					}
					Expression::Numeral(n) => {
						denominator = fraction_lcm(&denominator, &Fraction::from(n.denominator as i32));
					}
					// only handle - n/d at the moment.
					Expression::Product(p) => {
						if p.coefficient.is_negative_one() && p.factors.len() == 1 {
							if let Expression::Quotient(q) = p.factors[0].as_ref() {
								if let Expression::Numeral(n) = q.denominator.as_ref() {
									denominator = fraction_lcm(&denominator, &n);
								} else {
									panic!("Unexpected denominator in quotient")
								}
							}
						}
					}
					_ => (),
				}
			}
			// combine into fraction
			let mut terms: Vec<Box<Expression>> = Vec::new();
			if denominator.is_one() {
				return;
			}
			for t in s.terms.iter_mut() {
				match t.as_mut() {
					Expression::Product(p) => {
						if p.coefficient.is_negative_one() && p.factors.len() == 1 {
							if let Expression::Quotient(q) = p.factors[0].as_ref() {
								if let Expression::Numeral(n) = q.denominator.as_ref() {
									let multiple = denominator.clone() / n.clone();
									let mut exp = Expression::Product(Product {
										coefficient: multiple.negative(),
										factors: vec![q.numerator.clone()],
									});
									exp.expand_and_simplify();
									terms.push(Box::new(exp));
								} else {
									panic!("Unexpected denominator in quotient")
								}
							} else {
								terms.push(Box::new(Expression::Product(Product {
									coefficient: p.coefficient * denominator.clone(),
									factors: p.factors.clone(),
								})));
							}
						} else {
							let mut exp = Expression::Product(Product {
								coefficient: p.coefficient * denominator.clone(),
								factors: p.factors.clone(),
							});
							exp.expand_and_simplify();
							terms.push(Box::new(exp));
						}
					}
					Expression::Quotient(q) => {
						if let Expression::Numeral(n) = q.denominator.as_ref() {
							let multiple = denominator.clone() / n.clone();
							let mut exp = Expression::Product(Product {
								coefficient: multiple,
								factors: vec![q.numerator.clone()],
							});
							exp.expand_and_simplify();
							terms.push(Box::new(exp));
						} else {
							panic!("Unexpected denominator in quotient")
						}
					}
					Expression::Exponent(_) => {
						terms.push(Box::new(Expression::Product(Product {
							coefficient: denominator.clone(),
							factors: vec![t.clone()],
						})));
					}
					Expression::Variable(_) => {
						terms.push(Box::new(Expression::Product(Product {
							coefficient: denominator.clone(),
							factors: vec![t.clone()],
						})));
					}
					Expression::Numeral(n) => {
						terms.push(Box::new(Expression::Numeral(*n * denominator.clone())));
					}
					_ => {
						panic!("Unexpected sum in sum")
					}
				}
			}
			let sum = Expression::Sum(Sum { terms });
			let mut q = Expression::Quotient(Quotient {
				numerator: Box::new(sum),
				denominator: Box::new(Expression::Numeral(denominator)),
			});
			q.simplify();
			*self = q;
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
