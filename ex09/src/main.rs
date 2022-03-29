mod binary_tree;
mod parser;

#[macro_use]
extern crate pest_derive;

fn remove_duplicate(sets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // let mut sort_sets = sets;
    sets.iter()
        .map(|s| {
            let mut t = (*s).clone();
            t.sort();
            t.dedup();
            t
        })
        .collect::<Vec<Vec<i32>>>()
}

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let p = parser::ParserA::new(formula);
    p.resolve(remove_duplicate(sets))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_resolve_set_value() {
        let formula: &str = "A";
        let sets = vec![vec![]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }

    #[test]
    fn eval_resolve_set_value_1() {
        let formula: &str = "A";
        let sets = vec![vec![0, 2]];

        assert_eq!(eval_set(formula, sets), vec![0, 2]);
    }

    #[test]
    fn eval_resolve_set_value_not() {
        let formula: &str = "A!";
        let sets = vec![vec![]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }

    #[test]
    fn eval_resolve_set_value_not_1() {
        let formula: &str = "A!";
        let sets = vec![vec![2, 5, 7]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }
    #[test]
    fn eval_resolve_set_and() {
        let formula: &str = "AB&";
        let sets = vec![
            vec![1, 5, 7, 2222, 1, 9, 99, 999999999],
            vec![1, 9, 99, 999999999],
        ];

        assert_eq!(eval_set(formula, sets), vec![1, 9, 99, 999999999]);
    }
    #[test]
    fn eval_resolve_set_and_1() {
        let formula: &str = "AB&";
        let sets = vec![vec![1, 5, 7, 2222, 1], vec![9, 99, 999999999]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }

    #[test]
    fn eval_resolve_set_value_or() {
        let formula: &str = "AB|";
        let sets = vec![vec![1, 2], vec![-3, 8]];

        assert_eq!(eval_set(formula, sets), vec![1, 2, -3, 8]);
    }

    #[test]
    fn eval_resolve_set_value_xor() {
        let formula: &str = "AB^";
        let sets = vec![vec![1, 2, 4], vec![2, 3, 4, 7]];

        assert_eq!(eval_set(formula, sets), vec![1, 3, 7]);
    }

    #[test]
    fn eval_resolve_set_value_imply() {
        let formula: &str = "AB>";
        let sets = vec![vec![1, 2, 3], vec![1, 2, 3, 4]];

        assert_eq!(eval_set(formula, sets), vec![1, 2, 3]);
    }

    #[test]
    fn eval_resolve_set_value_imply_1() {
        let formula: &str = "AB>";
        let sets = vec![vec![1, 2, 3], vec![4]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }

    #[test]
    fn eval_resolve_set_value_equal() {
        let formula: &str = "AB=";
        let sets = vec![vec![1, 2, 3], vec![1, 2, 3]];

        assert_eq!(eval_set(formula, sets), vec![1, 2, 3]);
    }

    #[test]
    fn eval_resolve_set_value_equal_1() {
        let formula: &str = "AB=";
        let sets = vec![vec![1, 2, 3], vec![1, 4, 3]];

        assert_eq!(eval_set(formula, sets), vec![]);
    }

    // ERRORS
    #[test]
    #[should_panic]
    fn error_set_size() {
        let sets: Vec<Vec<i32>> = [[0, 1, 2].to_vec(), [0, 3, 4].to_vec()].to_vec();
        let formula = "C";

        eval_set(formula, sets);
    }
}

fn main() {
    let sets: Vec<Vec<i32>> = [[0, 1, 1, 2].to_vec(), [0, 3, 4].to_vec()].to_vec();
    let formula = "AB>";

    println!("{:?}", eval_set(formula, sets));
}
