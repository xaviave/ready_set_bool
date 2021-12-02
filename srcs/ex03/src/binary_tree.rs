mod binary_tree {
    enum Op<T> {
        AndNode,
        OrNode,
        XorNode,
        ImplyNode,
        EqualNode,
        IdNode(T)
    }

    struct BtNode<T> {
        left: Option<Box<BtNode<T>>>,
        right: Option<Box<BtNode<T>>>,
        op: Op<T>
    }

    struct BinaryTree<T> {
        head: Option<BtNode<T>>
    }

    impl BtNode<bool> {
        pub fn new(op: Op<bool>, l: BtNode<bool>, r :BtNode<bool>) -> Self {
            BtNode::<bool> {
                op: op, left: Some(Box::new(l)), right: Some(Box::new(r))
            }
        }
    }

    impl BinaryTree<bool> {
        pub fn new(head: BtNode<bool>) -> Self {
            BinaryTree::<bool> { head: Some(head) }
        }

        pub fn collapse(node: &Box<BtNode<bool>>) -> bool {
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
                Op::OrNode => { l | r }
                Op::XorNode => { l ^ r }
                Op::ImplyNode => { if l == true && r == false { false } else { true } }
                Op::EqualNode => { l == r }
                Op::IdNode(x) => { x }
            }
        }
    }

    fn and_node(l: BtNode<bool>, r: BtNode<bool>) -> BtNode<bool> {
        BtNode::new(Op::AndNode, l, r)
    }

    fn or_node(l: BtNode<bool>, r: BtNode<bool>) -> BtNode<bool> {
        BtNode::new(Op::OrNode, l, r)
    }

    fn xor_node(l: BtNode<bool>, r: BtNode<bool>) -> BtNode<bool> {
        BtNode::new(Op::XorNode, l, r)
    }

    fn imply_node(l: BtNode<bool>, r: BtNode<bool>) -> BtNode<bool> {
        BtNode::new(Op::ImplyNode, l, r)
    }

    fn equal_node(l: BtNode<bool>, r: BtNode<bool>) -> BtNode<bool> {
        BtNode::new(Op::EqualNode, l, r)
    }

    fn id_node(value: bool) -> BtNode<bool> {
        BtNode { op: Op::IdNode(value), left: None, right: None }
    }
}