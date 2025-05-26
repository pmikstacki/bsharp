use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::{DeconstructionExpression, DeconstructionTarget};
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword, nom_to_bs};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    branch::alt,
    character::complete::char as nom_char,
    combinator::{map, verify},
    multi::separated_list1,
    sequence::{delimited, tuple},
};

/// Parse a deconstruction expression: (var x, var y) = tuple
pub fn parse_deconstruction_expression(input: &str) -> BResult<&str, DeconstructionExpression> {
    bs_context(
        "deconstruction expression",
        map(
            tuple((
                parse_deconstruction_targets,
                bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('='))),
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
    bs_context(
        "deconstruction targets",
        delimited(
            bchar('('),
            verify(
                separated_list1(bws(bchar(',')), bws(parse_deconstruction_target)),
                |targets: &Vec<DeconstructionTarget>| targets.len() >= 2
            ),
            bws(bchar(')')),
        ),
    )(input)
}

/// Parse a single deconstruction target
fn parse_deconstruction_target(input: &str) -> BResult<&str, DeconstructionTarget> {
    bs_context(
        "deconstruction target",
        alt((
            // Discard: _
            map(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('_')), |_| DeconstructionTarget::Discard),
            // Nested deconstruction: (var a, var b)
            map(parse_deconstruction_targets, |targets| {
                DeconstructionTarget::Nested(targets)
            }),
            // Variable declaration with 'var': var x
            map(
                tuple((keyword("var"), bws(parse_identifier))),
                |(_, name)| DeconstructionTarget::Declaration {
                    variable_type: None,
                    name,
                    is_var: true,
                },
            ),
            // Variable declaration with explicit type: int x, string y
            map(
                tuple((
                    bws(parse_type_expression),
                    bws(parse_identifier),
                )),
                |(variable_type, name)| DeconstructionTarget::Declaration {
                    variable_type: Some(variable_type),
                    name,
                    is_var: false,
                },
            ),
            // Existing variable: existingVar
            map(parse_identifier, |name| DeconstructionTarget::Variable(name)),
        )),
    )(input)
} 