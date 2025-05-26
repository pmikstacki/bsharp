use nom::combinator::map;
use nom::sequence::preceded;

use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::await_expression::AwaitExpression;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bs_context, bws, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;

/// Parse an await expression: await expression
/// This is for handling await as a standalone expression, not just as a unary operator
pub fn parse_await_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "await expression",
        map(
            preceded(
                keyword("await"),
                bws(parse_expression),
            ),
            |expr| Expression::Await(Box::new(AwaitExpression {
                expr: Box::new(expr),
            })),
        ),
    )(input)
} 