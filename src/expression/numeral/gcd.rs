// taken and modified from
// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
// on 2024-01-12
pub fn gcd(mut n: i32, mut m: i32) -> i32 {
	assert!(!(n == 0 && m == 0));
	n = n.abs();
	m = m.abs();
	if n == 0 {
		return m;
	};
	if m == 0 {
		return n;
	};
	while m != 0 {
		if m < n {
			std::mem::swap(&mut m, &mut n);
		}
		m %= n;
	}
	n
}

pub fn lcm(n: i32, m: i32) -> i32 {
	assert!(!(n == 0 && m == 0));
	n.abs() * m.abs() / gcd(n, m)
}
