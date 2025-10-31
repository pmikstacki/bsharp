use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::keywords::expression_keywords::kw_await;
use crate::errors::BResult;
use nom::combinator::{cut, map};
use nom::sequence::preceded;
use syntax::expressions::{AwaitExpression, Expression};

use crate::trivia::comment_parser::ws;
use nom::Parser;
use nom::sequence::delimited;
use nom_supreme::ParserExt;

use syntax::span::Span;


/// Enhanced await expression syntax using robust Nom combinators
/// Handles complex patterns like: await _userRepository.GetByEmailAsync(email)
pub fn parse_await_expression(input: Span) -> BResult<Expression> {
    map(
        preceded(
            kw_await(),
            cut(delimited(ws, parse_awaitable_expression, ws)),
        ),
        |expr| {
            Expression::Await(Box::new(AwaitExpression {
                expr: Box::new(expr),
            }))
        },
    )
    .context("await expression")
    .parse(input)
}

/// Parse various types of awaitable expressions with fallback
fn parse_awaitable_expression(input: Span) -> BResult<Expression> {
    if let Ok(r) = parse_complex_method_chain(input) {
        return Ok(r);
    }
    parse_simple_awaitable(input)
}

/// Parse complex method chains like _userRepository.GetByEmailAsync(email)
fn parse_complex_method_chain(input: Span) -> BResult<Expression> {
    // Import the main expression syntax to handle the full complexity
    match parse_expression_spanned.parse(input) {
        Ok((rest, s)) => Ok((rest, s.node)),
        Err(e) => Err(e),
    }
}

/// Parse simple awaitable expressions as fallback
fn parse_simple_awaitable(input: Span) -> BResult<Expression> {
    // Parse identifier or simple expressions
    use crate::parser::identifier_parser::parse_identifier;
    map(parse_identifier, Expression::Variable).parse(input)
}
