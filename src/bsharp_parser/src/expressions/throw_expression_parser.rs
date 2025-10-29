use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::keywords::exception_and_safety_keywords::kw_throw;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;

use nom::{
    Parser,
    combinator::{map, opt},
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, ThrowExpression};

/// Parse a throw expression: `throw expr` or just `throw`
pub fn parse_throw_expression(input: Span) -> BResult<Expression> {
    map(
        preceded(kw_throw(), opt(delimited(ws, parse_expression_spanned, ws).map(|s| s.node))),
        |expr_opt| {
            Expression::Throw(Box::new(ThrowExpression {
                expr: expr_opt.map(Box::new),
            }))
        },
    )
    .context("throw expression")
    .parse(input)
}
use syntax::span::Span;

