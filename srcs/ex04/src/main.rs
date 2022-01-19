mod bits;
mod parser;
mod binary_tree;

use itertools::join;
use ::join_lazy_fmt::*;

fn get_data_value(p: &parser::Parser, nu_sol: i32, a: i32) -> u32 {
	let mut m: i32 = 1;
	let mut data: u32 = 0;

	print!("| ");
	for &v in p.vars.iter().rev() {
		let zz = nu_sol / m;
		if a % zz < zz / 2 {
			data += 1 << (v as u8 - 65);
			print!("1 | ");
		} else { print!("0 | "); }
		m += m;
	}
	data
}

fn eval_formula(formula: &str) {
	let p = parser::Parser::new(formula);

	let nu_sol: i32 = p.vars.len().pow(2) as i32;
	println!("| {} | = |\n|-{}-|", join(&p.vars, " | "), "-|-".join((0..(p.vars.len() + 1)).map(|_| "-")));
	for a in 0..nu_sol {
		let data: u32 = get_data_value(&p, nu_sol, a);
		println!("{} |", if p.resolve(data) { 1 } else { 0 });
	}
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn eval_no_op_0() {
		let data: u32 = 0;
		let p = parser::Parser::new("Z");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_no_op_1() {
		let data: u32 = 0b10000000000000000000000000u32;
		let p = parser::Parser::new("Z");
		assert_eq!(p.resolve(data), true);
	}

	// AND TESTS
	#[test]
	fn eval_and_ab_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_and_ab_01() {
		let data: u32 = 1;
		let p = parser::Parser::new("AB&");
		assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_and_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB&");
		assert_eq!(p.resolve(data), false);
	}
	
	#[test]
	fn eval_and_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB&");
		assert_eq!(p.resolve(data), true);
	}

	// OR TESTS
	#[test]
	fn eval_or_ab_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB|");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_or_ab_01() {
		let data: u32 = 1;
		let p = parser::Parser::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_or_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_or_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB|");
        assert_eq!(p.resolve(data), true);
	}

	// XOR TESTS
	#[test]
	fn eval_xor_ab_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB^");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_xor_ab_01() {
		let data: u32 = 1;
		let p = parser::Parser::new("AB^");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_xor_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB^");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_xor_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB^");
        assert_eq!(p.resolve(data), false);
	}

	// NEG TESTS
	#[test]
	fn eval_neg_0() {
		let data: u32 = 0;
		let p = parser::Parser::new("T!");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_neg_1() {
		let data: u32 = 0b10000000000000000000000000u32;
		let p = parser::Parser::new("Z!");
        assert_eq!(p.resolve(data), false);
	}

	// IMPLY TESTS
	#[test]
	fn eval_imply_ab_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_imply_ab_01() {
		let data: u32 = 1;
		let p = parser::Parser::new("AB>");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_imply_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_imply_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB>");
        assert_eq!(p.resolve(data), true);
	}

	// EQUAL TESTS
	#[test]
	fn eval_equal_ab_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_equal_ab_01() {
		let data: u32 = 1;
		let p = parser::Parser::new("AB=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_equal_ab_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_equal_ab_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB=");
        assert_eq!(p.resolve(data), true);
	}

	// ADVANCED TESTS
    #[test]
	fn eval_or_equal_000() {
		let data: u32 = 0;
		let p = parser::Parser::new("ABC|=");
        assert_eq!(p.resolve(data), true);
	}
	
    #[test]
	fn eval_or_equal_001() {
		let data: u32 = 1;
		let p = parser::Parser::new("ABC|=");
        assert_eq!(p.resolve(data), false);
	}
	
    #[test]
	fn eval_or_equal_101() {
		let data: u32 = 0b101u32;
		let p = parser::Parser::new("ABC|=");
        assert_eq!(p.resolve(data), true);
	}
	
	#[test]
	fn eval_or_or_equal_0000() {
		let data: u32 = 0;
		let p = parser::Parser::new("ABCD||=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_or_or_equal_1010() {
		let data: u32 = 0b1010u32;
		let p = parser::Parser::new("ABCD||=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_or_or_equal_1100() {
		let data: u32 = 0b1100u32;
		let p = parser::Parser::new("ABCD||=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_or_and_equal_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB|AB&=");
        assert_eq!(p.resolve(data), true);
	}
	
	#[test]
	fn eval_or_and_equal_10() {
		let data: u32 = 0b10u32;
		let p = parser::Parser::new("AB|AB&=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_or_and_equal_1010() {
		let data: u32 = 0b1010u32;
		let p = parser::Parser::new("AB|CD&=");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_or_and_equal_1011() {
		let data: u32 = 0b1011u32;
		let p = parser::Parser::new("CD|AB&=");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_and_and_and_0000() {
		let data: u32 = 0;
		let p = parser::Parser::new("ABCD&&&");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_and_and_and_1() {
		let data: u32 = 1;
		let p = parser::Parser::new("AAAA&&&");
        assert_eq!(p.resolve(data), true);
	}

	#[test]
	fn eval_and_and_and_1101() {
		let data: u32 = 0b1101u32;
		let p = parser::Parser::new("ABCD&&&");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_and_and_or_00() {
		let data: u32 = 0;
		let p = parser::Parser::new("AB&AZ&|");
        assert_eq!(p.resolve(data), false);
	}

	#[test]
	fn eval_and_and_or_11() {
		let data: u32 = 0b11u32;
		let p = parser::Parser::new("AB&AB&|");
        assert_eq!(p.resolve(data), true);
	}

	// PARSING TESTS
	#[test]
	#[should_panic]
	fn parsing_bad_char() {
		let _p = parser::Parser::new("-");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_char1() {
		let _p = parser::Parser::new("10|");
	}

	#[test]
	#[should_panic]
	fn parsing_no_op1() {
		let _p = parser::Parser::new("AAAA");
	}

	#[test]
	#[should_panic]
	fn parsing_no_v() {
		let _p = parser::Parser::new("&&");
	}
	
	#[test]
	#[should_panic]
	fn parsing_no_v1() {
		let _p = parser::Parser::new("!");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula() {
		let _p = parser::Parser::new("A!B!");
	}

	#[test]
	#[should_panic]
	fn parsing_bad_formula1() {
		let _p = parser::Parser::new("AB|||");
	}
}

fn main() {
	let p = "ABCD||=";

	eval_formula(p);
}
