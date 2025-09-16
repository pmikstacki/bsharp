use nom::combinator::{map, cut};
use nom::sequence::preceded;
use nom::branch::alt;

use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::await_expression::AwaitExpression;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{context, bws, keyword};
use crate::parser::expressions::expression_parser::parse_expression;

/// Enhanced await expression syntax using robust Nom combinators
/// Handles complex patterns like: await _userRepository.GetByEmailAsync(email)
pub fn parse_await_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "await expression",
        map(
            preceded(
                keyword("await"),
                cut(bws(parse_awaitable_expression)),
            ),
            |expr| Expression::Await(Box::new(AwaitExpression {
                expr: Box::new(expr),
            })),
        ),
    )(input)
}

/// Parse various types of awaitable expressions with fallback
fn parse_awaitable_expression(input: &str) -> BResult<&str, Expression> {
    alt((
        parse_complex_method_chain,
        parse_simple_awaitable,
    ))(input)
}

/// Parse complex method chains like _userRepository.GetByEmailAsync(email)
fn parse_complex_method_chain(input: &str) -> BResult<&str, Expression> {
    // Import the main expression syntax to handle the full complexity
    parse_expression(input)
}

/// Parse simple awaitable expressions as fallback
fn parse_simple_awaitable(input: &str) -> BResult<&str, Expression> {
    // Parse identifier or simple expressions
    use crate::parser::identifier_parser::parse_identifier;
    map(parse_identifier, |id| Expression::Variable(id))(input)
} 