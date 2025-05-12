use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::env;

// Import AST structs
use crate::abstract_syntax_tree::{
    Assignment, Block, Expression, FunctionCall, FunctionDefinition, Identifier, Number, Program,
};

mod abstract_syntax_tree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

// AST Builder Functions

fn build_ast_from_identifier_pair(pair: Pair<Rule>) -> Identifier {
    assert_eq!(pair.as_rule(), Rule::identifier);
    Identifier {
        name: pair.as_str().to_string(),
        span: pair.as_span(),
    }
}

fn build_ast_from_number_pair(pair: Pair<Rule>) -> Number {
    assert_eq!(pair.as_rule(), Rule::number);
    Number {
        value: pair
            .as_str()
            .parse::<i64>()
            .expect("Failed to parse number"),
        span: pair.as_span(),
    }
}

fn build_ast_from_function_call_pair(pair: Pair<Rule>) -> FunctionCall {
    assert_eq!(pair.as_rule(), Rule::function_call);
    let fn_call_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let function_name_pair = inner_pairs
        .next()
        .expect("Expected function name in function call");
    let function_name = build_ast_from_identifier_pair(function_name_pair);

    let mut arguments = Vec::new();
    // Check for optional function_arguments
    if let Some(args_pair) = inner_pairs.next() {
        assert_eq!(args_pair.as_rule(), Rule::function_arguments);
        for arg_expr_pair in args_pair.into_inner() {
            arguments.push(build_ast_from_expression_pair(arg_expr_pair));
        }
    }

    FunctionCall {
        function_name,
        arguments,
        span: fn_call_span,
    }
}

fn build_ast_from_function_definition_pair(pair: Pair<Rule>) -> FunctionDefinition {
    assert_eq!(pair.as_rule(), Rule::function_definition);
    let fn_def_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let mut parameters = Vec::new();
    // First inner pair could be ident_list (if params exist) or expression (body, if no params)
    let mut next_inner = inner_pairs
        .next()
        .expect("Expected ident_list or expression in function definition");

    if next_inner.as_rule() == Rule::ident_list {
        for ident_pair in next_inner.into_inner() {
            parameters.push(build_ast_from_identifier_pair(ident_pair));
        }
        // After ident_list, the next pair must be the body expression
        next_inner = inner_pairs
            .next()
            .expect("Expected expression body after parameters in function definition");
    }
    // At this point, next_inner must be the body expression
    assert_eq!(next_inner.as_rule(), Rule::expression);
    let body = build_ast_from_expression_pair(next_inner);

    FunctionDefinition {
        parameters,
        body: Box::new(body),
        span: fn_def_span,
    }
}

fn build_ast_from_block_pair(pair: Pair<Rule>) -> Block {
    assert_eq!(pair.as_rule(), Rule::block);
    let block_span = pair.as_span();
    let mut inner_pairs = pair.into_inner().peekable();

    let mut assignments = Vec::new();
    while let Some(peeked_pair) = inner_pairs.peek() {
        if peeked_pair.as_rule() == Rule::assignment {
            let assignment_pair = inner_pairs.next().unwrap(); // Consume it
            assignments.push(build_ast_from_assignment_pair(assignment_pair));
        } else {
            break; // Next should be the expression
        }
    }

    let expression_pair = inner_pairs
        .next()
        .expect("Block: expected expression after assignments");
    assert_eq!(expression_pair.as_rule(), Rule::expression);
    let expression = build_ast_from_expression_pair(expression_pair);

    assert!(
        inner_pairs.next().is_none(),
        "Block: unexpected extra pairs after expression"
    );

    Block {
        assignments,
        expression: Box::new(expression),
        span: block_span,
    }
}

fn build_ast_from_expression_pair(pair: Pair<Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::expression);
    // The expression rule contains one specific expression type as its inner rule
    let inner_expr_pair = pair
        .into_inner()
        .next()
        .expect("Expression rule should have one inner rule");

    match inner_expr_pair.as_rule() {
        Rule::number => Expression::Number(build_ast_from_number_pair(inner_expr_pair)),
        Rule::identifier => Expression::Identifier(build_ast_from_identifier_pair(inner_expr_pair)),
        Rule::function_call => {
            Expression::FunctionCall(build_ast_from_function_call_pair(inner_expr_pair))
        }
        Rule::function_definition => Expression::FunctionDefinition(
            build_ast_from_function_definition_pair(inner_expr_pair),
        ),
        Rule::block => Expression::Block(build_ast_from_block_pair(inner_expr_pair)),
        _ => unreachable!(
            "Unexpected rule in expression: {:?}",
            inner_expr_pair.as_rule()
        ),
    }
}

fn build_ast_from_assignment_pair(pair: Pair<Rule>) -> Assignment {
    assert_eq!(pair.as_rule(), Rule::assignment);
    let assignment_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let identifier_pair = inner_pairs
        .next()
        .expect("Expected identifier in assignment");
    let identifier = build_ast_from_identifier_pair(identifier_pair);

    let expression_pair = inner_pairs
        .next()
        .expect("Expected expression in assignment");
    let expression = build_ast_from_expression_pair(expression_pair);

    Assignment {
        identifier,
        expression: Box::new(expression),
        span: assignment_span,
    }
}

fn build_ast_from_program_pair(pair: Pair<Rule>) -> Program {
    assert_eq!(pair.as_rule(), Rule::program);
    let program_span = pair.as_span();
    let mut assignments = Vec::new();

    // Inner pairs of 'program' are the 'assignment's
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::assignment => {
                assignments.push(build_ast_from_assignment_pair(inner_pair));
            }
            // EOI is part of the program rule structure but not an inner pair here
            _ => unreachable!(
                "Unexpected rule in program's inner pairs: {:?}",
                inner_pair.as_rule()
            ),
        }
    }
    Program {
        assignments,
        span: program_span,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    let parse_result = MyParser::parse(Rule::program, &content);

    match parse_result {
        Ok(mut pairs) => {
            // MyParser::parse returns an iterator, usually with one item for the top rule.
            if let Some(program_pair) = pairs.next() {
                assert_eq!(program_pair.as_rule(), Rule::program);
                let ast = build_ast_from_program_pair(program_pair);
                println!("\nAST:\n{:#?}", ast);

                // Ensure no other top-level pairs exist, which would be unexpected.
                assert!(
                    pairs.next().is_none(),
                    "Expected only one top-level program pair"
                );
            } else {
                // This case implies successful parse of an empty structure,
                // which should still yield a program_pair for the 'program' rule.
                // If 'program' rule itself could be empty and not match, pest would error.
                // So, this branch might indicate an issue if reached.
                println!("\nParse successful, but no program pair found (empty input or grammar issue?).");
                // An empty input string "" correctly parses into an empty Program AST.
                // Program { assignments: [], span: Span { str: "", start: 0, end: 0 } }
            }
        }
        Err(e) => {
            eprintln!("\nParse failed:");
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
