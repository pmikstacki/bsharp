use crate::parser::keywords::expression_keywords::kw_typeof;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::typeof_expression::TypeofExpression;
use crate::syntax::parser_helpers::{bchar, bws, context};

use nom::combinator::cut;
use nom::{
    combinator::map,
    sequence::{delimited, preceded},
};

/// Parse a typeof expression: `typeof(Type)`
pub fn parse_typeof_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "typeof expression",
        map(
            preceded(
                kw_typeof(),
                delimited(
                    bws(bchar('(')),
                    bws(parse_type_expression),
                    cut(bws(bchar(')'))),
                ),
            ),
            |target_type| Expression::Typeof(Box::new(TypeofExpression { target_type })),
        ),
    )(input)
}
