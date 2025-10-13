use crate::parser::keywords::expression_keywords::kw_default;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::{DefaultExpression, Expression};

/// Parse a default expression: `default(Type)` or `default`
pub fn parse_default_expression<'a>(input: Span<'a>) -> BResult<'a, Expression> {
    alt((
        // default(Type) - explicit type
        map(
            preceded(
                kw_default(),
                delimited(
                    delimited(ws, nom_char('('), ws),
                    delimited(ws, parse_type_expression, ws),
                    cut(delimited(ws, nom_char(')'), ws)),
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
use crate::syntax::span::Span;
