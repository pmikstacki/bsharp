use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_var;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::Parser;
use nom::character::complete::satisfy;
use nom::combinator::cut;
use nom::sequence::delimited;
use nom::{
    branch::alt,
    combinator::{map, verify},
    multi::separated_list1,
};
use nom_supreme::ParserExt;
use syntax::expressions::{DeconstructionExpression, DeconstructionTarget};

/// Parse a deconstruction expression: (var x, var y) = tuple
pub fn parse_deconstruction_expression(input: Span) -> BResult<DeconstructionExpression> {
    map(
        (
            parse_deconstruction_targets,
            delimited(ws, tok_assign(), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(targets, _, value)| DeconstructionExpression {
            targets,
            value: Box::new(value),
        },
    )
    .context("deconstruction expression")
    .parse(input)
}

/// Parse deconstruction targets: (var x, var y) or (int a, string b)
fn parse_deconstruction_targets(input: Span) -> BResult<Vec<DeconstructionTarget>> {
    delimited(
        ws,
        delimited(
            delimited(ws, tok_l_paren(), ws),
            verify(
                separated_list1(
                    |i| delimited(ws, tok_comma(), ws).parse(i),
                    |i| delimited(ws, parse_deconstruction_target, ws).parse(i),
                ),
                |targets: &Vec<DeconstructionTarget>| targets.len() >= 2,
            ),
            cut(delimited(ws, tok_r_paren(), ws)),
        ),
        ws,
    )
    .context("deconstruction targets")
    .parse(input)
}

/// Parse a single deconstruction target
fn parse_deconstruction_target(input: Span) -> BResult<DeconstructionTarget> {
    alt((
        // Discard: _
        map(delimited(ws, satisfy(|c| c == '_'), ws), |_| {
            DeconstructionTarget::Discard
        }),
        // Nested deconstruction: (var a, var b)
        map(parse_deconstruction_targets, |targets| {
            DeconstructionTarget::Nested(targets)
        }),
        // Variable declaration with 'var': var x
        map(
            (
                delimited(ws, kw_var(), ws),
                delimited(ws, parse_identifier, ws),
            ),
            |(_, name)| DeconstructionTarget::Declaration {
                variable_type: None,
                name,
                is_var: true,
            },
        ),
        // Variable declaration with explicit type: int x, string y
        map(
            (
                delimited(ws, parse_type_expression, ws),
                delimited(ws, parse_identifier, ws),
            ),
            |(variable_type, name)| DeconstructionTarget::Declaration {
                variable_type: Some(variable_type),
                name,
                is_var: false,
            },
        ),
        // Existing variable: existingVar
        map(parse_identifier, |name| {
            DeconstructionTarget::Variable(name)
        }),
    ))
    .context("deconstruction target")
    .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::assignment::tok_assign;
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::separators::tok_comma;
