use itertools::Itertools;
use regex::Regex;

extern crate pest;

use pest::Parser;
#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct FormulaParser;

use crate::binary_tree::{
    and_node, equal_node, id_node, imply_node, negation_node, or_node, xor_node, BinaryTree, BtNode,
};

pub struct ParserA {
    pub vars: Vec<char>,
    pub bt: BinaryTree<u8>,
}

impl ParserA {
    fn check_str(raw_str: &str) -> bool {
        let re_bad_char = Regex::new(r"[^A-Z|&=!<>^]").unwrap();
        re_bad_char.find(raw_str) == None
    }

    fn get_all_vars(raw_str: &str) -> Vec<char> {
        let mut unique_vars = raw_str
            .chars()
            .filter(|x| x.is_ascii_uppercase())
            .sorted()
            .collect::<Vec<char>>();
        unique_vars.dedup();
        unique_vars
    }

    fn get_operator_node(
        pair: pest::iterators::Pair<Rule>,
        stack: &mut Vec<BtNode<u8>>,
    ) -> BtNode<u8> {
        let p = pair.into_inner().next().unwrap();
        // https://stackoverflow.com/questions/47618823/cannot-borrow-as-mutable-because-it-is-also-borrowed-as-immutable
        match p.as_rule() {
            Rule::neg => negation_node(stack.pop()),
            Rule::operator => {
                if stack.len() < 2 {
                    panic!("Error too many operator");
                }
                let r: Option<BtNode<u8>> = stack.pop();
                let l: Option<BtNode<u8>> = stack.pop();

                match p.as_str() {
                    "&" => and_node(l, r),
                    "|" => or_node(l, r),
                    "^" => xor_node(l, r),
                    ">" => imply_node(l, r),
                    "=" => equal_node(l, r),
                    _ => panic!("Error operator {}\n", p),
                }
            }
            _ => panic!("Error operator {}\n", p),
        }
    }

    fn parse_raw(raw_str: &str) -> BinaryTree<u8> {
        println!("formula = {raw_str:}");
        let mut stack = Vec::<BtNode<u8>>::new();
        let pairs = FormulaParser::parse(Rule::formula, raw_str)
            .unwrap_or_else(|e| panic!("Error during parsing: {:?}", e));

        for pair in pairs {
            for inner_pair in pair.into_inner() {
                let tmp = match inner_pair.as_rule() {
                    Rule::var => id_node(inner_pair.as_str().as_bytes()[0] - 65),
                    Rule::operators => ParserA::get_operator_node(inner_pair, &mut stack),
                    Rule::EOI => break,
                    _ => panic!("Error token not accepted: {:#?}", inner_pair),
                };
                stack.push(tmp);
            }
        }
        assert!(stack.len() == 1);
        BinaryTree::new(stack.pop().unwrap())
    }

    pub fn new(raw_str: &str) -> Self {
        if !ParserA::check_str(raw_str) {
            panic!("Bad char found or typing error in formula");
        }
        ParserA {
            vars: { ParserA::get_all_vars(raw_str) },
            bt: ParserA::parse_raw(raw_str),
        }
    }

    pub fn resolve(&self, data: u32) -> bool {
        BinaryTree::collapse(&self.bt.head, data)
    }

    pub fn get_nnf(&self) -> String {
        BinaryTree::collapse_printer(&self.bt.head)
    }
}
