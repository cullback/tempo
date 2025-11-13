use pest::Parser;
use pest_derive::Parser;

use crate::ast::{
    Assignment, Block, Expression, FunctionCall, FunctionDefinition,
    Identifier, Number, Program, Span,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TempoParser;

pub fn parse_program(
    source: &str,
) -> Result<Program, Box<dyn std::error::Error>> {
    let pairs = TempoParser::parse(Rule::program, source)?;
    let span = Span::new(source);
    let mut assignments = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::assignment => {
                assignments.push(parse_assignment(pair, span)?);
            }
            Rule::EOI => {}
            _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
        }
    }

    Ok(Program { assignments, span })
}

fn parse_assignment<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Assignment<'a>, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let identifier = parse_identifier(inner.next().unwrap(), span)?;
    let expression = parse_expression(inner.next().unwrap(), span)?;

    Ok(Assignment {
        identifier,
        expression: Box::new(expression),
        span,
    })
}

fn parse_identifier<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Identifier<'a>, Box<dyn std::error::Error>> {
    Ok(Identifier {
        name: pair.as_str().to_string(),
        span,
    })
}

fn parse_expression<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Expression<'a>, Box<dyn std::error::Error>> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::number => {
            let value = inner.as_str().parse::<i64>()?;
            Ok(Expression::Number(Number { value, span }))
        }
        Rule::string_literal => {
            let s = inner.as_str();
            let unquoted = &s[1..s.len() - 1];
            let unescaped = unquoted
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            Ok(Expression::Identifier(Identifier {
                name: unescaped,
                span,
            }))
        }
        Rule::identifier => {
            Ok(Expression::Identifier(parse_identifier(inner, span)?))
        }
        Rule::function_call => parse_function_call(inner, span),
        Rule::function_definition => parse_function_definition(inner, span),
        Rule::block => parse_block(inner, span),
        _ => unreachable!("Unexpected expression rule: {:?}", inner.as_rule()),
    }
}

fn parse_function_call<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Expression<'a>, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let function_name = parse_identifier(inner.next().unwrap(), span)?;
    let mut arguments = Vec::new();

    if let Some(args_pair) = inner.next() {
        for arg in args_pair.into_inner() {
            arguments.push(parse_expression(arg, span)?);
        }
    }

    Ok(Expression::FunctionCall(FunctionCall {
        function_name,
        arguments,
        span,
    }))
}

fn parse_function_definition<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Expression<'a>, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let mut parameters = Vec::new();

    let first = inner.next().unwrap();

    let body_pair = if first.as_rule() == Rule::ident_list {
        for param in first.into_inner() {
            parameters.push(parse_identifier(param, span)?);
        }
        inner.next().unwrap()
    } else {
        first
    };

    let body = parse_expression(body_pair, span)?;

    Ok(Expression::FunctionDefinition(FunctionDefinition {
        parameters,
        body: Box::new(body),
        span,
    }))
}

fn parse_block<'a>(
    pair: pest::iterators::Pair<'a, Rule>,
    span: Span<'a>,
) -> Result<Expression<'a>, Box<dyn std::error::Error>> {
    let mut inner = pair.into_inner();
    let mut assignments = Vec::new();
    let mut expression = None;

    for item in inner {
        match item.as_rule() {
            Rule::assignment => {
                assignments.push(parse_assignment(item, span)?);
            }
            Rule::expression => {
                expression = Some(parse_expression(item, span)?);
            }
            _ => unreachable!("Unexpected block item: {:?}", item.as_rule()),
        }
    }

    Ok(Expression::Block(Block {
        assignments,
        expression: Box::new(expression.unwrap()),
        span,
    }))
}
