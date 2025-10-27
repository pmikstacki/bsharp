use crate::parser::keywords::expression_keywords::kw_sizeof;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, SizeofExpression};

/// Parse a sizeof expression: `sizeof(Type)`
pub fn parse_sizeof_expression(input: Span) -> BResult<Expression> {
    map(
        preceded(
            kw_sizeof(),
            delimited(
                delimited(ws, tok_l_paren(), ws),
                delimited(ws, parse_type_expression, ws),
                cut(delimited(ws, tok_r_paren(), ws)),
            ),
        ),
        |target_type| Expression::Sizeof(Box::new(SizeofExpression { target_type })),
    )
    .context("sizeof expression")
    .parse(input)
}
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
