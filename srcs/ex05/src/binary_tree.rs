use crate::bits;
use std::str;

#[derive(Clone)]
pub enum Op<T> {
    And,
    Or,
    Xor,
    IdNode(T),
}

#[derive(Clone)]
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
    pub fn get_nnf(node: &BtNode<u8>) -> String {
        format!("sheesh")
    }
    pub fn get_cnf(node: &BtNode<u8>) -> String {
        format!("sheesh")
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
            Op::Xor => l ^ r,
            Op::IdNode(x) => bits::IntoBitHash::get_bit((data, x))
        };
        match node.neg {
            true => !tmp,
            false => tmp
        }
    }
}

pub fn and_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::And, l, r, false)
}

pub fn or_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Or, l, r, false)
}

pub fn xor_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Xor, l, r, false)
}

pub fn imply_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Or, Some(negation_node(l)), r, false)
}

pub fn equal_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::And, l, r, false)
}

pub fn negation_node(node: Option<BtNode<u8>>) -> BtNode<u8> {
    if let Some(n) = node {
        BtNode {
            op: n.op,
            left: n.left,
            right: n.right,
            neg: !n.neg,
        }
    } else {
        panic!("Negation can't be applied to a Null node");
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
