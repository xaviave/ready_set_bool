extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct FormulaParser;

/*
AB>
AB^
AB<
AB=
AB&!
AB|!
AB|C&
AB|C&!
AB|!C!&
AB&!C!|
AB&C&D&
AB|C|D|
*/

fn main() {
    let pairs = FormulaParser::parse(Rule::formula, "AB|C&!DA>C^=").unwrap_or_else(|e| panic!("Error during parsing: {e:?}"));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::var => println!("VAR:   {:?}", inner_pair.as_str()),
                Rule::operators => {
                    let a = &inner_pair.into_inner().next().unwrap();
                    match a.as_rule() {
                            Rule::neg => println!("NEG:   {}", a.as_str()),
                            Rule::operator => println!("OP:   {}", a.as_str()),
                            _ => {}
                        }
                    }
                // Rule::operators => println!("OP:   {:?}", inner_pair.into_inner().next().unwrap().as_rule()),
                _ => {}
            };
        }
    }
}
