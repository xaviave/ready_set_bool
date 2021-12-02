mod binary_tree as bt;

fn print_bit(mut n: u32) {
    for _i in 0..8 {
        print!("{}", n & 1);
        n >>= 1;
    }
    println!();
}

// fn eval_formula(formula: &str) -> bool {

// }

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_eval_and() {
		assert_eq!(eval_formula("10&"), 0);
	}

	#[test]
	fn test_eval_or() {
        assert_eq!(eval_formula("10|"), 1);
	}

	#[test]
	fn test_eval_material_condition() {
        assert_eq!(eval_formula("11>"), 1);
	}

	#[test]
	fn test_eval_logical_equivalence() {
        assert_eq!(eval_formula("10="), 0);
	}

    #[test]
	fn test_eval_or_or_equal() {
        assert_eq!(eval_formula("1011||="), 1);
	}
}

fn main() {
    // https://www.geeksforgeeks.org/expression-tree
    let b = bt::BinaryTree::new(imply_node(id_node(false), id_node(true)));
    println!("{} = {}", bt::BinaryTree::collapse(&Box::new(b.head.expect("No head initialized"))), false);
}
