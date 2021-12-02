fn sat(formula: &str) -> bool {
    
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_nnf_0() {
		assert_eq!(sat("AB|"), true);
	}

	#[test]
	fn test_nnf_1() {
		assert_eq!(sat("AB&")true);
	}

	#[test]
	fn test_nnf_2() {
		assert_eq!(sat("AB!&"), false);
	}

	#[test]
	fn test_nnf_3() {
		assert_eq!(sat("AA^"), false);
	}
}

fn main() {
    println!("{} | expected {}", sat("AB|"), true);
}
