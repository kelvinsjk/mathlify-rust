use mathlify::expression::*;
use mathlify::*;

#[test]
fn quotient() {
	let exp = sum!(quotient!(3, "x"), quotient!(4, "y"), -6);
	assert_eq!(exp.to_string(), "\\frac{3}{x} + \\frac{4}{y} - 6");
	let exp = exp.sub_in("x", &Fraction::new(1, 3).into());
	let exp = exp.sub_in("y", &Fraction::new(-1, 4).into());
	assert_eq!(exp.to_string(), "- 13");
	let exp = sum!(
		quotient!(prod!(exp!("x", 2), "z"), 5),
		prod!(
			-1,
			quotient!(
				sum!(prod!(3, "z"), prod!(-1, "y")),
				sum!(prod!(2, "x"), "z")
			)
		)
	);
	assert_eq!(exp.to_string(), "\\frac{x^2z}{5} - \\frac{3z - y}{2x + z}");
	let exp = exp.sub_in("x", &Fraction::new(-1, 2).into());
	let exp = exp.sub_in("y", &0.into());
	let exp = exp.sub_in("z", &4.into());
	assert_eq!(exp.to_string(), "- \\frac{19}{5}");
}
