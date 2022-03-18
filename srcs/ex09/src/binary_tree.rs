#[derive(Clone, PartialEq, Debug)]
pub enum Op<T> {
    And,
    Or,
    Xor,
    Imply,
    Equal,
    IdNode(T),
}

#[derive(Clone, PartialEq, Debug)]
pub struct BtNode<T> {
    op: Op<T>,
    neg: bool,
    left: Option<Box<BtNode<T>>>,
    right: Option<Box<BtNode<T>>>,
}

pub struct BinaryTree<T> {
    pub head: BtNode<T>,
}

impl BtNode<u8> {
    fn get_right_node(&self) -> Option<BtNode<u8>> {
        match self.op {
            Op::And | Op::Or => Some(*(self.right.clone().unwrap())),
            _ => Some(self.clone()),
        }
    }
    fn get_left_node(&self) -> Option<BtNode<u8>> {
        match self.op {
            Op::And | Op::Or => Some(*(self.left.clone().unwrap())),
            _ => Some(self.clone()),
        }
    }

    pub fn new(op: Op<u8>, l: Option<BtNode<u8>>, r: Option<BtNode<u8>>, neg: bool) -> Self {
        BtNode::<u8> {
            op,
            left: l.map(Box::new),
            right: r.map(Box::new),
            neg,
        }
    }
}

impl BinaryTree<u8> {
    pub fn new(head: BtNode<u8>) -> Self {
        BinaryTree::<u8> { head }
    }

    fn set_and(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        s1.iter()
            .filter_map(|x| if s2.contains(x) { Some(*x) } else { None })
            .collect()
    }

    fn set_or(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        let mut tmp: Vec<i32> = s1
            .iter()
            .filter_map(|x| if !s2.contains(x) { Some(*x) } else { None })
            .collect();
        tmp.extend(s2);
        tmp
    }

    fn set_xor(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        let tmp = BinaryTree::set_or(s1, s2);
        let tmp1 = BinaryTree::set_and(s1, s2);

        tmp.iter()
            .filter_map(|x| if !tmp1.contains(x) { Some(*x) } else { None })
            .collect()
    }

    fn set_imply(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        s1.iter()
            .filter_map(|x| if s2.contains(x) { Some(*x) } else { None })
            .collect()
    }

    fn set_equal(s1: &Vec<i32>, s2: &Vec<i32>) -> Vec<i32> {
        if s1 == s2 {
            (*s1).clone()
        } else {
            vec![]
        }
    }

    pub fn collapse(node: &BtNode<u8>, data: Vec<Vec<i32>>) -> Vec<i32> {
        let mut r: Vec<i32> = vec![];
        let mut l: Vec<i32> = vec![];

        if let Some(left) = &node.left {
            l = BinaryTree::collapse(left, data.clone());
        }

        if let Some(right) = &node.right {
            r = BinaryTree::collapse(right, data.clone());
        }

        let tmp: Vec<i32> = match node.op {
            Op::And => BinaryTree::set_and(&l, &r),
            Op::Or => BinaryTree::set_or(&l, &r),
            Op::Xor => BinaryTree::set_xor(&l, &r),
            Op::Imply => BinaryTree::set_imply(&l, &r),
            Op::Equal => BinaryTree::set_equal(&l, &r),

            Op::IdNode(x) => {
                if x > data.len() as u8 {
                    panic!("No sets at index {}", x)
                }
                data[x as usize].clone()
            }
        };
        if node.neg {
            vec![]
        } else {
            tmp
        }
    }
}

fn check_absorption(left: Option<BtNode<u8>>, right: Option<BtNode<u8>>, op: Op<u8>) -> BtNode<u8> {
    // check absoption, allow formula simplification
    fn _check_absorption(node: BtNode<u8>, node1: Option<BtNode<u8>>) -> bool {
        if let Some(n1) = node1 {
            node == n1
        } else {
            panic!("Null node")
        }
    }

    let d: (bool, bool) = check_distributivity(op.clone(), left.clone(), right.clone());
    if d.0 {
        if let Some(l) = left.clone() {
            if let Some(r) = right.clone() {
                if _check_absorption(l.clone(), r.get_left_node())
                    || _check_absorption(l.clone(), r.get_right_node())
                    || _check_absorption(r.clone(), l.get_left_node())
                    || _check_absorption(r.clone(), l.get_right_node())
                {
                    return if d.1 { r } else { l };
                } else {
                }
            } else {
                panic!("Null Node");
            }
        } else {
            panic!("Null Node");
        }
    }
    BtNode::new(op, left, right, false)
}

