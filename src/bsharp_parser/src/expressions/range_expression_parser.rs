use crate::parser::expressions::unary_expression_parser::parse_unary_expression_or_higher;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::range_expression::RangeExpression;
use crate::syntax::parser_helpers::{bchar, bws, context};

use nom::{combinator::opt, sequence::pair};

/// Helper for parse_range_expression_or_higher to specifically parse ranges starting with `..`
fn parse_range_starting_with_dots(input: &str) -> BResult<&str, Expression> {
    let (input, _) = pair(bchar('.'), bchar('.'))(input)?;
    let (input, end_operand) = opt(bws(parse_unary_expression_or_higher))(input)?;
    Ok((
        input,
        Expression::Range(Box::new(RangeExpression {
            start: None,
            end: end_operand.map(Box::new),
            is_inclusive: false,
        })),
    ))
}

/// This is the main function that should be in the precedence chain.
/// It will first try to parse a range that starts with `..`
/// If that fails, it will try to parse a unary expression, and then check if it's followed by `..`
pub(crate) fn parse_range_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    nom::branch::alt((
        // Attempt to parse ranges like `..expr` or `..` first
        context("range starting with ..", parse_range_starting_with_dots),
        // Then, attempt to parse expressions like `expr..`, `expr..expr` or just `expr`
        context("range starting with operand or just operand", |i: &str| {
            let (i, start_expr) = parse_unary_expression_or_higher(i)?;
            if let Ok((i_after_dots, _)) = pair(bchar('.'), bchar('.'))(i) {
                let (i_after_end, end_expr) = opt(parse_unary_expression_or_higher)(i_after_dots)?;
                Ok((
                    i_after_end,
                    Expression::Range(Box::new(RangeExpression {
                        start: Some(Box::new(start_expr)),
                        end: end_expr.map(Box::new),
                        is_inclusive: false,
                    })),
                ))
            } else {
                Ok((i, start_expr))
            }
        }),
    ))(input)
}
