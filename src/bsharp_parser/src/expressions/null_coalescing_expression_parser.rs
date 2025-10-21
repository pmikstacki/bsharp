use crate::parser::expressions::logical_expression_parser;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use crate::syntax::span::Span;
use nom::character::complete::char as nom_char;
use nom::combinator::not;
use nom::Parser;
use syntax::expressions::{BinaryOperator, Expression};
use crate::tokens::assignment::tok_assign;

pub(crate) fn parse_null_coalescing_expression_or_higher(input: Span) -> BResult<Expression> {
    let (mut input, mut left) =
        logical_expression_parser::parse_logical_or_expression_or_higher(input)?;

    // Handle ?? (null coalescing) - right associative, but avoid consuming if followed by =
    while let Ok((new_input, _)) = nom::sequence::delimited(
        ws,
        (nom_char('?'), nom_char('?'), not(tok_assign())),
        ws,
    )
        .parse(input)
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
