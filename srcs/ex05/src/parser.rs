use regex::Regex;
use itertools::Itertools;

use crate::binary_tree;

pub struct Parser {
    pub vars: Vec<char>,
    pub bt: binary_tree::BinaryTree<u8>
}

impl Parser {
    fn check_str(raw_str: &str) -> bool {
        let re_bad_char = Regex::new(r"[^A-Z|&=!<>^]").unwrap();
        re_bad_char.find(raw_str) == None
    }

    fn get_all_vars(raw_str: &str) -> Vec<char> {
        let mut unique_vars = raw_str.chars().filter(|x| x.is_ascii_uppercase()).sorted().collect::<Vec<char>>();
        unique_vars.dedup();
        unique_vars
    }

    fn parse_raw(raw_str: &str) -> binary_tree::BinaryTree<u8> {
        let mut stack = Vec::new();
        let re = Regex::new(r"(?P<v>[A-Z])|(?P<op>[|&=!>^])").unwrap();
        let caps = re.captures_iter(raw_str);

        for cap in caps {
            let group_name = {
                if cap.name("v") != None { "v" }
                else if cap.name("op") != None { "op" }
                else { "not defined" }
            };
            
            let tmp = match group_name {
                "v" => binary_tree::id_node(cap.name("v").map_or(u8::MAX, |m| m.as_str().as_bytes()[0] - 65)),
                "op" => {
                    assert!(!stack.is_empty());
                    let op = cap.name("op").map_or("", |m| m.as_str());

                    let r: Option<binary_tree::BtNode<u8>> = stack.pop();
                    let l: Option<binary_tree::BtNode<u8>> = {
                        if op != "!" { stack.pop() }
                        else { None }
                    };

                    if op != "!" && l.is_none() {
                        panic!("Missing variable after operator: '{}'", op);
                    }

                    match op {
                        "&" => { binary_tree::and_node(l, r) }
                        "|" => { binary_tree::or_node(l, r) }
                        "^" => { binary_tree::xor_node(l, r) }
                        "!" => { binary_tree::negation_node(r, None) }
                        ">" => { binary_tree::imply_node(l, r) }
                        "=" => { binary_tree::equal_node(l, r) }
                        _ => panic!("Error operator {}\n", op)
                    }
                },
                _ => panic!("Error regex group {}\n", group_name),
            };
            stack.push(tmp);
        }
        assert!(stack.len() == 1);
        binary_tree::BinaryTree::new(stack.pop().unwrap())
    }
    
    pub fn new(raw_str: &str) -> Self {
        if !Parser::check_str(raw_str) {
            panic!("Bad char found or typing error in formula");
        }
        Parser { vars: {Parser::get_all_vars(raw_str)}, bt: Parser::parse_raw(raw_str) }
    }

    pub fn resolve(&self, data: u32) -> bool {
        binary_tree::BinaryTree::collapse(&self.bt.head, data)
    }

    pub fn get_nnf(&self) -> String {
        binary_tree::BinaryTree::get_nnf(&self.bt.head)
    }
} 
