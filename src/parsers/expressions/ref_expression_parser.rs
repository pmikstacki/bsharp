use nom::{
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{map, not, peek},
    sequence::{terminated, tuple},
};

use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bs_context, bws, nom_to_bs};
use crate::parsers::expressions::expression_parser::parse_expression;

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

/// Parse a ref expression: ref expression
/// 
/// Examples:
/// ```csharp
/// ref field
/// ref array[index]
/// ref GetProperty()
/// ```
pub fn parse_ref_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "ref expression",
        map(
            tuple((
                bws(nom_to_bs(terminated(tag::<&str, &str, nom::error::Error<&str>>("ref"), word_boundary))),
                bws(parse_expression),
            )),
            |(_, expr)| Expression::Ref(Box::new(expr)),
        ),
    )(input)
} 