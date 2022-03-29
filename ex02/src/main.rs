fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_gray_code_0() {
		assert_eq!(gray_code(0), 0);
	}

	#[test]
	fn test_gray_code_1() {
		assert_eq!(gray_code(1), 1);
	}

	#[test]
	fn test_gray_code_2() {
		assert_eq!(gray_code(2), 3);
	}

	#[test]
	fn test_gray_code_3() {
		assert_eq!(gray_code(3), 2);
	}

	#[test]
	fn test_gray_code_4() {
		assert_eq!(gray_code(4), 6);
	}

	#[test]
	fn test_gray_code_5() {
		assert_eq!(gray_code(5), 7);
	}

	#[test]
	fn test_gray_code_6() {
		assert_eq!(gray_code(6), 5);
	}

	#[test]
	fn test_gray_code_7() {
		assert_eq!(gray_code(7), 4);
	}

	#[test]
	fn test_gray_code_8() {
		assert_eq!(gray_code(8), 12);
	}
}

fn main() {
    println!("gray code of 5 = {}", gray_code(5));
}
