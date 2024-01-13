use crate::*;

#[test]
fn quotient() {
	let exp = sum!(quotient!(3, "x"), quotient!(4, "y"), -6);
	assert_eq!(exp.to_string(), "\\frac{3}{x} + \\frac{4}{y} - ");
}
