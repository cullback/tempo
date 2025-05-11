use pest::Parser;
use pest_derive::Parser;
use std::env;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let content = std::fs::read_to_string(&args[1]).expect("file does not exist");

    let tree = MyParser::parse(Rule::program, &content);

    println!("{tree:?}");
}
