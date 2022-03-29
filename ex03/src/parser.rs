use regex::Regex;

use crate::binary_tree;

pub struct Parser {
    bt: binary_tree::BinaryTree<bool>
}

impl Parser {
    fn check_str(raw_str: &str) -> bool {
        let re_bad_char = Regex::new(r"[^01|&=!<>^]").unwrap();
        if re_bad_char.find(raw_str) != None { false } else { true }
    }

    fn parse_raw(raw_str: &str) -> binary_tree::BinaryTree<bool> {
        if !Parser::check_str(raw_str) {
            panic!("Bad char found or typing error in formula");
        }

        let mut stack = Vec::new();
        let re = Regex::new(r"(?P<v>[01])|(?P<op>[|&=!>^])").unwrap();
        let caps = re.captures_iter(raw_str);

        for cap in caps {
            let group_name = {
                if cap.name("v") != None { "v" }
                else if cap.name("op") != None { "op" }
                else { "not defined" }
            };
            
            let tmp = match group_name {
                "v" => {
                    let value = cap.name("v").map_or("", |m| m.as_str());
                    binary_tree::id_node( if value == "1" { true } else { false } )
                },
                "op" => {
                    assert_eq!(stack.len() > 0, true);
                    let op = cap.name("op").map_or("", |m| m.as_str());

                    let r: Option<binary_tree::BtNode<bool>> = stack.pop();
                    let l: Option<binary_tree::BtNode<bool>> = {
                        if op != "!" { stack.pop() }
                        else { None }
                    };

                    match op {
                        "&" => { binary_tree::and_node(l, r) }
                        "|" => { binary_tree::or_node(l, r) }
                        "^" => { binary_tree::xor_node(l, r) }
                        "!" => { binary_tree::negation_node(binary_tree::BinaryTree::collapse(&r.unwrap())) }
                        ">" => { binary_tree::imply_node(l, r) }
                        "=" => { binary_tree::equal_node(l, r) }
                        _ => panic!("Error operator {}\n", op)
                    }
                },
                _ => panic!("Error regex group {}\n", group_name),
            };
            stack.push(tmp);
        }
        assert_eq!(stack.len() == 1, true);
        binary_tree::BinaryTree::new(stack.pop().unwrap())
    }
    
    pub fn new(raw_str: &str) -> Self {
        Parser { bt: Parser::parse_raw(raw_str) }
    }

    pub fn resolve(&self) -> bool {
        binary_tree::BinaryTree::collapse(&self.bt.head)
    }
} 
// // https://www.geeksforgeeks.org/expression-tree
// let b = binary_tree::BinaryTree::new(imply_node(id_node(false), id_node(true)));
// println!("{} = {}", binary_tree::BinaryTree::collapse(&Box::new(b.head.expect("No head initialized"))), false);

