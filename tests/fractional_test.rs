//use std::env;

use mathlify::expression::*;
use mathlify::*;

#[test]
fn fractional_expressions() {
	//env::set_var("RUST_BACKTRACE", "1");
	// Sec 1A Worksheet 4d, page 69, Q1d
	let mut exp = sum_verbatim!(
		prod!(Fraction::new(5, 6), "x"),
		prod!(Fraction::new(-3, 4), "y"),
		"z",
		prod!(Fraction::new(3, 4), "x"),
		prod!(-2, "y"),
		prod!(Fraction::new(1, 2), "z")
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{5}{6}x - \\frac{3}{4}y + z + \\frac{3}{4}x - 2y + \\frac{1}{2}z"
	);
	exp.simplify();
	assert_eq!(
		exp.to_string(),
		"\\frac{19}{12}x - \\frac{11}{4}y + \\frac{3}{2}z"
	);
	// Q2a
	let mut exp = prod!(
		Fraction::new(4, 5),
		sum!(prod!(2, sum!(prod!(11, "x"), 7)), -4, prod!(23, "x"))
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{4}{5}\\left( 2\\left( 11x + 7 \\right) - 4 + 23x \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "36x + 8");
	// Q3a,c
	let mut exp = sum!(1, quotient!(prod!(5, "x"), 14));
	assert_eq!(exp.to_string(), "1 + \\frac{5x}{14}");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{14 + 5x}{14}");
	let mut exp = sum!(Fraction::new(7, 8), prod!(-1, "x"));
	assert_eq!(exp.to_string(), "\\frac{7}{8} - x");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{7 - 8x}{8}");
	// Pg 70 Q4a,c,i
	let mut exp = sum!(quotient!(prod!(3, "x"), 4), quotient!(prod!(7, "x"), 12));
	assert_eq!(exp.to_string(), "\\frac{3x}{4} + \\frac{7x}{12}");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{4x}{3}");
	let mut exp = sum!("x", quotient!(sum!(prod!(5, "x"), -3), 6));
	assert_eq!(exp.to_string(), "x + \\frac{5x - 3}{6}");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{11x - 3}{6}");
	let mut exp = sum!(
		quotient!(sum!(prod!(9, "x"), 1), 6),
		prod!(-1, quotient!(sum!(prod!(10, "x"), -3), 7)),
		Fraction::new(1, 3)
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{9x + 1}{6} - \\frac{10x - 3}{7} + \\frac{1}{3}"
	);
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{3x + 39}{42}");
	//TODO: factorize and simplify
	// Pg 71 Q5a,c,h,i
	let mut exp = sum!(
		quotient!(sum!(prod!(5, "x"), "y"), 6),
		quotient!(sum!(prod!(4, "x"), prod!(9, "y")), 3)
	);
	assert_eq!(exp.to_string(), "\\frac{5x + y}{6} + \\frac{4x + 9y}{3}");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{13x + 19y}{6}");
	let mut exp = sum!(
		quotient!(sum!(prod!(3, "y"), prod!(-10, "x")), 4),
		prod!(-1, quotient!(sum!("x", prod!(2, "y")), 5))
	);
	assert_eq!(exp.to_string(), "\\frac{3y - 10x}{4} - \\frac{x + 2y}{5}");
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{7y - 54x}{20}");
	let mut exp = sum!(
		prod!(4, "y"),
		prod!(-1, quotient!(sum!(prod!(3, "x"), prod!(2, "y")), 7)),
		quotient!(sum!(prod!(2, "x"), prod!(-3, "y")), 4)
	);
	assert_eq!(
		exp.to_string(),
		"4y - \\frac{3x + 2y}{7} + \\frac{2x - 3y}{4}"
	);
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{83y + 2x}{28}");
	let mut exp = sum!(
		prod!(2, "x"),
		prod!(-1, "y"),
		quotient!(sum!(prod!(6, "x"), prod!(-9, "y")), 2),
		prod!(-1, quotient!(sum!(prod!(5, "x"), prod!(2, "y")), 8))
	);
	assert_eq!(
		exp.to_string(),
		"2x - y + \\frac{6x - 9y}{2} - \\frac{5x + 2y}{8}"
	);
	exp.combine_fraction();
	assert_eq!(exp.to_string(), "\\frac{35x - 46y}{8}");
}

#[test]
fn cancellation() {
	// Sec 2a worksheet 6a q1a,c,e,f
	let mut exp = quotient!(prod!(10, "a", "b"), prod!(100, "b", "c"));
	assert_eq!(exp.to_string(), "\\frac{10ab}{100bc}");
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{a}{10c}");
	let mut exp = quotient!(prod!(8, exp!("h", 2), "k"), prod!(2, "h", exp!("k", 2)));
	assert_eq!(exp.to_string(), "\\frac{8h^2k}{2hk^2}");
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{4h}{k}");
	let mut exp = quotient!(
		prod!(exp!("p", 3), sum!("p", prod!(4, "q"))),
		prod!(3, "p", exp!(sum!("p", prod!(4, "q")), 2))
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{p^3\\left( p + 4q \\right)}{3p\\left( p + 4q \\right)^2}"
	);
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{p^2}{3\\left( p + 4q \\right)}");
	let mut exp = quotient!(
		prod!(30, "p", sum!(prod!(2, "q"), "r")),
		prod!(24, exp!("p", 2), sum!(prod!(2, "r"), "q"))
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{30p\\left( 2q + r \\right)}{24p^2\\left( 2r + q \\right)}"
	);
	exp.simplify();
	assert_eq!(
		exp.to_string(),
		"\\frac{5\\left( 2q + r \\right)}{4p\\left( 2r + q \\right)}"
	);
	let mut exp = quotient!(
		prod!(30, "p", sum!(prod!(2, "q"), "r")),
		prod!(24, exp!("p", 2), sum!("r", prod!(2, "q")))
	);
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{5}{4p}");
	// Q2a,c,e,h
	let mut exp = quotient!("a", sum!(prod!(5, exp!("a", 2)), "a"));
	assert_eq!(exp.to_string(), "\\frac{a}{5a^2 + a}");
	exp.factorize_denominator();
	assert_eq!(exp.to_string(), "\\frac{a}{a\\left( 5a + 1 \\right)}");
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{1}{5a + 1}");
	let mut exp = quotient!(
		sum!(prod!(16, exp!("h", 2)), prod!(2, "h", "k")),
		prod!(6, exp!("h", 2))
	);
	assert_eq!(exp.to_string(), "\\frac{16h^2 + 2hk}{6h^2}");
	exp.factorize_numerator();
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{8h + k}{3h}");
	let mut exp = quotient!(sum!(prod!(4, "p"), prod!(4, "q")), exp!(sum!("p", "q"), 2));
	assert_eq!(exp.to_string(), "\\frac{4p + 4q}{\\left( p + q \\right)^2}");
	exp.factorize_numerator();
	exp.simplify();
	assert_eq!(exp.to_string(), "\\frac{4}{p + q}");
	let mut exp = quotient!(
		sum!(prod!(14, exp!("y", 3), "z"), prod!(-14, "x", exp!("y", 3))),
		prod!(7, "y", exp!(sum!("x", prod!(-1, "z")), 2))
	);
	assert_eq!(
		exp.to_string(),
		"\\frac{14y^3z - 14xy^3}{7y\\left( x - z \\right)^2}"
	);
	exp.factorize_numerator();
	exp.simplify();
	assert_eq!(
		exp.to_string(),
		"\\frac{2y^2\\left( z - x \\right)}{\\left( x - z \\right)^2}"
	);
	// TODO: z-x vs x-z
}
