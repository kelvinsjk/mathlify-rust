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

	// Sec 2a worksheet 3b Pg 35 Q1g, 2h
	let mut exp = prod!(-3, "a", sum!("y", prod!(8, "x")));
	assert_eq!(exp.to_string(), "- 3a\\left( y + 8x \\right)");
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "- 3ay - 24ax");
	let mut exp = sum!(
		prod!(2, "a", sum!(prod!(3, "y"), prod!(-10, "x"))),
		prod!(-7, "a", sum!("x", prod!(-8, "y")))
	);
	assert_eq!(
		exp.to_string(),
		"2a\\left( 3y - 10x \\right) - 7a\\left( x - 8y \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "6ay - 20ax - 7ax + 56ay");
	exp.simplify();
	assert_eq!(exp.to_string(), "62ay - 27ax");
}

#[test]
fn quadratic_expansion() {
	// Sec 2a worksheet 3b Pg 36, Q3a,j,4b
	let mut exp = prod!(sum!("a", "b"), sum!("x", "y"));
	assert_eq!(
		exp.to_string(),
		"\\left( a + b \\right)\\left( x + y \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "ax + ay + bx + by");
	let mut exp = prod!(
		sum!(prod!(2, "m"), "n"),
		sum!(prod!(4, "x"), prod!(-5, "y"), prod!(-6, "z"))
	);
	assert_eq!(
		exp.to_string(),
		"\\left( 2m + n \\right)\\left( 4x - 5y - 6z \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "8mx - 10my - 12mz + 4nx - 5ny - 6nz");
	let mut exp = prod!(
		sum!(prod!(6, "b"), prod!(-5, "a")),
		sum!(3, prod!(-2, "x"), prod!(7, "y"))
	);
	assert_eq!(
		exp.to_string(),
		"\\left( 6b - 5a \\right)\\left( 3 - 2x + 7y \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "18b - 12bx + 42by - 15a + 10ax - 35ay");

	// worksheet 3c page 37, q1a, 2a,c,3d,4d
	let mut exp = prod!(5, "x", sum!(prod!(2, "x"), 2));
	assert_eq!(exp.to_string(), "5x\\left( 2x + 2 \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "10x^2 + 10x");
	let mut exp = sum!(
		prod!(5, exp!("x", 2)),
		prod!(2, "x", sum!(1, prod!(6, "x")))
	);
	assert_eq!(exp.to_string(), "5x^2 + 2x\\left( 1 + 6x \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "5x^2 + 2x + 12x^2");
	exp.simplify();
	assert_eq!(exp.to_string(), "17x^2 + 2x");
	let mut exp = sum!(
		prod!(4, "x", sum!("x", 9)),
		prod!(-1, "x", sum!(prod!(4, "x"), 9))
	);
	assert_eq!(
		exp.to_string(),
		"4x\\left( x + 9 \\right) - x\\left( 4x + 9 \\right)"
	);
	exp.expand();
	assert_eq!(exp.to_string(), "4x^2 + 36x - 4x^2 - 9x");
	exp.simplify();
	assert_eq!(exp.to_string(), "27x");
	let mut exp = prod!(
		-4,
		"x",
		"y",
		sum!(prod!("x", "z"), prod!(-10, exp!("y", 2)), "z")
	);
	assert_eq!(exp.to_string(), "- 4xy\\left( xz - 10y^2 + z \\right)");
	exp.expand();
	assert_eq!(exp.to_string(), "- 4x^2yz + 40xy^3 - 4xyz");
	let mut exp = sum!(
		prod!(-1, "y", sum!(prod!(7, "y"), prod!(2, "x"))),
		prod!(-2, "x", sum!(prod!(7, "x"), prod!(-2, "y")))
	);
	assert_eq!(
		exp.to_string(),
		"- y\\left( 7y + 2x \\right) - 2x\\left( 7x - 2y \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "- 7y^2 + 2yx - 14x^2");
	// page 38-40 Q5l, 6b, 7l, 8b, 9f
	let mut exp = prod!(sum!(12, prod!(-7, "x")), sum!(9, prod!(-2, "x")));
	assert_eq!(
		exp.to_string(),
		"\\left( 12 - 7x \\right)\\left( 9 - 2x \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "108 - 87x + 14x^2");
	let mut exp = sum!(
		prod!(sum!(prod!(7, "x"), -3), sum!(prod!(4, "x"), -9)),
		prod!(-2, sum!(prod!(4, "x"), 5), sum!(prod!(2, "x"), 5))
	);
	assert_eq!(
		exp.to_string(),
		"\\left( 7x - 3 \\right)\\left( 4x - 9 \\right) - 2\\left( 4x + 5 \\right)\\left( 2x + 5 \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "12x^2 - 135x - 23");
	let mut exp = prod!(
		sum!(prod!(8, "y"), prod!(-5, "x")),
		sum!(prod!(8, "y"), prod!(-3, "x"))
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "64y^2 - 64yx + 15x^2");
	let mut exp = sum!(
		prod!(4, "x", sum!(prod!(10, "y"), prod!(-3, "x"))),
		prod!(
			-3,
			sum!(prod!(5, "x"), prod!(-1, "y")),
			sum!(prod!(7, "y"), prod!(-2, "x"))
		)
	);
	assert_eq!(
		exp.to_string(),
		"4x\\left( 10y - 3x \\right) - 3\\left( 5x - y \\right)\\left( 7y - 2x \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "- 71xy + 18x^2 + 21y^2");
	let mut exp = prod!(
		sum!(prod!(6, exp!("y", 2)), prod!(-1, "y"), 3),
		sum!(prod!(2, "y"), -1)
	);
	assert_eq!(
		exp.to_string(),
		"\\left( 6y^2 - y + 3 \\right)\\left( 2y - 1 \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "12y^3 - 8y^2 + 7y - 3");
}

#[test]
fn power_expansion() {
	let mut exp = exp!(sum!("x", "y"), 2);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "x^2 + 2xy + y^2");
	let mut exp = exp!(prod!("x", "y"), 2);
	assert_eq!(exp.to_string(), "\\left( xy \\right)^2");
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "x^2y^2");
	// Sec 2b worksheet 4a Pg 49 Q1a,i,p
	let mut exp = exp!(sum!("a", 5), 2);
	assert_eq!(exp.to_string(), "\\left( a + 5 \\right)^2");
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "a^2 + 10a + 25");
	let mut exp = exp!(sum!(prod!(2, "p"), Fraction::new(1, 4)), 2);
	assert_eq!(exp.to_string(), "\\left( 2p + \\frac{1}{4} \\right)^2");
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "4p^2 + p + \\frac{1}{16}");
	let mut exp = exp!(
		sum!(
			prod!(Fraction::new(3, 8), "x"),
			prod!(Fraction::new(4, 5), "y", "z")
		),
		2
	);
	assert_eq!(
		exp.to_string(),
		"\\left( \\frac{3}{8}x + \\frac{4}{5}yz \\right)^2"
	);
	exp.expand_and_simplify();
	assert_eq!(
		exp.to_string(),
		"\\frac{9}{64}x^2 + \\frac{3}{5}xyz + \\frac{16}{25}y^2z^2"
	);
	// Pg 50 Q2g,p
	let mut exp = exp!(sum!(prod!(3, "m"), prod!(-10, "n")), 2);
	assert_eq!(exp.to_string(), "\\left( 3m - 10n \\right)^2");
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "9m^2 - 60mn + 100n^2");
	let mut exp = exp!(
		sum!(
			prod!(Fraction::new(4, 5), "x"),
			prod!(Fraction::new(-5, 6), "y", "z")
		),
		2
	);
	assert_eq!(
		exp.to_string(),
		"\\left( \\frac{4}{5}x - \\frac{5}{6}yz \\right)^2"
	);
	exp.expand_and_simplify();
	assert_eq!(
		exp.to_string(),
		"\\frac{16}{25}x^2 - \\frac{4}{3}xyz + \\frac{25}{36}y^2z^2"
	);
	// Pg 51 Q3c,m,n
	let mut exp = prod!(sum!(prod!(6, "c"), 1), sum!(prod!(6, "c"), -1));
	assert_eq!(
		exp.to_string(),
		"\\left( 6c + 1 \\right)\\left( 6c - 1 \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "36c^2 - 1");
	let mut exp = prod!(sum!(prod!("x", "y"), 12), sum!(prod!("x", "y"), -12));
	assert_eq!(
		exp.to_string(),
		"\\left( xy + 12 \\right)\\left( xy - 12 \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "x^2y^2 - 144");
	let mut exp = prod!(sum!(7, prod!(-5, "x", "y")), sum!(prod!(5, "x", "y"), 7));
	assert_eq!(
		exp.to_string(),
		"\\left( 7 - 5xy \\right)\\left( 5xy + 7 \\right)"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "49 - 25x^2y^2");
	// Pg 52 Q4c
	let mut exp = sum!(
		exp!(sum!(4, prod!(9, "x")), 2),
		prod!(-1, exp!(sum!(-4, prod!(-9, "x")), 2))
	);
	assert_eq!(
		exp.to_string(),
		"\\left( 4 + 9x \\right)^2 - \\left( - 4 - 9x \\right)^2"
	);
	exp.expand_and_simplify();
	assert_eq!(exp.to_string(), "0");
}
