use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::throw_expression::ThrowExpression;
use crate::parser::parser_helpers::{bs_context, keyword};
use crate::parsers::expressions::expression_parser::parse_expression;

use nom::{
    combinator::{map, opt},
    sequence::preceded,
};

/// Parse a throw expression: `throw expr` or just `throw`
pub fn parse_throw_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "throw expression",
        map(
            preceded(
                keyword("throw"),
                opt(preceded(
                    nom::character::complete::multispace1,
                    parse_expression,
                )),
            ),
            |expr_opt| {
                Expression::Throw(Box::new(ThrowExpression {
                    expr: expr_opt.map(Box::new),
                }))
            },
        ),
    )(input)
} 