fn conjunctive_normal_form(formula: &str) -> String {

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_nnf_0() {
		assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
	}

	#[test]
	fn test_nnf_1() {
		assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
	}

	#[test]
	fn test_nnf_2() {
		assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
	}

	#[test]
	fn test_nnf_3() {
		assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
	}

	#[test]
	fn test_nnf_4() {
		assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
	}
    
	#[test]
	fn test_nnf_5() {
		assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
	}
    
	#[test]
	fn test_nnf_6() {
		assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
	}
}

fn main() {
    println!("{} | expected {}", conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
}
