use crate::parser::keywords::expression_keywords::kw_typeof;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;

use nom::combinator::cut;
use nom::character::complete::char as nom_char;
use nom::{
    combinator::map,
    sequence::{delimited, preceded},
    Parser,
};
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, TypeofExpression};

/// Parse a typeof expression: `typeof(Type)`
pub fn parse_typeof_expression<'a>(input: Span<'a>) -> BResult<'a, Expression> {
    map(
        preceded(
            kw_typeof(),
            delimited(
                delimited(ws, nom_char('('), ws),
                delimited(ws, parse_type_expression, ws),
                cut(delimited(ws, nom_char(')'), ws)),
            ),
        ),
        |target_type| Expression::Typeof(Box::new(TypeofExpression { target_type })),
    )
    .context("typeof expression")
    .parse(input)
}
