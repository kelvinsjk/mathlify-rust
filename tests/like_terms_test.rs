use mathlify::expression::*;
use mathlify::*;

#[test]
fn like_terms() {
	// Sec 1a, Page 63 Q1a,b,d
	let mut exp = sum_verbatim!(prod!(7, "x"), prod!(4, "x"));
	assert_eq!(exp.to_string(), "7x + 4x");
	exp.simplify();
	assert_eq!(exp.to_string(), "11x");
	let exp = sum!(prod!(7, "x"), prod!(4, "x"));
	assert_eq!(exp.to_string(), "11x");
	let mut exp = sum_verbatim!(prod!(5, "x"), prod!(-2, "x"));
	assert_eq!(exp.to_string(), "5x - 2x");
	exp.simplify();
	assert_eq!(exp.to_string(), "3x");
	let mut exp = sum_verbatim!(prod!(-6, "x"), "x");
	assert_eq!(exp.to_string(), "- 6x + x");
	exp.simplify();
	assert_eq!(exp.to_string(), "- 5x");
	// Sec 1a, Page 63, Q2a,c,g
	let mut exp = sum_verbatim!(prod!(3, "x"), 10, prod!(-6, "x"), 5);
	assert_eq!(exp.to_string(), "3x + 10 - 6x + 5");
	exp.simplify();
	assert_eq!(exp.to_string(), "- 3x + 15");
	let mut exp = sum_verbatim!(prod!(5, "x"), prod!(8, "y"), prod!(7, "x"), prod!(-1, "y"));
	assert_eq!(exp.to_string(), "5x + 8y + 7x - y");
	exp.simplify();
	assert_eq!(exp.to_string(), "12x + 7y");
	let mut exp = sum_verbatim!(
		prod!(4, "x"),
		prod!(-1, "y"),
		12,
		prod!(5, "y"),
		-9,
		prod!(-9, "x")
	);
	assert_eq!(exp.to_string(), "4x - y + 12 + 5y - 9 - 9x");
	exp.simplify();
	assert_eq!(exp.to_string(), "- 5x + 4y + 3");
}
