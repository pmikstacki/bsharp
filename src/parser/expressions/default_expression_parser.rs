use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::default_expression::DefaultExpression;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::cut;
use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
};

/// Parse a default expression: `default(Type)` or `default`
pub fn parse_default_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "default expression",
        alt((
            // default(Type) - explicit type
            map(
                preceded(
                    keyword("default"),
                    delimited(
                        bws(bchar('(')),
                        bws(parse_type_expression),
                        cut(bws(bchar(')'))),
                    ),
                ),
                |target_type| {
                    Expression::Default(Box::new(DefaultExpression {
                        target_type: Some(target_type),
                    }))
                },
            ),
            // default - literal without type
            map(keyword("default"), |_| {
                Expression::Default(Box::new(DefaultExpression { target_type: None }))
            }),
        )),
    )(input)
}
