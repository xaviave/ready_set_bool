mod parser;
mod binary_tree;


fn eval_formula(formula: &str) -> bool {
	let p = parser::Parser::new(formula);
	p.resolve()
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn eval_no_op_0() {
		assert_eq!(eval_formula("0"), false);
	}

	#[test]
	fn eval_no_op_1() {
		assert_eq!(eval_formula("1"), true);
	}

	// AND TESTS
	#[test]
	fn eval_and_11() {
		assert_eq!(eval_formula("11&"), true);
	}

	#[test]
	fn eval_and_00() {
		assert_eq!(eval_formula("00&"), false);
	}

	#[test]
	fn eval_and_01() {
		assert_eq!(eval_formula("01&"), false);
	}
	
	#[test]
	fn eval_and_10() {
		assert_eq!(eval_formula("10&"), false);
	}

	// OR TESTS
	#[test]
	fn eval_or_10() {
        assert_eq!(eval_formula("10|"), true);
	}

	#[test]
	fn eval_or_01() {
        assert_eq!(eval_formula("01|"), true);
	}

	#[test]
	fn eval_or_11() {
        assert_eq!(eval_formula("11|"), true);
	}

	#[test]
	fn eval_or_00() {
        assert_eq!(eval_formula("00|"), false);
	}

	// XOR TESTS
	#[test]
	fn eval_xor_10() {
        assert_eq!(eval_formula("10^"), true);
	}

	#[test]
	fn eval_xor_01() {
        assert_eq!(eval_formula("01^"), true);
	}

	#[test]
	fn eval_xor_11() {
        assert_eq!(eval_formula("11^"), false);
	}

	#[test]
	fn eval_xor_00() {
        assert_eq!(eval_formula("00^"), false);
	}

	// NEG TESTS
	#[test]
	fn eval_neg_0() {
        assert_eq!(eval_formula("0!"), true);
	}

	#[test]
	fn eval_neg_1() {
        assert_eq!(eval_formula("1!"), false);
	}

	// IMPLY TESTS
	#[test]
	fn eval_imply_00() {
        assert_eq!(eval_formula("00>"), true);
	}

	#[test]
	fn eval_imply_01() {
        assert_eq!(eval_formula("01>"), true);
	}

	#[test]
	fn eval_imply_10() {
        assert_eq!(eval_formula("10>"), false);
	}

	#[test]
	fn eval_imply_11() {
        assert_eq!(eval_formula("11>"), true);
	}

	// EQUAL TESTS
	#[test]
	fn eval_equal_00() {
        assert_eq!(eval_formula("00="), true);
	}

	#[test]
	fn eval_equal_01() {
        assert_eq!(eval_formula("01="), false);
	}

	#[test]
	fn eval_equal_10() {
        assert_eq!(eval_formula("10="), false);
	}

	#[test]
	fn eval_equal_11() {
        assert_eq!(eval_formula("11="), true);
	}

	// ADVANCED TESTS
    #[test]
	fn eval_or_equal() {
        assert_eq!(eval_formula("101|="), true);
	}

	#[test]
	fn eval_or_or_equal() {
        assert_eq!(eval_formula("1011||="), true);
	}

	#[test]
	fn eval_or_and_equal() {
        assert_eq!(eval_formula("10|11&="), true);
	}
	
	#[test]
	fn eval_and_and_and() {
        assert_eq!(eval_formula("1011&&&"), false);
	}

	#[test]
	fn eval_and_and_or() {
        assert_eq!(eval_formula("10&11&|"), true);
	}

	// PARSING TESTS
	#[test]
	#[should_panic]
	fn parsing_bad_char() {
        assert_eq!(eval_formula("-"), true);
	}

	#[test]
	#[should_panic]
	fn parsing_bad_char1() {
        assert_eq!(eval_formula("AB|"), true);
	}

	#[test]
	#[should_panic]
	fn parsing_no_op() {
        assert_eq!(eval_formula("10"), true);
	}

	#[test]
	#[should_panic]
	fn parsing_no_v() {
        assert_eq!(eval_formula("&&|"), true);
	}
	
	#[test]
	#[should_panic]
	fn parsing_no_v1() {
        assert_eq!(eval_formula("!"), true);
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula() {
        assert_eq!(eval_formula("1!1!"), true);
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula1() {
        assert_eq!(eval_formula("11|||"), true);
	}
}

fn main() {
	let p = "1010||=";

	println!("'{}' = {} | {}", p, eval_formula(p), true);
}
