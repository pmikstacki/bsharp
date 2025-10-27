use crate::parser::keywords::expression_keywords::kw_typeof;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use syntax::span::Span;


use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use nom::combinator::cut;
use nom::{
    Parser,
    combinator::map,
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, TypeofExpression};

/// Parse a typeof expression: `typeof(Type)`
pub fn parse_typeof_expression(input: Span) -> BResult<Expression> {
    map(
        preceded(
            kw_typeof(),
            delimited(
                delimited(ws, tok_l_paren(), ws),
                delimited(ws, parse_type_expression, ws),
                cut(delimited(ws, tok_r_paren(), ws)),
            ),
        ),
        |target_type| Expression::Typeof(Box::new(TypeofExpression { target_type })),
    )
    .context("typeof expression")
    .parse(input)
}
