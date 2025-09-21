use nom::branch::alt;
use nom::combinator::{cut, map};
use nom::sequence::preceded;

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::await_expression::AwaitExpression;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bws, context};
use crate::parser::keywords::expression_keywords::kw_await;

/// Enhanced await expression syntax using robust Nom combinators
/// Handles complex patterns like: await _userRepository.GetByEmailAsync(email)
pub fn parse_await_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "await expression",
        map(
            preceded(kw_await(), cut(bws(parse_awaitable_expression))),
            |expr| {
                Expression::Await(Box::new(AwaitExpression {
                    expr: Box::new(expr),
                }))
            },
        ),
    )(input)
}

/// Parse various types of awaitable expressions with fallback
fn parse_awaitable_expression(input: &str) -> BResult<&str, Expression> {
    alt((parse_complex_method_chain, parse_simple_awaitable))(input)
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
