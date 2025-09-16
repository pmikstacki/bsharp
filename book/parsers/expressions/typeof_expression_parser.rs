use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::typeof_expression::TypeofExpression;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};

/// Parse a typeof expression: `typeof(Type)`
pub fn parse_typeof_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "typeof expression",
        map(
            preceded(
                keyword("typeof"),
                delimited(
                    bws(bchar('(')),
                    bws(parse_type_expression),
                    bws(bchar(')')),
                ),
            ),
            |target_type| {
                Expression::Typeof(Box::new(TypeofExpression {
                    target_type,
                }))
            },
        ),
    )(input)
} 