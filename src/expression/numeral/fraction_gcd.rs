use super::gcd::{gcd, lcm};
use super::Fraction;

pub fn fraction_gcd(x: &Fraction, y: &Fraction) -> Fraction {
	assert!(!(x.is_zero() && y.is_zero()));
	let mut num = gcd(x.numerator, y.numerator);
	let den = lcm(x.denominator as i32, y.denominator as i32);
	if (x.is_negative() && (y.is_negative() || y.is_zero()))
		|| ((x.is_negative() || x.is_zero()) && y.is_negative())
	{
		num = -num;
	}
	Fraction::new(num, den)
}

pub fn fraction_lcm(x: &Fraction, y: &Fraction) -> Fraction {
	assert!(!(x.is_zero() && y.is_zero()));
	let num = lcm(x.numerator, y.numerator);
	let den = gcd(x.denominator as i32, y.denominator as i32);
	Fraction::new(num, den)
}
