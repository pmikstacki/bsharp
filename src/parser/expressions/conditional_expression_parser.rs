use crate::parser::expressions;
use crate::parser::expressions::null_coalescing_expression_parser;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{ConditionalExpression, Expression};
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::branch::alt;

pub(crate) fn parse_conditional_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Parse the condition (left side)
    let (input, condition) =
        null_coalescing_expression_parser::parse_null_coalescing_expression_or_higher(input)?;

    // Check for ternary operator: condition ? true_expr : false_expr
    // We need to be careful not to consume ?. or ?[ (null-conditional operators)
    // or ?? (null-coalescing operators)

    // First, try to parse whitespace and then ?
    let (after_ws, _) = ws(input)?;

    // Check if we have a ? that's not followed by . or [ or ?
    if let Ok((after_q, _)) = bchar('?')(after_ws) {
        // Use peek to check what comes next without consuming
        if nom::combinator::peek(nom::combinator::not(alt((
            bchar('.'),
            bchar('['),
            bchar('?'),
        ))))(after_q)
        .is_ok()
        {
            // It's a ternary operator, not null-conditional or null-coalescing
            let (input, _) = bws(bchar('?'))(input)?;
            let (input, true_expr) = context(
                "conditional expression: true branch",
                bws(expressions::parse_expression),
            )(input)?;
            let (input, _) = nom::combinator::cut(bws(bchar(':')))(input)?;
            let (input, false_expr) = bws(parse_conditional_expression_or_higher)(input)?;

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
