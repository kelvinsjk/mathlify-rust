use mathlify::expression::*;
use mathlify::*;

#[test]
fn expansion() {
	// Sec 1A Worksheet 4c, page 65, Q1a,d,e,h,m,p
	let mut exp = prod!(5, sum!("x", 7));
	assert_eq!(exp.to_string(), "5\\left( x + 7 \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "5x + 35");
	let mut exp = prod!(9, sum!(prod!(2, "x"), -1));
	assert_eq!(exp.to_string(), "9\\left( 2x - 1 \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "18x - 9");
	let mut exp = prod!(-1, sum!("x", 12));
	assert_eq!(exp.to_string(), "- \\left( x + 12 \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- x - 12");
	let mut exp = prod!(-4, sum!(prod!(5, "x"), -3));
	assert_eq!(exp.to_string(), "- 4\\left( 5x - 3 \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- 20x + 12");
	let mut exp = prod!(7, sum!(prod!(5, "x"), prod!(4, "y")));
	assert_eq!(exp.to_string(), "7\\left( 5x + 4y \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "35x + 28y");
	let mut exp = prod!(-5, sum!(prod!(2, "y"), prod!(-9, "x")));
	assert_eq!(exp.to_string(), "- 5\\left( 2y - 9x \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- 10y + 45x");
	// Sec 1A Worksheet 4c, page 66, Q3a,c,f
	let mut exp = prod!(prod!(2, "a"), sum!(prod!(9, "x"), prod!(4, "y")));
	assert_eq!(exp.to_string(), "2a\\left( 9x + 4y \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "18ax + 8ay");
	let mut exp = prod!(prod!(-1, "a"), sum!(prod!(5, "x"), "y"));
	assert_eq!(exp.to_string(), "- a\\left( 5x + y \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- 5ax - ay");
	let mut exp = prod!(prod!(-5, "b", "c"), sum!(prod!(3, "y"), prod!(16, "x")));
	assert_eq!(exp.to_string(), "- 5bc\\left( 3y + 16x \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- 15bcy - 80bcx");
}

#[test]
fn expand_and_simplify() {
	// Sec 1A Worksheet 4c, page 67, Q4a,c,f,h
	let mut exp = sum!(prod!(12, sum!(prod!(3, "x"), "y")), prod!(-10, "y"));
	assert_eq!(exp.to_string(), "12\\left( 3x + y \\right) - 10y");
	exp.expand();
	assert_eq!(exp.to_string(), "36x + 12y - 10y");
	exp.simplify();
	assert_eq!(exp.to_string(), "36x + 2y");
	let mut exp = sum!(
		prod!(8, "x"),
		prod!(3, "y"),
		prod!(-1, sum!(prod!(3, "x"), prod!(8, "y")))
	);
	assert_eq!(exp.to_string(), "8x + 3y - \\left( 3x + 8y \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "8x + 3y - 3x - 8y");
	exp.simplify();
	assert_eq!(exp.to_string(), "5x - 5y");
	let mut exp = sum!(
		prod!(3, sum!(prod!(3, "x"), prod!(8, "y"))),
		prod!(-2, sum!(prod!(4, "x"), prod!(-9, "y")))
	);
	assert_eq!(
		exp.to_string(),
		"3\\left( 3x + 8y \\right) - 2\\left( 4x - 9y \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "9x + 24y - 8x + 18y");
	exp.simplify();
	assert_eq!(exp.to_string(), "x + 42y");
	let mut exp = sum!(
		prod!(-4, "x"),
		prod!(-3, sum!(prod!(2, "x"), prod!(12, "y"), prod!(-3, "z"))),
		prod!(-9, "z")
	);
	assert_eq!(
		exp.to_string(),
		"- 4x - 3\\left( 2x + 12y - 3z \\right) - 9z"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "- 4x - 6x - 36y + 9z - 9z");
	exp.simplify();
	assert_eq!(exp.to_string(), "- 10x - 36y");
	// Sec 1A Worksheet 4c, page 67, Q5
	let mut exp = prod!(
		-2,
		sum!(
			prod!(5, "x"),
			prod!(-6, "a", sum!("y", prod!(-1, sum!(prod!(14, "y"), "x"))))
		)
	);
	assert_eq!(
		exp.to_string(),
		"- 2\\left( 5x - 6a\\left( y - \\left( 14y + x \\right) \\right) \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "- 10x - 156ay - 12ax");
	let mut exp = prod!(
		10,
		sum!(
			prod!(7, "y"),
			prod!(
				-3,
				"a",
				sum!(
					prod!(8, "x"),
					prod!(-3, "y"),
					prod!(-2, sum!("x", prod!(-4, "y")))
				)
			)
		)
	);
	assert_eq!(
		exp.to_string(),
		"10\\left( 7y - 3a\\left( 8x - 3y - 2\\left( x - 4y \\right) \\right) \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "70y - 180ax - 150ay");
}
