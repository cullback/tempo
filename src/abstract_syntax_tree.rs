use pest::{
    Span,
    iterators::{Pair, Pairs},
};

// AST Node types
#[derive(Debug)]
pub enum AstNode<'a> {
    Program(Program<'a>),
    Assignment(Assignment<'a>),
    Expression(Expression<'a>),
    Identifier(Identifier<'a>),
    Number(Number<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
pub struct Program<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Assignment<'a> {
    pub identifier: Identifier<'a>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Identifier<'a> {
    pub name: String,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Number<'a> {
    pub value: i64,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub enum Expression<'a> {
    Number(Number<'a>),
    Identifier(Identifier<'a>),
    FunctionCall(FunctionCall<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    Block(Block<'a>),
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    pub function_name: Identifier<'a>,
    pub arguments: Vec<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct FunctionDefinition<'a> {
    pub parameters: Vec<Identifier<'a>>,
    pub body: Box<Expression<'a>>,
    pub span: Span<'a>,
}

#[derive(Debug)]
pub struct Block<'a> {
    pub assignments: Vec<Assignment<'a>>,
    pub expression: Box<Expression<'a>>,
    pub span: Span<'a>,
}

// Helper to extract the next inner pair or return an error
fn next_inner_or_err<'a>(
    pairs: &mut pest::iterators::Pairs<'a, crate::Rule>,
    expected_rule_name: &str,
) -> Result<Pair<'a, crate::Rule>, String> {
    pairs
        .next()
        .ok_or_else(|| format!("Expected {} but found nothing", expected_rule_name))
}

// Parsing functions

pub fn parse_identifier<'a>(pair: Pair<'a, crate::Rule>) -> Result<Identifier<'a>, String> {
    if pair.as_rule() != crate::Rule::identifier {
        return Err(format!(
            "Expected identifier, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    Ok(Identifier {
        name: pair.as_str().to_string(),
        span: pair.as_span(),
    })
}

fn parse_number<'a>(pair: Pair<'a, crate::Rule>) -> Result<Number<'a>, String> {
    if pair.as_rule() != crate::Rule::number {
        return Err(format!(
            "Expected number, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    let value = pair
        .as_str()
        .parse::<i64>()
        .map_err(|e| format!("Failed to parse number '{}': {}", pair.as_str(), e))?;
    Ok(Number {
        value,
        span: pair.as_span(),
    })
}

fn parse_function_call<'a>(pair: Pair<'a, crate::Rule>) -> Result<FunctionCall<'a>, String> {
    if pair.as_rule() != crate::Rule::function_call {
        return Err(format!(
            "Expected function_call, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    let fn_call_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let ident_pair = next_inner_or_err(&mut inner_pairs, "function_call identifier")?;
    let function_name = parse_identifier(ident_pair)?;

    let args_pair = next_inner_or_err(&mut inner_pairs, "function_call arguments")?;
    if args_pair.as_rule() != crate::Rule::function_arguments {
        return Err(format!(
            "Expected function_arguments, got {:?}",
            args_pair.as_rule()
        ));
    }

    let mut arguments = Vec::new();
    for arg_expr_pair in args_pair.into_inner() {
        arguments.push(parse_expression(arg_expr_pair)?);
    }

    Ok(FunctionCall {
        function_name,
        arguments,
        span: fn_call_span,
    })
}

fn parse_function_definition<'a>(
    pair: Pair<'a, crate::Rule>,
) -> Result<FunctionDefinition<'a>, String> {
    if pair.as_rule() != crate::Rule::function_definition {
        return Err(format!(
            "Expected function_definition, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    let fn_def_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let params_list_pair = next_inner_or_err(
        &mut inner_pairs,
        "function_definition parameters (ident_list)",
    )?;
    if params_list_pair.as_rule() != crate::Rule::ident_list {
        return Err(format!(
            "Expected ident_list for parameters, got {:?}",
            params_list_pair.as_rule()
        ));
    }
    let mut parameters = Vec::new();
    for ident_pair in params_list_pair.into_inner() {
        parameters.push(parse_identifier(ident_pair)?);
    }

    let body_expr_pair =
        next_inner_or_err(&mut inner_pairs, "function_definition body expression")?;
    let body = parse_expression(body_expr_pair)?;

    Ok(FunctionDefinition {
        parameters,
        body: Box::new(body),
        span: fn_def_span,
    })
}

fn parse_block<'a>(pair: Pair<'a, crate::Rule>) -> Result<Block<'a>, String> {
    if pair.as_rule() != crate::Rule::block {
        return Err(format!(
            "Expected block, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    let block_span = pair.as_span();
    let mut inner_pairs = pair.into_inner().peekable();
    let mut assignments = Vec::new();

    while let Some(peeked_pair) = inner_pairs.peek() {
        if peeked_pair.as_rule() == crate::Rule::assignment {
            let assignment_pair = inner_pairs.next().unwrap(); // Consume it
            assignments.push(parse_assignment(assignment_pair)?);
        } else {
            break; // Next should be the expression
        }
    }

    let expression_pair = inner_pairs
        .next()
        .ok_or_else(|| "Block: expected expression after assignments".to_string())?;
    let expression = parse_expression(expression_pair)?;

    if inner_pairs.next().is_some() {
        return Err("Block: unexpected extra pairs after expression".to_string());
    }

    Ok(Block {
        assignments,
        expression: Box::new(expression),
        span: block_span,
    })
}

pub fn parse_expression<'a>(pair: Pair<'a, crate::Rule>) -> Result<Expression<'a>, String> {
    if pair.as_rule() != crate::Rule::expression {
        return Err(format!(
            "Expected expression, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    // An 'expression' rule always contains exactly one inner specific expression type.
    let inner_expr_pair = pair
        .into_inner()
        .next()
        .ok_or_else(|| "Expression rule was unexpectedly empty".to_string())?;

    match inner_expr_pair.as_rule() {
        crate::Rule::number => Ok(Expression::Number(parse_number(inner_expr_pair)?)),
        crate::Rule::identifier => Ok(Expression::Identifier(parse_identifier(inner_expr_pair)?)),
        crate::Rule::function_call => Ok(Expression::FunctionCall(parse_function_call(
            inner_expr_pair,
        )?)),
        crate::Rule::function_definition => Ok(Expression::FunctionDefinition(
            parse_function_definition(inner_expr_pair)?,
        )),
        crate::Rule::block => Ok(Expression::Block(parse_block(inner_expr_pair)?)),
        _ => Err(format!(
            "Unexpected rule {:?} inside expression for \"{}\"",
            inner_expr_pair.as_rule(),
            inner_expr_pair.as_str()
        )),
    }
}

fn parse_assignment<'a>(pair: Pair<'a, crate::Rule>) -> Result<Assignment<'a>, String> {
    if pair.as_rule() != crate::Rule::assignment {
        return Err(format!(
            "Expected assignment, got {:?} for \"{}\"",
            pair.as_rule(),
            pair.as_str()
        ));
    }
    let assignment_span = pair.as_span();
    let mut inner_pairs = pair.into_inner();

    let ident_pair = next_inner_or_err(&mut inner_pairs, "assignment identifier")?;
    let identifier = parse_identifier(ident_pair)?;

    let expr_pair = next_inner_or_err(&mut inner_pairs, "assignment expression")?;
    let expression = parse_expression(expr_pair)?;

    Ok(Assignment {
        identifier,
        expression: Box::new(expression),
        span: assignment_span,
    })
}

pub fn parse_program<'a>(mut program_level_pairs: Pairs<'a, crate::Rule>) -> Result<Program<'a>, String> {
    // When parsing Rule::program, Pest returns a Pairs iterator that should yield
    // exactly one Pair, corresponding to the 'program' rule itself.
    let program_pair = program_level_pairs.next()
        .ok_or_else(|| "Expected a program pair from parser, but found none.".to_string())?;

    // Validate that this pair is indeed for the 'program' rule.
    if program_pair.as_rule() != crate::Rule::program {
        return Err(format!(
            "Expected top-level pair to be Rule::program, but got {:?} for \"{}\"",
            program_pair.as_rule(),
            program_pair.as_str()
        ));
    }

    // Ensure there are no other sibling pairs at this top level.
    if program_level_pairs.next().is_some() {
        return Err("Expected only one top-level program pair, but found more. This indicates an issue with the parser or grammar.".to_string());
    }

    // The span of the 'program_pair' is the span of the entire program.
    let program_span = program_pair.as_span();

    let mut assignments = Vec::new();
    // The `program` rule is `SOI ~ (assignment)* ~ EOI`.
    // `SOI` and `EOI` are silent (`_`).
    // `program_pair.into_inner()` will yield the sequence of `assignment` pairs.
    for inner_pair in program_pair.into_inner() {
        // Each inner_pair must be an assignment.
        if inner_pair.as_rule() == crate::Rule::assignment {
            assignments.push(parse_assignment(inner_pair)?);
        } else {
            // This case should not be reached if the grammar is correctly defined
            // and Pest processes it as expected, as only 'assignment' rules are
            // non-silent children of 'program'.
            return Err(format!(
                "Unexpected rule {:?} found inside program structure. Expected only assignments. Rule content: \"{}\"",
                inner_pair.as_rule(),
                inner_pair.as_str()
            ));
        }
    }

    Ok(Program {
        assignments,
        span: program_span, // Use the correct span for the entire program
    })
}
