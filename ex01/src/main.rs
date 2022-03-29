fn adder(a: u32, b: u32) -> u32 {
	if b == 0 { return a; }
	adder(a^b, (a&b) << 1)
}

fn multiplier(a: u32, b: u32) -> u32 {
    let mut i: u32 = 1;
    let mut res: u32 = a;

	if a == 0 || b == 0 { return 0; }

    while i < b {
        res = adder(res, a);
        i = adder(i, 1);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_zero_mul_0() {
		assert_eq!(multiplier(0, 0), 0);
	}

	#[test]
	fn test_zero_mul_1() {
		assert_eq!(multiplier(1, 0), 0);
	}

	#[test]
	fn test_zero_mul_2() {
		assert_eq!(multiplier(1, 1), 1);
	}

	#[test]
	fn test_zero_mul_3() {
		assert_eq!(multiplier(200, 1), 200);
	}

	#[test]
	fn test_mul_1() {
		assert_eq!(multiplier(2, 2), 4);
	}
	
	#[test]
	fn test_mul_2() {
		assert_eq!(multiplier(1456, 1000), 1_456_000);
	}
	
	#[test]
	fn test_overflow_1() {
		assert_eq!(multiplier(4_294_967_295, 1), 4_294_967_295);
	}

	#[test]
	fn test_overflow_2() {
		assert_eq!(multiplier(4_294_967_292, 3), 4_294_967_284);
	}

	#[test]
    #[should_panic]
	fn test_bad_mul_1() {
		assert_eq!(multiplier(1, 0), 1);
	}

	#[test]
    #[should_panic]
	fn test_bad_mul_2() {
		assert_eq!(multiplier(1, 11110), 2470);
	}
}

fn main() {
    println!("5 x 7 = {}", multiplier(5, 7));
}
