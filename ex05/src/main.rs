mod bits;
mod parser;
mod binary_tree;

#[macro_use]
extern crate pest_derive;

fn negation_normal_form(formula: &str) -> String {
	// miss distributivity and neg at the end
	let p = parser::ParserA::new(formula);
	p.get_nnf()
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn eval_resolve_no_op_0() {
		let data: u32 = 0;
		let p = parser::ParserA::new("Z");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_no_op_1() {
		let data: u32 = 0b10000000000000000000000000u32;
		let p = parser::ParserA::new("Z");
		assert_eq!(p.resolve(data), true);
	}

	// AND TESTS
	#[test]
	fn eval_resolve_and_ab_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_and_ab_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_and_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB&");
		assert_eq!(p.resolve(data), false);
	}
	
	#[test]
	fn eval_resolve_and_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB&");
		assert_eq!(p.resolve(data), true);
	}

	// OR TESTS
	#[test]
	fn eval_resolve_or_ab_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB|");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_or_ab_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_or_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_or_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	// XOR TESTS
	#[test]
	fn eval_resolve_xor_ab_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB^");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_xor_ab_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB^");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_xor_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB^");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_xor_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB^");
        assert_eq!(p.resolve(data), false);
	}

	// NEG TESTS
	#[test]
	fn eval_resolve_neg_0() {
		let data: u32 = 0;
		let p = parser::ParserA::new("T!");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_neg_1() {
		let data: u32 = 0b10000000000000000000000000u32;
		let p = parser::ParserA::new("Z!");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_double_neg_0() {
		let data: u32 = 0;
		let p = parser::ParserA::new("T!!");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_double_neg_1() {
		let data: u32 = 1;
		let p = parser::ParserA::new("A!!");
        assert_eq!(p.resolve(data), true);
	}

	// IMPLY TESTS
	#[test]
	fn eval_resolve_imply_ab_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_imply_ab_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB>");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_imply_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_imply_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	// EQUAL TESTS
	#[test]
	fn eval_resolve_equal_ab_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_equal_ab_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_equal_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_equal_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB=");
        assert_eq!(p.resolve(data), true);
	}

	// ADVANCED TESTS
    #[test]
	fn eval_resolve_or_neg_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB|!");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_resolve_or_neg_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB|!");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_resolve_or_neg_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB|!");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_resolve_or_neg_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB|!");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_resolve_and_neg_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB&!");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_resolve_and_neg_01() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AB&!");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_resolve_and_neg_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB&!");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_resolve_and_neg_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB&!");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_resolve_or_equal_000() {
		let data: u32 = 0;
		let p = parser::ParserA::new("ABC|=");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_resolve_or_equal_001() {
		let data: u32 = 1;
		let p = parser::ParserA::new("ABC|=");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_resolve_or_equal_101() {
		let data: u32 = 0b101u32;
		let p = parser::ParserA::new("ABC|=");
        assert_eq!(p.resolve(data), true);
	}
	
	#[test]
	fn eval_resolve_or_or_equal_0000() {
		let data: u32 = 0;
		let p = parser::ParserA::new("ABCD||=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_or_or_equal_1010() {
		let data: u32 = 0b1010u32;
		let p = parser::ParserA::new("ABCD||=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_or_or_equal_1100() {
		let data: u32 = 0b1100u32;
		let p = parser::ParserA::new("ABCD||=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_or_and_equal_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB|AB&=");
        assert_eq!(p.resolve(data), true);
	}
	
	#[test]
	fn eval_resolve_or_and_equal_10() {
		let data: u32 = 0b10u32;
		let p = parser::ParserA::new("AB|AB&=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_or_and_equal_1010() {
		let data: u32 = 0b1010u32;
		let p = parser::ParserA::new("AB|CD&=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_or_and_equal_1011() {
		let data: u32 = 0b1011u32;
		let p = parser::ParserA::new("CD|AB&=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_and_and_and_0000() {
		let data: u32 = 0;
		let p = parser::ParserA::new("ABCD&&&");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_and_and_and_1() {
		let data: u32 = 1;
		let p = parser::ParserA::new("AAAA&&&");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_and_and_and_1101() {
		let data: u32 = 0b1101u32;
		let p = parser::ParserA::new("ABCD&&&");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_and_and_or_00() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB&AZ&|");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_and_and_or_11() {
		let data: u32 = 0b11u32;
		let p = parser::ParserA::new("AB&AB&|");
        assert_eq!(p.resolve(data), true);
	}

	// PARSING TESTS
	#[test]
	#[should_panic]
	fn parsing_bad_char() {
		let _p = parser::ParserA::new("-");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_char1() {
		let _p = parser::ParserA::new("10|");
	}

	#[test]
	#[should_panic]
	fn parsing_no_op1() {
		let _p = parser::ParserA::new("AAAA");
	}

	#[test]
	#[should_panic]
	fn parsing_no_v() {
		let _p = parser::ParserA::new("&&");
	}
	
	#[test]
	#[should_panic]
	fn parsing_no_v1() {
		let _p = parser::ParserA::new("!");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula() {
		let _p = parser::ParserA::new("A!B!");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula1() {
		let _p = parser::ParserA::new("AB|||");
	}

	// NNF FORM
	#[test]
	fn eval_no_op() {
		assert_eq!(negation_normal_form("A"), "A");
	}

	#[test]
	fn eval_and() {
		assert_eq!(negation_normal_form("AB&"), "AB&");
	}

	#[test]
	fn eval_or() {
		assert_eq!(negation_normal_form("AB|"), "AB|");
	}

	#[test]
	fn eval_double_neg() {
		assert_eq!(negation_normal_form("A!!"), "A");
	}

	#[test]
	fn eval_triple_neg() {
		assert_eq!(negation_normal_form("A!!!"), "A!");
	}

	#[test]
	fn eval_de_morgan_0() {
		assert_eq!(negation_normal_form("AB|!"), "A!B!&");
	}
	
	#[test]
	fn eval_de_morgan_1() {
		assert_eq!(negation_normal_form("AB&!"), "A!B!|");
	}

	#[test]
	fn eval_imply() {
		assert_eq!(negation_normal_form("AB>"), "A!B|");
	}

	#[test]
	fn eval_equal() {
		assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
	}
}

fn main() {
	let formula = "AB>";
	println!("result: {}\n", negation_normal_form(formula));

	let formula = "BA>";
	println!("result: {}\n", negation_normal_form(formula));
	
	let formula = "AB=";
	println!("result: {}\n", negation_normal_form(formula));
}
