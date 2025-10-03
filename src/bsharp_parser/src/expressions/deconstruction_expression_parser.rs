use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::contextual_misc_keywords::kw_var;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{DeconstructionExpression, DeconstructionTarget};
use crate::syntax::parser_helpers::{bchar, bws, context};

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::{map, verify},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

/// Parse a deconstruction expression: (var x, var y) = tuple
pub fn parse_deconstruction_expression(input: &str) -> BResult<&str, DeconstructionExpression> {
    context(
        "deconstruction expression",
        map(
            tuple((
                parse_deconstruction_targets,
                bws(bchar('=')),
                bws(parse_expression),
            )),
            |(targets, _, value)| DeconstructionExpression {
                targets,
                value: Box::new(value),
            },
        ),
    )(input)
}

/// Parse deconstruction targets: (var x, var y) or (int a, string b)
fn parse_deconstruction_targets(input: &str) -> BResult<&str, Vec<DeconstructionTarget>> {
    context(
        "deconstruction targets",
        delimited(
            bws(bchar('(')),
            verify(
                separated_list1(bws(bchar(',')), bws(parse_deconstruction_target)),
                |targets: &Vec<DeconstructionTarget>| targets.len() >= 2,
            ),
            cut(bws(bchar(')'))),
        ),
    )(input)
}

/// Parse a single deconstruction target
fn parse_deconstruction_target(input: &str) -> BResult<&str, DeconstructionTarget> {
    context(
        "deconstruction target",
        alt((
            // Discard: _
            map(bchar('_'), |_| DeconstructionTarget::Discard),
            // Nested deconstruction: (var a, var b)
            map(parse_deconstruction_targets, |targets| {
                DeconstructionTarget::Nested(targets)
            }),
            // Variable declaration with 'var': var x
            map(tuple((kw_var(), bws(parse_identifier))), |(_, name)| {
                DeconstructionTarget::Declaration {
                    variable_type: None,
                    name,
                    is_var: true,
                }
            }),
            // Variable declaration with explicit type: int x, string y
            map(
                tuple((bws(parse_type_expression), bws(parse_identifier))),
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
        )),
    )(input)
}
