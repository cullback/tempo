use pest::Parser;
use pest_derive::Parser;
use std::env;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

// Function to recursively print the parse tree
fn print_pair(pair: pest::iterators::Pair<Rule>, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    // Print the rule and its matched string
    println!("{}{:?}: \"{}\"", indent, pair.as_rule(), pair.as_str());

    // Recursively print inner pairs
    for inner_pair in pair.into_inner() {
        print_pair(inner_pair, indent_level + 1);
    }
}

fn main() {
    println!("Hello, world!"); // This can be kept or removed based on preference

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let content = match std::fs::read_to_string(&args[1]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", &args[1], e);
            return;
        }
    };

    let parse_result = MyParser::parse(Rule::program, &content);

    match parse_result {
        Ok(pairs) => {
            println!("\nParse successful. Tree structure:");
            // The 'program' rule is the entry point, so we iterate through its pairs.
            // Usually, there's one top-level pair if the whole input matches 'program'.
            for pair in pairs {
                print_pair(pair, 0);
            }
        }
        Err(e) => {
            eprintln!("\nParse failed:");
            eprintln!("{}", e);
        }
    }
}
