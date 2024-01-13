pub mod sum;
pub use sum::Sum;
pub mod product;
pub use product::Product;
pub mod numeral;
pub use numeral::Fraction;
pub mod exponent;
pub mod variable;
pub use exponent::Exponent;
use std::convert::TryInto;

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
			let p = Product { coefficient: Fraction::from(1), factors: _factors, fraction_mode: false };
			Expression::Product(p)
		}
	};
}

#[macro_export]
macro_rules! quotient {
	( $a:expr, $b:expr ) => {{
		let mut p = Product {
			coefficient: Fraction::from(1),
			factors: vec![
				Box::new($a.into()),
				Box::new(Expression::Exponent(Exponent {
					base: Box::new($b.into()),
					exponent: Box::new((-1).into()),
				})),
			],
			fraction_mode: true,
		};
		p.simplify();
		Expression::Product(p)
	}};
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
			p.simplify();
			Expression::Product(p)
		}
	};
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
pub enum Expression<'a> {
	Sum(Sum<'a>),
	Product(Product<'a>),
	Exponent(Exponent<'a>),
	Numeral(Fraction),
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
			Expression::Exponent(e) => write!(f, "{}", e),
		}
	}
}

pub trait SubIn {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a>;
}

impl SubIn for Expression<'_> {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		let mut r = match self {
			Expression::Sum(s) => s.sub_in(var, val),
			Expression::Product(p) => p.sub_in(var, val),
			Expression::Numeral(n) => n.sub_in(var, val),
			Expression::Variable(v) => v.sub_in(var, val),
			Expression::Exponent(e) => e.sub_in(var, val),
		};
		r.simplify();
		r
	}
}

impl Expression<'_> {
	pub fn simplify(&mut self) -> () {
		// remove singleton sums and products
		match self {
			Expression::Sum(s) => {
				for t in s.terms.iter_mut() {
					t.simplify();
				}
				if s.terms.len() == 1 {
					*self = s.terms[0].as_mut().clone();
				}
			}
			Expression::Product(p) => {
				for f in p.factors.iter_mut() {
					f.simplify();
				}
				if p.factors.len() == 0 {
					*self = Expression::Numeral(p.coefficient.clone());
				} else if p.factors.len() == 1 && p.coefficient == 1.into() {
					*self = p.factors[0].as_mut().clone();
				}
			}
			// number^number
			// remove power 1
			// exponent of products become product of exponents
			Expression::Exponent(e) => {
				e.base.simplify();
				e.exponent.simplify();
				// remove power 1
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
									fraction_mode: p.fraction_mode,
								};
								p.simplify();
								*self = Expression::Product(p);
								self.simplify();
							}
						} else if let Expression::Product(p) = e.base.as_ref() {
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
								fraction_mode: p.fraction_mode,
							};
							p.simplify();
							*self = Expression::Product(p);
							self.simplify();
						}
					}
				}
			}
			// variable, numeral
			_ => (),
		}
	}
}

impl TryInto<Fraction> for Expression<'_> {
	type Error = ();
	fn try_into(self) -> Result<Fraction, ()> {
		match self {
			Expression::Numeral(n) => Ok(n),
			_ => Err(()),
		}
	}
}
impl TryInto<String> for Expression<'_> {
	type Error = ();
	fn try_into(self) -> Result<String, ()> {
		match self {
			Expression::Variable(v) => Ok(v.to_string()),
			_ => Err(()),
		}
	}
}
impl<'a> TryInto<Sum<'a>> for Expression<'a> {
	type Error = ();
	fn try_into(self) -> Result<Sum<'a>, ()> {
		match self {
			Expression::Sum(s) => Ok(s),
			_ => Err(()),
		}
	}
}
impl<'a> TryInto<Product<'a>> for Expression<'a> {
	type Error = ();
	fn try_into(self) -> Result<Product<'a>, ()> {
		match self {
			Expression::Product(p) => Ok(p),
			_ => Err(()),
		}
	}
}
impl<'a> TryInto<Exponent<'a>> for Expression<'a> {
	type Error = ();
	fn try_into(self) -> Result<Exponent<'a>, ()> {
		match self {
			Expression::Exponent(e) => Ok(e),
			_ => Err(()),
		}
	}
}
