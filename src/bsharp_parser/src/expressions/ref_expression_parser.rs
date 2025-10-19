use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::keywords::parameter_modifier_keywords::kw_ref;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::Expression;

/// Parse a ref expression: ref expression
///
/// Examples:
/// ```csharp
/// ref field
/// ref array[index]
/// ref GetProperty()
/// ```
pub fn parse_ref_expression(input: Span) -> BResult<Expression> {
    map(
        (
            delimited(ws, kw_ref(), ws),
            delimited(ws, parse_expression, ws),
        ),
        |(_, expr)| Expression::Ref(Box::new(expr)),
    )
        .context("ref expression")
        .parse(input.into())
}
use crate::syntax::span::Span;
