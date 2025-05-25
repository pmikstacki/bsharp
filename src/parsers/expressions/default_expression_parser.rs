use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::default_expression::DefaultExpression;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    branch::alt,
    combinator::{map, opt},
    sequence::{delimited, preceded},
};

/// Parse a default expression: `default(Type)` or `default`
pub fn parse_default_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "default expression",
        alt((
            // default(Type) - explicit type
            map(
                preceded(
                    keyword("default"),
                    delimited(
                        bws(bchar('(')),
                        bws(parse_type_expression),
                        bws(bchar(')')),
                    ),
                ),
                |target_type| {
                    Expression::Default(Box::new(DefaultExpression {
                        target_type: Some(target_type),
                    }))
                },
            ),
            // default - literal without type
            map(
                keyword("default"),
                |_| {
                    Expression::Default(Box::new(DefaultExpression {
                        target_type: None,
                    }))
                },
            ),
        )),
    )(input)
} 