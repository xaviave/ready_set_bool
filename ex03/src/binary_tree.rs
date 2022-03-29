pub enum Op<T> {
    AndNode,
    OrNode,
    XorNode,
    ImplyNode,
    EqualNode,
    NegNode(T),
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

impl BtNode<bool> {
    pub fn new(op: Op<bool>, l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> Self {
        BtNode::<bool> {
            op: op, left: Some(Box::new(l.unwrap())), right: Some(Box::new(r.unwrap()))
        }
    }
}

impl BinaryTree<bool> {
    pub fn new(head: BtNode<bool>) -> Self {
        BinaryTree::<bool> { head: head }
    }

    pub fn collapse(node: &BtNode<bool>) -> bool {
        let mut r: Option<bool> = None;
        let mut l: Option<bool> = None;

        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }

        let r = if let Some(x) = r { x } else { false };
        let l = if let Some(x) = l { x } else { false };

        match node.op {
            Op::AndNode => { l & r }
            // Op::OrNode => { l | r }
            Op::OrNode => { println!("{} | {} = {}", l, r, l | r); l | r }
            Op::XorNode => { l ^ r }
            Op::ImplyNode => { if l == false || (l == true && r == true) { true } else { false } }
            Op::EqualNode => { println!("{} == {} = {}", l, r, l == r); l == r }
            // Op::EqualNode => { l == r }
            Op::NegNode(x) => { !x }
            Op::IdNode(x) => { x }
        }
    }
}

pub fn and_node(l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> BtNode<bool> {
    BtNode::new(Op::AndNode, l, r)
}

pub fn or_node(l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> BtNode<bool> {
    BtNode::new(Op::OrNode, l, r)
}

pub fn xor_node(l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> BtNode<bool> {
    BtNode::new(Op::XorNode, l, r)
}

pub fn imply_node(l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> BtNode<bool> {
    BtNode::new(Op::ImplyNode, l, r)
}

pub fn equal_node(l: Option<BtNode<bool>>, r: Option<BtNode<bool>>) -> BtNode<bool> {
    BtNode::new(Op::EqualNode, l, r)
}

pub fn negation_node(value: bool) -> BtNode<bool> {
    BtNode { op: Op::NegNode(value), left: None, right: None }
}

pub fn id_node(value: bool) -> BtNode<bool> {
    BtNode { op: Op::IdNode(value), left: None, right: None }
}