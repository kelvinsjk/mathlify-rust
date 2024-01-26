use crate::expression::{Exponent, Expression, Fraction, SubIn};
use std::{collections::HashMap, fmt};

use super::fraction_gcd;

#[cfg(test)]
mod tests {
	use crate::expression::*;
	use crate::*;

	#[test]
	fn display() {
		assert_eq!(prod!().to_string(), "1");
		assert_eq!(prod!(0, "x").to_string(), "0");
		assert_eq!(
			prod_verbatim!(2, "x", Fraction::new(1, 2), "y").to_string(),
			"2x\\frac{1}{2}y"
		);
		assert_eq!(
			prod!(Fraction::new(1, 3), "x", "y").to_string(),
			"\\frac{1}{3}xy"
		);
		// Sec 1a, Page 60, Q6d
		assert_eq!(quotient!(prod!("x", "y"), 3).to_string(), "\\frac{xy}{3}");
	}

	#[test]
	fn sub_in() {
		// Sec 1a, Page 60, Q6a,b
		let exp = prod!(3, "x", "y");
		let exp = exp.sub_in("x", &(5 as i32).into());
		let exp = exp.sub_in("y", &(-2 as i32).into());
		assert_eq!(exp.to_string(), "- 30");
	}
}

#[derive(Debug, Clone)]
pub struct Quotient {
	pub numerator: Box<Expression>,
	pub denominator: Box<Expression>,
}

impl fmt::Display for Quotient {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Expression::Numeral(n) = self.denominator.as_ref() {
			if n.is_one() {
				return write!(f, "{}", self.numerator);
			}
		}
		write!(f, "\\frac{{{}}}{{{}}}", self.numerator, self.denominator)
	}
}

impl SubIn for Quotient {
	fn sub_in(&self, var: &str, val: &Expression) -> Expression {
		let mut e = Expression::Quotient(Quotient {
			numerator: Box::new(self.numerator.sub_in(var, val)),
			denominator: Box::new(self.denominator.sub_in(var, val)),
		});
		e.simplify();
		e
	}
}

// only works for numeral numerator
impl Quotient {
	pub fn abs(&self) -> Quotient {
		if let Expression::Numeral(n) = self.numerator.as_ref() {
			if n.is_negative() {
				return Quotient {
					numerator: Box::new(n.abs().into()),
					denominator: Box::new(*self.denominator.clone()),
				};
			}
		}
		self.clone()
	}

