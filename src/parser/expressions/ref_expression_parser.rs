use nom::{combinator::map, sequence::tuple};

use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bws, context, keyword};

/// Parse a ref expression: ref expression
///
/// Examples:
/// ```csharp
/// ref field
/// ref array[index]
/// ref GetProperty()
/// ```
pub fn parse_ref_expression(input: &str) -> BResult<&str, Expression> {
    context(
        "ref expression",
        map(
            tuple((bws(keyword("ref")), bws(parse_expression))),
            |(_, expr)| Expression::Ref(Box::new(expr)),
        ),
    )(input)
}
