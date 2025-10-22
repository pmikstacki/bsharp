use crate::parser::expressions;
use crate::parser::expressions::null_coalescing_expression_parser;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use crate::syntax::span::Span;
use nom::Parser;
use nom::branch::alt;
use nom::character::complete::char as nom_char;
use nom_supreme::ParserExt;
use syntax::expressions::{ConditionalExpression, Expression};

pub(crate) fn parse_conditional_expression_or_higher<'a>(
    input: Span<'a>,
) -> BResult<'a, Expression> {
    // Parse the condition (left side)
    let (input, condition) =
        null_coalescing_expression_parser::parse_null_coalescing_expression_or_higher(input)?;

    // Check for ternary operator: condition ? true_expr : false_expr
    // We need to be careful not to consume ?. or ?[ (null-conditional operators)
    // or ?? (null-coalescing operators)

    // First, try to parse whitespace and then ?
    let (after_ws, _) = ws(input)?;

    // Check if we have a ? that's not followed by . or [ or ?
    if let Ok((after_q, _)) =
        nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>('?').parse(after_ws)
    {
        if nom::combinator::peek(nom::combinator::not(alt((
            nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>('.'),
            nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>('['),
            nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>('?'),
        ))))
        .parse(after_q)
        .is_ok()
        {
            // It's a ternary operator, not null-conditional or null-coalescing
            let (input, _) = nom::sequence::delimited(
                ws,
                nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>('?'),
                ws,
            )
            .parse(input)?;
            let (input, true_expr) = (|i| expressions::parse_expression(i))
                .context("conditional expression: true branch")
                .parse(input)?;
            let (input, _) = nom::combinator::cut(nom::sequence::delimited(
                ws,
                nom_char::<Span<'a>, nom_supreme::error::ErrorTree<Span<'a>>>(':'),
                ws,
            ))
            .parse(input)?;
            let (input, false_expr) =
                nom::sequence::delimited(ws, parse_conditional_expression_or_higher, ws)
                    .parse(input)?;

            return Ok((
                input,
                Expression::Conditional(Box::new(ConditionalExpression {
                    condition: Box::new(condition),
                    consequence: Box::new(true_expr),
                    alternative: Box::new(false_expr),
                })),
            ));
        }
    }

    // Not a ternary operator, just return the condition
    Ok((input, condition))
}
