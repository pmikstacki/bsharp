use crate::parser::keywords::expression_keywords::kw_default;
use crate::parser::types::type_parser::parse_type_expression;
use crate::errors::BResult;

use crate::trivia::comment_parser::ws;

use nom::Parser;
use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::expressions::{DefaultExpression, Expression};

/// Parse a default expression: `default(Type)` or `default`
pub fn parse_default_expression(input: Span) -> BResult<Expression> {
    alt((
        // default(Type) - explicit type
        map(
            preceded(
                kw_default(),
                delimited(
                    delimited(ws, tok_l_paren(), ws),
                    delimited(ws, parse_type_expression, ws),
                    cut(delimited(ws, tok_r_paren(), ws)),
                ),
            ),
            |target_type| {
                Expression::Default(Box::new(DefaultExpression {
                    target_type: Some(target_type),
                }))
            },
        ),
        // default - literal without type
        map(kw_default(), |_| {
            Expression::Default(Box::new(DefaultExpression { target_type: None }))
        }),
    ))
    .context("default expression")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
