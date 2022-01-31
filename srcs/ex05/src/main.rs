mod bits;
mod parser;
mod binary_tree;

// https://en.wikipedia.org/wiki/Negation_normal_form

fn negation_normal_form(formula: &str) -> String {
	// miss distributivity and neg at the end
	let p = parser::Parser::new(formula);
	p.get_nnf()
}

#[cfg(test)]
mod tests {
    use super::*;

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

	#[test]
	fn eval_distributy_0() {
		assert_eq!(negation_normal_form("AB|C&"), "CA&CB&|");
	}

	#[test]
	fn eval_distributy_1() {
		assert_eq!(negation_normal_form("AB&C|"), "CA|CB|&");
	}

	#[test]
	fn eval_distributy_2() {
		assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
	}

	// #[test]
	// #[should_panic]
	// fn parsing_bad_formula1() {
	// 	let _p = parser::Parser::new("AB|||");
	// }
}

fn main() {
	let formula = "A!";

	println!("reuslt: {}", negation_normal_form(formula));
}