pub fn and_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    // return simplified node or the original AND node
    check_absorption(l, r, Op::And)
}

pub fn or_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    // return simplified node or the original OR node
    check_absorption(l, r, Op::Or)
}

pub fn xor_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Xor, l, r, false)
}

pub fn imply_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Imply, l, r, false)
}

pub fn equal_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Equal, l, r, false)
}

pub fn negation_node(node: Option<BtNode<u8>>) -> BtNode<u8> {
    if let Some(n) = node {
        if !n.neg == true && (n.op == Op::And || n.op == Op::Or) {
            BtNode::new(
                match n.op {
                    Op::And => Op::Or,
                    Op::Or => Op::And,
                    _ => n.op.clone(),
                },
                Some(negation_node(n.get_left_node())),
                Some(negation_node(n.get_right_node())),
                n.neg,
            )
        } else {
            BtNode {
                op: n.op,
                left: n.left,
                right: n.right,
                neg: !n.neg,
            }
        }
    } else {
        panic!("Negation can't be applied to a Null node")
    }
}

pub fn id_node(value: u8) -> BtNode<u8> {
    BtNode {
        op: Op::IdNode(value),
        left: None,
        right: None,
        neg: false,
    }
}

pub fn distributivity_or(d: bool, l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    if d {
        if let Some(n) = l {
            BtNode::new(
                Op::And,
                Some(BtNode::new(Op::Or, n.get_left_node(), r.clone(), false)),
                Some(BtNode::new(Op::Or, n.get_right_node(), r, false)),
                false,
            )
        } else {
            panic!("Null Node error")
        }
    } else {
        if let Some(n) = r {
            BtNode::new(
                Op::And,
                Some(BtNode::new(Op::Or, n.get_left_node(), l.clone(), false)),
                Some(BtNode::new(Op::Or, n.get_right_node(), l, false)),
                false,
            )
        } else {
            panic!("Null Node error")
        }
    }
}

fn check_distributivity(op: Op<u8>, l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> (bool, bool) {
    fn _check_distributivity(op: Op<u8>, node: Option<BtNode<u8>>) -> bool {
        if let Some(n) = node {
            match n.op {
                Op::IdNode(_) => false,
                _ => op != n.op,
            }
        } else {
            panic!("Null node")
        }
    }
    let d1: bool = _check_distributivity(op.clone(), l.clone());
    let d2: bool = _check_distributivity(op.clone(), r.clone());
    if (d1 && !d2) || (!d1 && d2) {
        (true, d1)
    } else {
        (false, false)
    }
}

pub fn apply_distributivity(mut node: Option<BtNode<u8>>) -> Option<BtNode<u8>> {
    // recursively capply distributivity law after de morgan's law and negation's one
    let d: (bool, bool);
    if let Some(n) = node.clone() {
        match n.op {
            Op::And => {
                //  dont apply distributivity over Op::And
                Some(BtNode::new(
                    n.op.clone(),
                    apply_distributivity(n.get_left_node().clone()),
                    apply_distributivity(n.get_right_node().clone()),
                    n.neg,
                ))
            }
            Op::Or => {
                d = check_distributivity(
                    Op::Or,
                    n.get_left_node().clone(),
                    n.get_right_node().clone(),
                );
                if d.0 {
                    node = Some(distributivity_or(
                        d.1,
                        n.get_left_node(),
                        n.get_right_node(),
                    ));
                }

                Some(BtNode::new(
                    node.clone().unwrap().op,
                    apply_distributivity(node.clone().unwrap().get_left_node()),
                    apply_distributivity(node.clone().unwrap().get_right_node()),
                    node.clone().unwrap().neg,
                ))
            }
            _ => node,
        }
    } else {
        panic!("Null node");
    }
}
