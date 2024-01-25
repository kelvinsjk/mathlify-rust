use mathlify::expression::*;
use mathlify::*;

#[test]
fn factorize() {
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

	let mut exp = sum!(
		"x",
		prod!(2, exp!("x", 3)),
		exp!("x", 2),
		prod!(3, "x", "y")
	);
	exp.factorize();
	assert_eq!(exp.to_string(), "x\\left( 1 + 2x^2 + x + 3y \\right)");
	let mut exp = sum!(
		exp!("x", 2),
		"x",
		prod!(2, exp!("x", 3)),
		prod!(3, "x", "y")
	);
	exp.factorize();
	assert_eq!(exp.to_string(), "x\\left( x + 1 + 2x^2 + 3y \\right)");
}

#[test]
fn factorize_variables() {
	// Sec 1A Worksheet 4c, page 68, Q1e,f,h
	let mut exp = sum!(prod!(14, "a", "x"), prod!(6, "a", "y"));
	assert_eq!(exp.to_string(), "14ax + 6ay");
	exp.factorize();
	assert_eq!(exp.to_string(), "2a\\left( 7x + 3y \\right)");
	let mut exp = sum!(prod!(-21, "a", "x"), prod!(56, "a", "y"));
	assert_eq!(exp.to_string(), "- 21ax + 56ay");
	exp.factorize();
	assert_eq!(exp.to_string(), "7a\\left( - 3x + 8y \\right)");
	let mut exp = sum!(
		prod!(-8, "a", "x"),
		prod!(10, "b", "x"),
		prod!(12, "c", "x")
	);
	assert_eq!(exp.to_string(), "- 8ax + 10bx + 12cx");
	exp.factorize();
	assert_eq!(exp.to_string(), "2x\\left( - 4a + 5b + 6c \\right)");
	// Q10
	let mut exp = sum!(
		prod!(-34, exp!("a", 4), "b", exp!("x", 2)),
		prod!(-85, exp!("a", 3), exp!("b", 2), exp!("x", 2)),
		prod!(-68, exp!("a", 3), "b", exp!("c", 2), exp!("x", 2))
	);
	assert_eq!(exp.to_string(), "- 34a^4bx^2 - 85a^3b^2x^2 - 68a^3bc^2x^2");
	exp.factorize();
	assert_eq!(
		exp.to_string(),
		"- 17a^3bx^2\\left( 2a + 5b + 4c^2 \\right)"
	);
	// Q9c,d
	let mut exp = sum!(
		prod!(-3, "x", sum!(prod!(4, "y"), prod!(7, "z"))),
		prod!(-12, "x")
	);
	assert_eq!(exp.to_string(), "- 3x\\left( 4y + 7z \\right) - 12x");
	exp.factorize();
	assert_eq!(exp.to_string(), "- 3x\\left( 4y + 7z + 4 \\right)");
	let mut exp = sum!(prod!(-17, exp!("x", 2)), prod!(-34, "x", "y"));
	assert_eq!(exp.to_string(), "- 17x^2 - 34xy");
	exp.factorize();
	assert_eq!(exp.to_string(), "- 17x\\left( x + 2y \\right)");
	// Q9a
	let mut exp = sum!(prod!(9, "x"), prod!(18, "x", sum!("a", "b")));
	assert_eq!(exp.to_string(), "9x + 18x\\left( a + b \\right)");
	exp.factorize();
	assert_eq!(exp.to_string(), "9x\\left( 1 + 2a + 2b \\right)");
	// Q9e
	let mut exp = sum!(
		prod!(7, "a", sum!(1, prod!(-4, "x"))),
		prod!(3, "a", sum!(prod!(5, "x"), -6))
	);
	assert_eq!(
		exp.to_string(),
		"7a\\left( 1 - 4x \\right) + 3a\\left( 5x - 6 \\right)"
	);
	exp.factorize();
	assert_eq!(exp.to_string(), "a\\left( - 11 - 13x \\right)");
	// TODO: nested factorization
}
