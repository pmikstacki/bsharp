use crate::parser::expressions::logical_expression_parser;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{BinaryOperator, Expression};
use crate::syntax::parser_helpers::{bchar, bws};
use nom::sequence::tuple;

pub(crate) fn parse_null_coalescing_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (mut input, mut left) =
        logical_expression_parser::parse_logical_or_expression_or_higher(input)?;

    // Handle ?? (null coalescing) - right associative, but avoid consuming if followed by =
    while let Ok((new_input, _)) = bws(tuple((
        bchar('?'),
        bchar('?'),
        nom::combinator::not(bchar('=')),
    )))(input)
    {
        let (new_input, right) = parse_null_coalescing_expression_or_higher(new_input)?;
        left = Expression::Binary {
            left: Box::new(left),
            op: BinaryOperator::NullCoalescing,
            right: Box::new(right),
        };
        input = new_input;
    }

    Ok((input, left))
}
