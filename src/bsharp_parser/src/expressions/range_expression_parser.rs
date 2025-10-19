use crate::parser::expressions::unary_expression_parser::parse_unary_expression_or_higher;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use nom::character::complete::char as nom_char;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::expressions::{Expression, RangeExpression};

/// Helper for parse_range_expression_or_higher to specifically parse ranges starting with `..`
fn parse_range_starting_with_dots(input: Span) -> BResult<Expression> {
    let (input, _) = delimited(ws, (nom_char('.'), nom_char('.')), ws).parse(input.into())?;
    let (input, end_operand) = opt(delimited(ws, parse_unary_expression_or_higher, ws)).parse(input.into())?;
    Ok((input, Expression::Range(Box::new(RangeExpression { start: None, end: end_operand.map(Box::new), is_inclusive: false }))))
}

/// This is the main function that should be in the precedence chain.
/// It will first try to parse a range that starts with `..`
/// If that fails, it will try to parse a unary expression, and then check if it's followed by `..`
pub(crate) fn parse_range_expression_or_higher(input: Span) -> BResult<Expression> {
    nom::branch::alt((
        parse_range_starting_with_dots.context("range starting with .."),
        // Then, attempt to parse expressions like `expr..`, `expr..expr` or just `expr`
        (|i| {
            let (i, start_expr) = parse_unary_expression_or_higher(i)?;
            if let Ok((i_after_dots, _)) = delimited(ws, (nom_char('.'), nom_char('.')), ws).parse(i) {
                let (i_after_end, end_expr) = opt(delimited(ws, parse_unary_expression_or_higher, ws)).parse(i_after_dots)?;
                Ok((i_after_end, Expression::Range(Box::new(RangeExpression { start: Some(Box::new(start_expr)), end: end_expr.map(Box::new), is_inclusive: false }))))
            } else {
                Ok((i, start_expr))
            }
        }).context("range starting with operand or just operand"),
    ))
        .parse(input.into())
}
