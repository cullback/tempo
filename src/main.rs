use abstract_syntax_tree::parse_program;
use pest::Parser;
use pest_derive::Parser;
use std::env;

mod abstract_syntax_tree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = std::fs::read_to_string(&args[1]).unwrap();
    let pairs = MyParser::parse(Rule::program, &content).unwrap();

    let ast = parse_program(pairs);

    println!("{ast:#?}");
}
