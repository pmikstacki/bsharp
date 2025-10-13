use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_var;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::{map, verify},
    multi::separated_list1,
    sequence::tuple,
};
use syntax::expressions::{DeconstructionExpression, DeconstructionTarget};
use nom::character::complete::char as nom_char;
use nom::character::complete::satisfy;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;

/// Parse a deconstruction expression: (var x, var y) = tuple
pub fn parse_deconstruction_expression(input: Span) -> BResult<DeconstructionExpression> {
    map(
        tuple((
            parse_deconstruction_targets,
            delimited(ws, satisfy(|c| c == '='), ws),
            delimited(ws, parse_expression, ws),
        )),
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
            delimited(ws, nom_char('('), ws),
            verify(
                separated_list1(
                    |i| delimited(ws, satisfy(|c| c == ','), ws).parse(i),
                    |i| delimited(ws, parse_deconstruction_target, ws).parse(i),
                ),
                |targets: &Vec<DeconstructionTarget>| targets.len() >= 2,
            ),
            cut(delimited(ws, nom_char(')'), ws)),
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
        map(delimited(ws, satisfy(|c| c == '_'), ws), |_| DeconstructionTarget::Discard),
        // Nested deconstruction: (var a, var b)
        map(parse_deconstruction_targets, |targets| {
            DeconstructionTarget::Nested(targets)
        }),
        // Variable declaration with 'var': var x
        map(tuple((delimited(ws, kw_var(), ws), delimited(ws, parse_identifier, ws))), |(_, name)| {
            DeconstructionTarget::Declaration {
                variable_type: None,
                name,
                is_var: true,
            }
        }),
        // Variable declaration with explicit type: int x, string y
        map(
            tuple((delimited(ws, parse_type_expression, ws), delimited(ws, parse_identifier, ws))),
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
