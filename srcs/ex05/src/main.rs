fn negation_normal_form(formula: &str) -> String {

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_nnf_0() {
		assert_eq!(negation_normal_form("AB&!"), "A!B!|");
	}

	#[test]
	fn test_nnf_1() {
		assert_eq!(negation_normal_form("AB|!"), "A!B!&");
	}

	#[test]
	fn test_nnf_2() {
		assert_eq!(negation_normal_form("AB>"), "A!B|");
	}

	#[test]
	fn test_nnf_3() {
		assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
	}

	#[test]
	fn test_nnf_4() {
		assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
	}
}

fn main() {
    println!("{} | expected {}", negation_normal_form("AB|C&!"), "A!B!|");
}
