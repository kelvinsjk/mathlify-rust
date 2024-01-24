use mathlify::expression::*;
use mathlify::*;

#[test]
fn expansion() {
	// Sec 1A Worksheet 4c, page 68, Q1a,c,d,g
	let mut exp = sum!(prod!(16, "x"), 12);
	assert_eq!(exp.to_string(), "16x + 12");
	exp.factorize();
	assert_eq!(exp.to_string(), "4\\left( 4x + 3 \\right)");
	let mut exp = sum!(10, prod!(-15, "x"));
	assert_eq!(exp.to_string(), "10 - 15x");
	exp.factorize();
	assert_eq!(exp.to_string(), "5\\left( 2 - 3x \\right)");
	let mut exp = sum!(prod!(-33, "x"), -44);
	assert_eq!(exp.to_string(), "- 33x - 44");
	exp.factorize();
	assert_eq!(exp.to_string(), "- 11\\left( 3x + 4 \\right)");
	let mut exp = sum!(prod!(24, "x"), prod!(-27, "y"), prod!(3, "z"));
	assert_eq!(exp.to_string(), "24x - 27y + 3z");
	exp.factorize();
	assert_eq!(exp.to_string(), "3\\left( 8x - 9y + z \\right)");

	let mut exp = sum!(prod!(18, "x"), prod!(9, sum!("a", "b")));
	assert_eq!(exp.to_string(), "18x + 9\\left( a + b \\right)");
	exp.factorize();
	assert_eq!(exp.to_string(), "9\\left( 2x + a + b \\right)");

	let mut exp = sum!("x", exp!("x", 2), exp!("x", Fraction::new(3, 2)));
	assert_eq!(exp.to_string(), "x + x^2 + x^{\\frac{3}{2}}");
	exp.factorize();
	assert_eq!(
		exp.to_string(),
		"x\\left( 1 + x + x^{\\frac{1}{2}} \\right)"
	);
	let mut exp = sum!("x", exp!("x", 2), exp!("x", Fraction::new(-1, 2)));
	exp.factorize();
	assert_eq!(exp.to_string(), "x + x^2 + x^{- \\frac{1}{2}}");
	let mut exp = sum!(exp!("x", 2), "x", exp!("x", Fraction::new(3, 2)));
	exp.factorize();
	assert_eq!(
		exp.to_string(),
		"x\\left( x + 1 + x^{\\frac{1}{2}} \\right)"
	);

	let mut exp = sum!(exp!("x", 2), "x", exp!("x", Fraction::new(1, 2)));
	exp.factorize();
	assert_eq!(
		exp.to_string(),
		"x^{\\frac{1}{2}}\\left( x^{\\frac{3}{2}} + x^{\\frac{1}{2}} + 1 \\right)"
	);
}
