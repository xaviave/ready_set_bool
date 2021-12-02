fn adder(a: u32, b: u32) -> u32 {
	// https://iq.opengenus.org/addition-using-bitwise-operations/
	if b == 0 { return a; }
	adder(a^b, (a&b) << 1)
}

#[cfg(test)]
mod tests {
	// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
	use super::*;

	#[test]
	fn test_zero_sum_1() {
		assert_eq!(adder(0, 0), 0);
	}

	#[test]
	fn test_zero_sum_2() {
		assert_eq!(adder(0, 10), 10);
	}

	#[test]
	fn test_zero_sum_3() {
		assert_eq!(adder(200, 0), 200);
	}

	#[test]
	fn test_sum_1() {
		assert_eq!(adder(1, 1), 2);
	}
	
	#[test]
	fn test_sum_2() {
		assert_eq!(adder(1456, 1000), 2456);
	}
	
	#[test]
	fn test_overflow_1() {
		assert_eq!(adder(4_294_967_292, 3), 4_294_967_295);
	}

	#[test]
	fn test_overflow_2() {
		assert_eq!(adder(4_294_967_292, 3), 4_294_967_295);
	}

	#[test]
	fn test_overflow_3() {
		assert_eq!(adder(4_294_967_295, 2), 1);
	}

	#[test]
    #[should_panic]
	fn test_bad_sum_1() {
		assert_eq!(adder(1, 0), 0);
	}

	#[test]
    #[should_panic]
	fn test_bad_sum_2() {
		assert_eq!(adder(1, 11110), 2470);
	}

	// #[test]
	// #[should_panic]
	// fn test_bad_sign() {
	// 	assert_eq!(adder(-1110, 1110), 0);
	// }
}

fn main() {
	println!("add 1 + 2 = {}", adder(1, 2));
}
