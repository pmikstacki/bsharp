use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::sizeof_expression::SizeofExpression;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};

use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};

/// Parse a sizeof expression: `sizeof(Type)`
pub fn parse_sizeof_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "sizeof expression",
        map(
            preceded(
                keyword("sizeof"),
                delimited(
                    bws(bchar('(')),
                    bws(parse_type_expression),
                    cut(bws(bchar(')'))),
                ),
            ),
            |target_type| Expression::Sizeof(Box::new(SizeofExpression { target_type })),
        ),
    )(input)
}
