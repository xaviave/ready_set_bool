use crate::bits;

pub enum Op<T> {
    And,
    Or,
    Xor,
    Imply,
    Equal,
    Neg,
    IdNode(T)
}

pub struct BtNode<T> {
    left: Option<Box<BtNode<T>>>,
    right: Option<Box<BtNode<T>>>,
    op: Op<T>
}

pub struct BinaryTree<T> {
    pub head: BtNode<T>
}

impl BtNode<u8> {
    pub fn new(op: Op<u8>, l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> Self {
        BtNode::<u8> {
            op, left: l.map(Box::new), right: r.map(Box::new)
        }
    }
}

impl BinaryTree<u8> {
    pub fn new(head: BtNode<u8>) -> Self {
        BinaryTree::<u8> { head }
    }
    
    pub fn collapse(node: &BtNode<u8>, data: u32) -> bool {
        let mut r: Option<bool> = None;
        let mut l: Option<bool> = None;

        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left, data));
        }

        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right, data));
        }

        let r = if let Some(x) = r { bits::IntoBitHash::get_bit((data, x)) } else { false };
        let l = if let Some(x) = l { bits::IntoBitHash::get_bit((data, x)) } else { false };

        match node.op {
            Op::And => { l & r }
            Op::Or => { l | r }
            Op::Xor => { l ^ r }
            // Op::Imply => { !l || (l && r) }
            Op::Imply => { !l || r }
            Op::Equal => { l == r }
            Op::Neg => { !l }
            Op::IdNode(x) => { bits::IntoBitHash::get_bit((data, x)) }
        }
    }
}

pub fn and_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::And, l, r)
}

pub fn or_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Or, l, r)
}

pub fn xor_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Xor, l, r)
}

pub fn imply_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Imply, l, r)
}

pub fn equal_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Equal, l, r)
}

pub fn negation_node(l: Option<BtNode<u8>>, r: Option<BtNode<u8>>) -> BtNode<u8> {
    BtNode::new(Op::Neg, l, r)
}

pub fn id_node(value: u8) -> BtNode<u8> {
    BtNode { op: Op::IdNode(value), left: None, right: None }
}