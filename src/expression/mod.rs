mod sum;
pub use sum::Sum;
mod product;
pub use product::Product;
mod numeral;
pub use numeral::Fraction;
mod variable;

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

pub trait SubIn {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a>;
}

impl SubIn for Expression<'_> {
	fn sub_in<'a>(&'a self, var: &str, val: &Expression<'a>) -> Expression<'a> {
		match self {
			Expression::Sum(s) => s.sub_in(var, val),
			Expression::Product(p) => p.sub_in(var, val),
			Expression::Numeral(n) => n.sub_in(var, val),
			Expression::Variable(v) => v.sub_in(var, val),
		}
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
			Expression::Numeral(_) => (),
			Expression::Variable(_) => (),
		}
	}
}
