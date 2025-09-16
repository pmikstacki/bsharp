use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::typeof_expression::TypeofExpression;
use crate::syntax::parser_helpers::{bchar, context, bws, keyword};
use crate::parser::types::type_parser::parse_type_expression;

use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};
use nom::combinator::cut;

/// Parse a typeof expression: `typeof(Type)`
pub fn parse_typeof_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "typeof expression",
        map(
            preceded(
                keyword("typeof"),
                delimited(
                    bws(bchar('(')),
                    bws(parse_type_expression),
                    cut(bws(bchar(')'))),
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