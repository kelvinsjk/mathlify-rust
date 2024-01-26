use crate::expression::{fraction_lcm, Exponent, Expression, Fraction, Product};
use std::collections::HashMap;

// does not handle quotients/nested products/numerals within factors
// for exponents, only handle numerical power
pub fn product_lcm(a: &Product, b: &Expression) -> Product {
	let mut ordering: Vec<String> = Vec::new();
	let mut exponent_map: HashMap<String, Fraction> = HashMap::new();
	let mut expression_map: HashMap<String, Expression> = HashMap::new();
	for f in &a.factors {
		match f.as_ref() {
			Expression::Sum(s) => {
				exponent_map.insert(s.lexical_string(), Fraction::new(1, 1));
				expression_map.insert(s.lexical_string(), *f.clone());
				ordering.push(s.lexical_string());
			}
			Expression::Variable(v) => {
				exponent_map.insert(v.clone(), Fraction::new(1, 1));
				expression_map.insert(v.clone(), *f.clone());
				ordering.push(v.clone());
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(n) = e.exponent.as_ref() {
					match e.base.as_ref() {
						Expression::Sum(s) => {
							exponent_map.insert(s.lexical_string(), n.clone());
							expression_map.insert(s.lexical_string(), *e.base.clone());
							ordering.push(s.lexical_string());
						}
						Expression::Variable(v) => {
							exponent_map.insert(v.clone(), n.clone());
							expression_map.insert(v.clone(), *e.base.clone());
							ordering.push(v.clone());
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
	}

	let mut coefficient = a.coefficient.clone();
	match b {
		Expression::Sum(s) => {
			let val = exponent_map.get_mut(&s.lexical_string());
			if let Some(power) = val {
				*power = fraction_lcm(power, &1.into());
			} else {
				exponent_map.insert(s.lexical_string(), Fraction::new(1, 1));
				expression_map.insert(s.lexical_string(), b.clone());
				ordering.push(s.lexical_string());
			}
		}
		Expression::Variable(v) => {
			let val = exponent_map.get_mut(v);
			if let Some(power) = val {
				*power = fraction_lcm(power, &1.into());
			} else {
				exponent_map.insert(v.clone(), Fraction::new(1, 1));
				expression_map.insert(v.clone(), b.clone());
				ordering.push(v.clone());
			}
		}
		Expression::Exponent(e) => {
			if let Expression::Numeral(n) = e.exponent.as_ref() {
				match e.base.as_ref() {
					Expression::Sum(s) => {
						let val = exponent_map.get_mut(&s.lexical_string());
						if let Some(power) = val {
							*power = fraction_lcm(power, n);
						} else {
							exponent_map.insert(s.lexical_string(), n.clone());
							expression_map.insert(s.lexical_string(), *e.base.clone());
							ordering.push(s.lexical_string());
						}
					}
					Expression::Variable(v) => {
						let val = exponent_map.get_mut(v);
						if let Some(power) = val {
							*power = fraction_lcm(power, n);
						} else {
							exponent_map.insert(v.clone(), n.clone());
							expression_map.insert(v.clone(), *e.base.clone());
							ordering.push(v.clone());
						}
					}
					_ => {}
				}
			}
		}
		Expression::Product(p) => {
			coefficient = fraction_lcm(&coefficient, &p.coefficient);
			for f in &p.factors {
				match f.as_ref() {
					Expression::Sum(s) => {
						let val = exponent_map.get_mut(&s.lexical_string());
						if let Some(power) = val {
							*power = fraction_lcm(power, &1.into());
						} else {
							exponent_map.insert(s.lexical_string(), Fraction::new(1, 1));
							expression_map.insert(s.lexical_string(), b.clone());
							ordering.push(s.lexical_string());
						}
					}
					Expression::Variable(v) => {
						let val = exponent_map.get_mut(v);
						if let Some(power) = val {
							*power = fraction_lcm(power, &1.into());
						} else {
							exponent_map.insert(v.clone(), Fraction::new(1, 1));
							expression_map.insert(v.clone(), b.clone());
							ordering.push(v.clone());
						}
					}
					Expression::Exponent(e) => {
						if let Expression::Numeral(n) = e.exponent.as_ref() {
							match e.base.as_ref() {
								Expression::Sum(s) => {
									let val = exponent_map.get_mut(&s.lexical_string());
									if let Some(power) = val {
										*power = fraction_lcm(power, n);
									} else {
										exponent_map.insert(s.lexical_string(), n.clone());
										expression_map.insert(s.lexical_string(), *e.base.clone());
										ordering.push(s.lexical_string());
									}
								}
								Expression::Variable(v) => {
									let val = exponent_map.get_mut(v);
									if let Some(power) = val {
										*power = fraction_lcm(power, n);
									} else {
										exponent_map.insert(v.clone(), n.clone());
										expression_map.insert(v.clone(), *e.base.clone());
										ordering.push(v.clone());
									}
								}
								_ => {}
							}
						}
					}
					_ => {}
				}
			}
		}
		Expression::Numeral(n) => {
			coefficient = fraction_lcm(&coefficient, n);
		}
		_ => {}
	}

	let mut factors: Vec<Box<Expression>> = Vec::new();
	for key in ordering {
		let power = exponent_map.get(&key).unwrap();
		let exp = expression_map.get(&key).unwrap();
		if power.is_one() {
			factors.push(Box::new(exp.clone()));
		} else {
			factors.push(Box::new(Expression::Exponent(Exponent {
				base: Box::new(exp.clone()),
				exponent: Box::new(Expression::Numeral(power.clone())),
			})));
		}
	}
	let mut p = Product {
		coefficient,
		factors,
	};
	p.simplify();
	p
}

// does not handle quotients/nested products/numerals within factors
// for exponents, only handle numerical power
// given an lcm of exp and other expressions,
// return the diff lcm/exp as a product
pub fn lcm_diff(lcm: &Product, exp: &Expression) -> Product {
	let mut ordering: Vec<String> = Vec::new();
	let mut exponent_map: HashMap<String, Fraction> = HashMap::new();
	let mut expression_map: HashMap<String, Expression> = HashMap::new();
	for f in &lcm.factors {
		match f.as_ref() {
			Expression::Sum(s) => {
				exponent_map.insert(s.lexical_string(), Fraction::new(1, 1));
				expression_map.insert(s.lexical_string(), *f.clone());
				ordering.push(s.lexical_string());
			}
			Expression::Variable(v) => {
				exponent_map.insert(v.clone(), Fraction::new(1, 1));
				expression_map.insert(v.clone(), *f.clone());
				ordering.push(v.clone());
			}
			Expression::Exponent(e) => {
				if let Expression::Numeral(n) = e.exponent.as_ref() {
					match e.base.as_ref() {
						Expression::Sum(s) => {
							exponent_map.insert(s.lexical_string(), n.clone());
							expression_map.insert(s.lexical_string(), *e.base.clone());
							ordering.push(s.lexical_string());
						}
						Expression::Variable(v) => {
							exponent_map.insert(v.clone(), n.clone());
							expression_map.insert(v.clone(), *e.base.clone());
							ordering.push(v.clone());
						}
						_ => {}
					}
				}
			}
			_ => {}
		}
	}

	let mut coefficient = lcm.coefficient.clone();
	match exp {
		Expression::Sum(s) => {
			let val = exponent_map.get_mut(&s.lexical_string());
			if let Some(power) = val {
				*power = *power - 1.into();
			} else {
				panic!("lcm_diff: exp not in lcm (sum)");
			}
		}
		Expression::Variable(v) => {
			let val = exponent_map.get_mut(v);
			if let Some(power) = val {
				*power = *power - 1.into();
			} else {
				panic!("lcm_diff: exp not in lcm (var)");
			}
		}
		Expression::Exponent(e) => {
			if let Expression::Numeral(n) = e.exponent.as_ref() {
				match e.base.as_ref() {
					Expression::Sum(s) => {
						let val = exponent_map.get_mut(&s.lexical_string());
						if let Some(power) = val {
							*power = *power - *n;
						} else {
							panic!("lcm_diff: exp not in lcm (sum^n)");
						}
					}
					Expression::Variable(v) => {
						let val = exponent_map.get_mut(v);
						if let Some(power) = val {
							*power = *power - *n;
						} else {
							panic!("lcm_diff: exp not in lcm (var^n)");
						}
					}
					_ => {}
				}
			}
		}
		Expression::Product(p) => {
			coefficient = coefficient / p.coefficient;
			for f in &p.factors {
				match f.as_ref() {
					Expression::Sum(s) => {
						let val = exponent_map.get_mut(&s.lexical_string());
						if let Some(power) = val {
							*power = *power - 1.into();
						} else {
							panic!("lcm_diff: exp not in lcm (p.sum)");
						}
					}
					Expression::Variable(v) => {
						let val = exponent_map.get_mut(v);
						if let Some(power) = val {
							*power = *power - 1.into();
						} else {
							panic!("lcm_diff: exp not in lcm (p.var)");
						}
					}
					Expression::Exponent(e) => {
						if let Expression::Numeral(n) = e.exponent.as_ref() {
							match e.base.as_ref() {
								Expression::Sum(s) => {
									let val = exponent_map.get_mut(&s.lexical_string());
									if let Some(power) = val {
										*power = *power - *n;
									} else {
										panic!("lcm_diff: exp not in lcm (p.sum^n)");
									}
								}
								Expression::Variable(v) => {
									let val = exponent_map.get_mut(v);
									if let Some(power) = val {
										*power = *power - *n;
									} else {
										panic!("lcm_diff: exp not in lcm (p.var^n)");
									}
								}
								_ => {}
							}
						}
					}
					_ => {}
				}
			}
		}
		Expression::Numeral(n) => {
			coefficient = coefficient / *n;
		}
		_ => {}
	}

	let mut factors: Vec<Box<Expression>> = Vec::new();
	for key in ordering {
		let power = exponent_map.get(&key).unwrap();
		let exp = expression_map.get(&key).unwrap();
		if power.is_one() {
			factors.push(Box::new(exp.clone()));
		} else {
			factors.push(Box::new(Expression::Exponent(Exponent {
				base: Box::new(exp.clone()),
				exponent: Box::new(Expression::Numeral(power.clone())),
			})));
		}
	}
	let mut p = Product {
		coefficient,
		factors,
	};
	p.simplify();
	p
}
