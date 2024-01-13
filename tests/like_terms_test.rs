use mathlify::expression::*;
use mathlify::*;

#[test]
fn like_terms() {
	let mut exp = sum!(prod!(7, "x"), prod!(4, "x"));
	assert_eq!(exp.to_string(), "7x + 4x");
	exp.simplify();
	assert_eq!(exp.to_string(), "11x");
}
