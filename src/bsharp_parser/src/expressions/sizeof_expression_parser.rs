use crate::parser::keywords::expression_keywords::kw_sizeof;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, SizeofExpression};

/// Parse a sizeof expression: `sizeof(Type)`
pub fn parse_sizeof_expression<'a>(input: Span<'a>) -> BResult<'a, Expression> {
    map(
        preceded(
            kw_sizeof(),
            delimited(
                delimited(ws, nom_char('('), ws),
                delimited(ws, parse_type_expression, ws),
                cut(delimited(ws, nom_char(')'), ws)),
            ),
        ),
        |target_type| Expression::Sizeof(Box::new(SizeofExpression { target_type })),
    )
    .context("sizeof expression")
    .parse(input)
}
use crate::syntax::span::Span;
