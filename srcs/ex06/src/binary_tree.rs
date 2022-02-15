use crate::bits;

#[derive(Clone, PartialEq, Debug)]
pub enum Op<T> {
    And,
    Or,
    IdNode(T),
}

#[derive(Clone, Debug)]
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
            Op::And | Op::Or => Some(*(self.right.clone().unwrap())),
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

    pub fn collapse_printer(node: &BtNode<u8>) -> String {
        let mut r = "".to_string();
        let mut l = "".to_string();

        if let Some(left) = &node.left {
            l = BinaryTree::collapse_printer(left);
        }

        if let Some(right) = &node.right {
            r = BinaryTree::collapse_printer(right);
        }

        let tmp = match node.op {
            Op::And => "&".to_string(),
            Op::Or => "|".to_string(),
            Op::IdNode(x) => ((x + 65) as char).to_string(),
        };
        match node.neg {
            true => format!("{}{}{}!", l, r, tmp),
            false => format!("{}{}{}", l, r, tmp),
        }
    }

    pub fn collapse(node: &BtNode<u8>, data: u32) -> bool {
        let mut r: bool = false;
        let mut l: bool = false;

        if let Some(left) = &node.left {
            l = BinaryTree::collapse(left, data);
        }

        if let Some(right) = &node.right {
            r = BinaryTree::collapse(right, data);
        }

        let tmp = match node.op {
            Op::And => l & r,
            Op::Or => l | r,
            Op::IdNode(x) => bits::IntoBitHash::get_bit((data, x)),
        };
        match node.neg {
            true => !tmp,
            false => tmp,
        }
    }
}

fn check_distributivity(op: Op<u8>, node: Option<BtNode<u8>>) -> bool {
    if let Some(n) = node {
        match n.op {
            Op::IdNode(T) => false,
            _ => op != n.op,
        }
    } else {
        panic!("Null node")
    }
}

pub fn and_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    // let d1 = check_distributivity(Op::And, l.clone());
    // let d2 = check_distributivity(Op::And, r.clone());

    BtNode::new(Op::And, l, r, false)

    // if d1 && !d2 {
    //     if let Some(n) = r {
    //         println!("right = {n:?}");
    //         BtNode::new(
    //             Op::Or,
    //             Some(BtNode::new(Op::And, l.clone(), n.get_left_node(), false)),
    //             Some(BtNode::new(Op::And, l, n.get_right_node(), false)),
    //             false,
    //         )
    //     } else {
    //         panic!("")
    //     }
    // } else if d2 && !d1 {
    //     if let Some(n) = l {
    //         println!("left = {n:?}");
    //         BtNode::new(
    //             Op::Or,
    //             Some(BtNode::new(Op::And, r.clone(), n.get_left_node(), false)),
    //             Some(BtNode::new(Op::And, r, n.get_left_node(), false)),
    //             false,
    //         )
    //     } else {
    //         panic!("")
    //     }
    // } else {
    //     BtNode::new(Op::And, l, r, false)
    // }
}

pub fn or_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    // check_distributivity(Op::Or, l, r)
    BtNode::new(Op::Or, l, r, false)
}

pub fn xor_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(
        Op::Or,
        Some(and_node(l.clone(), Some(negation_node(r.clone())))),
        Some(and_node(Some(negation_node(l.clone())), r.clone())),
        false,
    )
}

pub fn imply_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Or, Some(negation_node(l)), r, false)
}

pub fn equal_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    // = <=> XNOR <=> !NOR
    BtNode::new(
        Op::Or,
        Some(and_node(l.clone(), r.clone())),
        Some(and_node(
            Some(negation_node(l.clone())),
            Some(negation_node(r.clone())),
        )),
        false,
    )
}

pub fn negation_node(node: Option<BtNode<u8>>) -> BtNode<u8> {
    if let Some(n) = node {
        if !n.neg == true && (n.op == Op::And || n.op == Op::Or) {
            BtNode::new(
                match n.op {
                    Op::And => Op::Or,
                    Op::Or => Op::And,
                    _ => n.op,
                },
                Some(negation_node(Some(*(n.left.unwrap())))),
                Some(negation_node(Some(*(n.right.unwrap())))),
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
