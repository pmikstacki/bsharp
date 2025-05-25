use crate::parser::errors::BResult;
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::nodes::expressions::sizeof_expression::SizeofExpression;
use crate::parser::parser_helpers::{bchar, bs_context, bws, keyword};
use crate::parsers::types::type_parser::parse_type_expression;

use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};

/// Parse a sizeof expression: `sizeof(Type)`
pub fn parse_sizeof_expression(input: &str) -> BResult<&str, Expression> {
    bs_context(
        "sizeof expression",
        map(
            preceded(
                keyword("sizeof"),
                delimited(
                    bws(bchar('(')),
                    bws(parse_type_expression),
                    bws(bchar(')')),
                ),
            ),
            |target_type| {
                Expression::Sizeof(Box::new(SizeofExpression {
                    target_type,
                }))
            },
        ),
    )(input)
} 