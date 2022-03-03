mod binary_tree;
mod bits;
mod parser;

#[macro_use]
extern crate pest_derive;

fn conjunctive_normal_form(formula: &str) -> String {
	// miss distributivity and neg at the end
	let p = parser::ParserA::new(formula);
	p.get_cnf()
}

fn get_data_value(p: &parser::ParserA, nu_sol: i32, a: i32) -> u32 {
	let mut m: i32 = 1;
	let mut data: u32 = 0;

	for &v in p.vars.iter().rev() {
		let zz = nu_sol / m;
		if a % zz < zz / 2 {
			data += 1 << (v as u8 - 65);
		}
		m += m;
	}
	data
}

fn sat(formula: &str) -> bool {
	let p = parser::ParserA::new(formula);

	let mut nu_sol: i32 = p.vars.len().pow(2) as i32;
	if nu_sol == 1 {
		nu_sol += 1;
	}
	for a in 0..nu_sol {
		let data: u32 = get_data_value(&p, nu_sol, a);
		if p.resolve(data) {
			return true;
		}
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;

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

	#[test]
	fn eval_resolve_distributivity_or_and_000() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB|C&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_or_and_010() {
		let data: u32 = 0b010u32;
		let p = parser::ParserA::new("AB|C&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_or_and_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("AB|C&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_or_and_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("CA&CB&|");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_and_or_000() {
		let data: u32 = 0;
		let p = parser::ParserA::new("AB&C|");
		assert_eq!(p.resolve(data), false);
	}
	
	#[test]
	fn eval_resolve_distributivity_and_or_010() {
		let data: u32 = 0b010u32;
		let p = parser::ParserA::new("AB&C|");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_and_or_010() {
		let data: u32 = 0b010u32;
		let p = parser::ParserA::new("CA|CB|&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_and_or_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("AB&C|");
		assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_distributivity_and_or_neg_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("AB|C&!");
		assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_and_or_neg_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("A!B!&C!|");
		assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_or_or_or_0011() {
		let data: u32 = 0b0011u32;
		let p = parser::ParserA::new("ABCD|||");
		assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_and_and_and_1011() {
		let data: u32 = 0b1011u32;
		let p = parser::ParserA::new("ABCD&&&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_neg_neg_neg_or_or_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("A!B!C!||");
		assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_resolve_distributivity_cnf_neg_neg_neg_and_and_011() {
		let data: u32 = 0b011u32;
		let p = parser::ParserA::new("A!B!C!&&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn sat_true_0() {
		let formula = "A";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_1() {
		let formula = "AA!|";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_2() {
		let formula = "AB|";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_3() {
		let formula = "AB|C&";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_4() {
		let formula = "ABC&&";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_5() {
		let formula = "A!B!C!&&";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_6() {
		let formula = "AB|C&!";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_7() {
		let formula = "AB=";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_true_8() {
		let formula = "AB>";
		assert_eq!(sat(formula), true);
	}
	
	#[test]
	fn sat_true_9() {
		let formula = "A!";
		assert_eq!(sat(formula), true);
	}

	#[test]
	fn sat_false_1() {
		let formula = "AA!&";
		assert_eq!(sat(formula), false);
	}
	
	#[test]
	fn sat_false_2() {
		let formula = "AA!&";
		assert_eq!(sat(formula), false);
	}

	#[test]
	fn sat_false_3() {
		let formula = "AA^";
		assert_eq!(sat(formula), false);
	}
}

fn main() {
	let formula = "AB&C&D&";

	println!("{}", sat(formula));
}