	// prod (3x)/n -> take gcd
	pub fn simplify(&mut self) -> () {
		self.numerator.simplify();
		self.denominator.simplify();
		if let (Expression::Product(p), Expression::Numeral(n)) =
			(self.numerator.as_mut(), self.denominator.as_mut())
		{
			let gcd = fraction_gcd(&p.coefficient, n);
			if !gcd.is_one() {
				p.coefficient = p.coefficient / gcd.clone();
				*n = *n / gcd;
			}
		} else if let (Expression::Numeral(n), Expression::Product(p)) =
			(self.numerator.as_mut(), self.denominator.as_mut())
		{
			let gcd = fraction_gcd(&p.coefficient, n);
			if !gcd.is_one() {
				p.coefficient = p.coefficient / gcd.clone();
				*n = *n / gcd;
			}
		} else if let (Expression::Product(p1), Expression::Product(p2)) =
			(self.numerator.as_mut(), self.denominator.as_mut())
		{
			let gcd = fraction_gcd(&p1.coefficient, &p2.coefficient);
			if !gcd.is_one() {
				p1.coefficient = p1.coefficient / gcd.clone();
				p2.coefficient = p2.coefficient / gcd;
			}
		}
		// cancels if common terms appear in both numerator and denominator
		// assumes each key only appears at most once in numerator, denominator
		let mut exponent_map: HashMap<String, (Option<Fraction>, Option<Fraction>)> = HashMap::new();
		match self.numerator.as_ref() {
			Expression::Sum(s) => {
				exponent_map.insert(s.lexical_string(), (Some(1.into()), None));
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(n) = e.exponent.as_ref() {
					let key = if let Expression::Sum(s) = e.base.as_ref() {
						s.lexical_string()
					} else {
						e.base.to_string()
					};
					exponent_map.insert(key, (Some(n.clone()), None));
				}
			}
			Expression::Variable(v) => {
				exponent_map.insert(v.to_string(), (Some(1.into()), None));
			}
			Expression::Product(p) => {
				for f in p.factors.clone() {
					match f.as_ref() {
						Expression::Sum(s) => {
							exponent_map.insert(s.lexical_string(), (Some(1.into()), None));
						}
						Expression::Exponent(e) => {
							if let Expression::Numeral(n) = e.exponent.as_ref() {
								let key = if let Expression::Sum(s) = e.base.as_ref() {
									s.lexical_string()
								} else {
									e.base.to_string()
								};
								exponent_map.insert(key, (Some(n.clone()), None));
							}
						}
						Expression::Variable(v) => {
							exponent_map.insert(v.to_string(), (Some(1.into()), None));
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
		match self.denominator.as_ref() {
			Expression::Sum(s) => {
				let key = s.lexical_string();
				let val = exponent_map.get_mut(&key);
				if let Some((_, d)) = val {
					*d = Some(1.into());
				} else {
					exponent_map.insert(key, (None, Some(1.into())));
				}
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(n) = e.exponent.as_ref() {
					let key = if let Expression::Sum(s) = e.base.as_ref() {
						s.lexical_string()
					} else {
						e.base.to_string()
					};
					let val = exponent_map.get_mut(&key);
					if let Some((_, d)) = val {
						*d = Some(n.clone());
					} else {
						exponent_map.insert(key, (None, Some(n.clone())));
					}
				}
			}
			Expression::Variable(v) => {
				let val = exponent_map.get_mut(&v.to_string());
				if let Some((_, d)) = val {
					*d = Some(1.into());
				} else {
					exponent_map.insert(v.to_string(), (None, Some(1.into())));
				}
			}
			Expression::Product(p) => {
				for f in p.factors.clone() {
					match f.as_ref() {
						Expression::Sum(s) => {
							let key = s.lexical_string();
							let val = exponent_map.get_mut(&key);
							if let Some((_, d)) = val {
								*d = Some(1.into());
							} else {
								exponent_map.insert(key, (None, Some(1.into())));
							}
						}
						Expression::Exponent(e) => {
							if let Expression::Numeral(n) = e.exponent.as_ref() {
								let key = if let Expression::Sum(s) = e.base.as_ref() {
									s.lexical_string()
								} else {
									e.base.to_string()
								};
								let val = exponent_map.get_mut(&key);
								if let Some((_, d)) = val {
									*d = Some(n.clone());
								} else {
									exponent_map.insert(key, (None, Some(n.clone())));
								}
							}
						}
						Expression::Variable(v) => {
							let val = exponent_map.get_mut(&v.to_string());
							if let Some((_, d)) = val {
								*d = Some(1.into());
							} else {
								exponent_map.insert(v.to_string(), (None, Some(1.into())));
							}
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
		match self.numerator.as_mut() {
			Expression::Sum(s) => {
				let key = s.lexical_string();
				let (n, d) = exponent_map.get_mut(&key).unwrap();
				if let (Some(num), Some(den)) = (n, d) {
					if num <= den {
						self.numerator = Box::new(Expression::Numeral(1.into()));
					} else {
						let pow = num.clone() - den.clone();
						if pow.is_one() {
							self.numerator = Box::new(Expression::Sum(s.clone()));
						} else {
							self.numerator = Box::new(Expression::Exponent(Exponent {
								base: Box::new(Expression::Sum(s.clone())),
								exponent: Box::new(Expression::Numeral(pow)),
							}));
						}
					}
				}
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(_) = e.exponent.as_ref() {
					let key = if let Expression::Sum(s) = e.base.as_ref() {
						s.lexical_string()
					} else {
						e.base.to_string()
					};
					let (num, den) = exponent_map.get_mut(&key).unwrap();
					if let (Some(num), Some(den)) = (num, den) {
						if num <= den {
							self.numerator = Box::new(Expression::Numeral(1.into()));
						} else {
							let pow = num.clone() - den.clone();
							if pow.is_one() {
								self.numerator = e.base.clone();
							} else {
								self.numerator = Box::new(Expression::Exponent(Exponent {
									base: e.base.clone(),
									exponent: Box::new(Expression::Numeral(pow)),
								}));
							}
						}
					}
				}
			}
			Expression::Variable(v) => {
				let (num, den) = exponent_map.get_mut(&v.to_string()).unwrap();
				if let (Some(num), Some(den)) = (num, den) {
					if num <= den {
						self.numerator = Box::new(Expression::Numeral(1.into()));
					} else {
						let pow = num.clone() - den.clone();
						if pow.is_one() {
							self.numerator = Box::new(Expression::Variable(v.clone()));
						} else {
							self.numerator = Box::new(Expression::Exponent(Exponent {
								base: Box::new(Expression::Variable(v.clone())),
								exponent: Box::new(Expression::Numeral(pow)),
							}));
						}
					}
				}
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					match f.as_mut() {
						Expression::Sum(s) => {
							let key = s.lexical_string();
							let (n, d) = exponent_map.get_mut(&key).unwrap();
							if let (Some(num), Some(den)) = (n, d) {
								if num <= den {
									*f = Box::new(Expression::Numeral(1.into()));
								} else {
									let pow = num.clone() - den.clone();
									if pow.is_one() {
										*f = Box::new(Expression::Sum(s.clone()));
									} else {
										*f = Box::new(Expression::Exponent(Exponent {
											base: Box::new(Expression::Sum(s.clone())),
											exponent: Box::new(Expression::Numeral(pow)),
										}));
									}
								}
							}
						}
						Expression::Exponent(e) => {
							if let Expression::Numeral(_) = e.exponent.as_ref() {
								let key = if let Expression::Sum(s) = e.base.as_ref() {
									s.lexical_string()
								} else {
									e.base.to_string()
								};
								let (num, den) = exponent_map.get_mut(&key).unwrap();
								if let (Some(num), Some(den)) = (num, den) {
									if num <= den {
										*f = Box::new(Expression::Numeral(1.into()));
									} else {
										let pow = num.clone() - den.clone();
										if pow.is_one() {
											*f = e.base.clone();
										} else {
											*f = Box::new(Expression::Exponent(Exponent {
												base: e.base.clone(),
												exponent: Box::new(Expression::Numeral(pow)),
											}));
										}
									}
								}
							}
						}
						Expression::Variable(v) => {
							let (num, den) = exponent_map.get_mut(&v.to_string()).unwrap();
							if let (Some(num), Some(den)) = (num, den) {
								if num <= den {
									*f = Box::new(Expression::Numeral(1.into()));
								} else {
									let pow = num.clone() - den.clone();
									if pow.is_one() {
										*f = Box::new(Expression::Variable(v.clone()));
									} else {
										*f = Box::new(Expression::Exponent(Exponent {
											base: Box::new(Expression::Variable(v.clone())),
											exponent: Box::new(Expression::Numeral(pow)),
										}));
									}
								}
							}
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
		match self.denominator.as_mut() {
			Expression::Sum(s) => {
				let key = s.lexical_string();
				let (n, d) = exponent_map.get_mut(&key).unwrap();
				if let (Some(num), Some(den)) = (n, d) {
					if num >= den {
						self.denominator = Box::new(Expression::Numeral(1.into()));
					} else {
						let pow = den.clone() - num.clone();
						if pow.is_one() {
							self.denominator = Box::new(Expression::Sum(s.clone()));
						} else {
							self.denominator = Box::new(Expression::Exponent(Exponent {
								base: Box::new(Expression::Sum(s.clone())),
								exponent: Box::new(Expression::Numeral(pow)),
							}));
						}
					}
				}
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(_) = e.exponent.as_ref() {
					let key = if let Expression::Sum(s) = e.base.as_ref() {
						s.lexical_string()
					} else {
						e.base.to_string()
					};
					let (num, den) = exponent_map.get_mut(&key).unwrap();
					if let (Some(num), Some(den)) = (num, den) {
						if num >= den {
							self.denominator = Box::new(Expression::Numeral(1.into()));
						} else {
							let pow = den.clone() - num.clone();
							if pow.is_one() {
								self.denominator = e.base.clone();
							} else {
								self.denominator = Box::new(Expression::Exponent(Exponent {
									base: e.base.clone(),
									exponent: Box::new(Expression::Numeral(pow)),
								}));
							}
						}
					}
				}
			}
			Expression::Variable(v) => {
				let (num, den) = exponent_map.get_mut(&v.to_string()).unwrap();
				if let (Some(num), Some(den)) = (num, den) {
					if num >= den {
						self.denominator = Box::new(Expression::Numeral(1.into()));
					} else {
						let pow = den.clone() - num.clone();
						if pow.is_one() {
							self.denominator = Box::new(Expression::Variable(v.clone()));
						} else {
							self.denominator = Box::new(Expression::Exponent(Exponent {
								base: Box::new(Expression::Variable(v.clone())),
								exponent: Box::new(Expression::Numeral(pow)),
							}));
						}
					}
				}
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					match f.as_mut() {
						Expression::Sum(s) => {
							let key = s.lexical_string();
							let (n, d) = exponent_map.get_mut(&key).unwrap();
							if let (Some(num), Some(den)) = (n, d) {
								if num >= den {
									*f = Box::new(Expression::Numeral(1.into()));
								} else {
									let pow = den.clone() - num.clone();
									if pow.is_one() {
										*f = Box::new(Expression::Sum(s.clone()));
									} else {
										*f = Box::new(Expression::Exponent(Exponent {
											base: Box::new(Expression::Sum(s.clone())),
											exponent: Box::new(Expression::Numeral(pow)),
										}));
									}
								}
							}
						}
						Expression::Exponent(e) => {
							if let Expression::Numeral(_) = e.exponent.as_ref() {
								let key = if let Expression::Sum(s) = e.base.as_ref() {
									s.lexical_string()
								} else {
									e.base.to_string()
								};
								let (num, den) = exponent_map.get_mut(&key).unwrap();
								if let (Some(num), Some(den)) = (num, den) {
									if num >= den {
										*f = Box::new(Expression::Numeral(1.into()));
									} else {
										let pow = den.clone() - num.clone();
										if pow.is_one() {
											*f = e.base.clone();
										} else {
											*f = Box::new(Expression::Exponent(Exponent {
												base: e.base.clone(),
												exponent: Box::new(Expression::Numeral(pow)),
											}));
										}
									}
								}
							}
						}
						Expression::Variable(v) => {
							let (num, den) = exponent_map.get_mut(&v.to_string()).unwrap();
							if let (Some(num), Some(den)) = (num, den) {
								if num >= den {
									*f = Box::new(Expression::Numeral(1.into()));
								} else {
									let pow = den.clone() - num.clone();
									if pow.is_one() {
										*f = Box::new(Expression::Variable(v.clone()));
									} else {
										*f = Box::new(Expression::Exponent(Exponent {
											base: Box::new(Expression::Variable(v.clone())),
											exponent: Box::new(Expression::Numeral(pow)),
										}));
									}
								}
							}
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
		self.numerator.simplify();
		self.denominator.simplify();
	}
}
