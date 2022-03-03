fn get_set_combinaison(set: &[i32], nu_pset: i32, a: i32) -> Vec<i32> {
    let mut m: i32 = 1;
    let mut pset: Vec<i32> = Vec::new();

    for &v in set.iter().rev() {
        let zz = nu_pset / m;
        if a % zz < zz / 2 {
            pset.push(v);
        }
        m += m;
    }
    pset
}

fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    let mut pset: Vec<Vec<i32>> = Vec::new();
    let nu_pset: i32 = (2 as i32).pow(set.len() as u32) as i32;

    for a in 0..nu_pset {
        pset.push(get_set_combinaison(set, nu_pset, a));
    }
    pset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_no_set() {
        let set: &[i32] = &[];
        assert_eq!(powerset(set), vec![vec![]]);
    }

    #[test]
    fn eval_set_size_1() {
        let set: &[i32] = &[0];
        assert_eq!(powerset(set), vec![vec![0], vec![]]);
    }

    #[test]
    fn eval_set_size_2() {
        let set: &[i32] = &[0, 1];
        assert_eq!(powerset(set), vec![vec![1, 0], vec![1], vec![0], vec![]]);
    }

    #[test]
    fn eval_set_size_3() {
        let set: &[i32] = &[0, 1, 2];
        assert_eq!(
            powerset(set),
            vec![
                vec![2, 1, 0],
                vec![2, 1],
                vec![2, 0],
                vec![2],
                vec![1, 0],
                vec![1],
                vec![0],
                vec![]
            ]
        );
    }

    #[test]
    fn eval_set_size_3_1() {
        let set: &[i32] = &[10, 4, 9, 30];
        assert_eq!(
            powerset(set),
            vec![
                vec![30, 9, 4, 10],
                vec![30, 9, 4],
                vec![30, 9, 10],
                vec![30, 9],
                vec![30, 4, 10],
                vec![30, 4],
                vec![30, 10],
                vec![30],
                vec![9, 4, 10],
                vec![9, 4],
                vec![9, 10],
                vec![9],
                vec![4, 10],
                vec![4],
                vec![10],
                vec![]
            ]
        );
    }

    #[test]
    fn eval_set_size_4() {
        let set: &[i32] = &[0, 1, 2, 3];
        assert_eq!(
            powerset(set),
            vec![
                vec![3, 2, 1, 0],
                vec![3, 2, 1],
                vec![3, 2, 0],
                vec![3, 2],
                vec![3, 1, 0],
                vec![3, 1],
                vec![3, 0],
                vec![3],
                vec![2, 1, 0],
                vec![2, 1],
                vec![2, 0],
                vec![2],
                vec![1, 0],
                vec![1],
                vec![0],
                vec![]
            ]
        );
    }
}

fn main() {
    println!("{:?}", powerset(&[0, 1, 2, 3, 4]));
}
