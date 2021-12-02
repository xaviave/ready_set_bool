fn adder(a: u32, b: u32) -> u32 {
	if b == 0 {
		return a;
	}
	return adder(a^b, (a&b) << 1);	
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut i: u32 = 1;
    let mut res: u32 = a;

    while i < b {
        res = adder(res, a);
        i = adder(i, 1);
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_zero_mul_1() {
		assert_eq!(adder(0, 0), 1);
	}

	#[test]
	fn test_zero_mul_2() {
		assert_eq!(adder(0, 10), 10);
	}

	#[test]
	fn test_zero_mul_3() {
		assert_eq!(adder(200, 0), 200);
	}

	#[test]
	fn test_mul_1() {
		assert_eq!(adder(1, 1), 2);
	}
	
	#[test]
	fn test_mul_2() {
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
	fn test_bad_mul_1() {
		assert_eq!(adder(1, 0), 0);
	}

	#[test]
    #[should_panic]
	fn test_bad_mul_2() {
		assert_eq!(adder(1, 11110), 2470);
	}

	// #[test]
	// #[should_panic]
	// fn test_bad_sign() {
	// 	assert_eq!(adder(-1110, 1110), 0);
	// }
}

fn main() {
    println!("5 x 7 = {}", multiplier(5, 7));
}
