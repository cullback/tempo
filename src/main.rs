use pest::Parser;
use pest_derive::Parser;
use std::env;
use std::process;

mod abstract_syntax_tree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let content = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    let pest_parse_result = MyParser::parse(Rule::program, &content);

    match pest_parse_result {
        Ok(mut pairs) => {
            // `MyParser::parse` returns an iterator. For the `program` rule,
            // we expect exactly one pair if the entire input matches.
            let program_pair = pairs.next().unwrap_or_else(|| {
                // This case should ideally not happen if parsing was Ok and content is not empty
                // leading to a valid program structure (even an empty one).
                // If `program = _{ SOI ~ EOI }` (empty program), it should still yield one pair.
                eprintln!("Pest parse successful, but no top-level program pair found. This is unexpected.");
                // For an empty input string "", pest with `program = _{ SOI ~ EOI }`
                // will produce one `program` pair spanning the empty input.
                // If the grammar was `program = _{ SOI ~ assignment+ ~ EOI }`, then empty input would be a parse error.
                // Our current grammar `program = _{ SOI ~ (assignment)* ~ EOI }` handles empty input.
                process::exit(1); // Or handle as an empty program AST if appropriate
            });

            // Ensure it's the program rule, as expected
            if program_pair.as_rule() != Rule::program {
                eprintln!(
                    "Expected top-level rule to be 'program', but found {:?}. This is a bug.",
                    program_pair.as_rule()
                );
                process::exit(1);
            }

            // Ensure there are no other sibling pairs at the top level
            if pairs.next().is_some() {
                eprintln!("Pest parse successful, but found multiple top-level pairs. This is unexpected.");
                process::exit(1);
            }

            println!("\nPest parse successful. Building AST...");
            match abstract_syntax_tree::parse_program(program_pair) {
                Ok(ast_program) => {
                    println!("\nAST construction successful:");
                    println!("{:#?}", ast_program);
                }
                Err(e) => {
                    eprintln!("\nAST construction failed:");
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("\nPest parse failed:");
            eprintln!("{}", e); // This error message from Pest is usually very informative.
            process::exit(1);
        }
    }
}
