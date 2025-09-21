use crate::parser::expressions::conditional_expression_parser;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{AssignmentExpression, BinaryOperator, Expression};
use crate::syntax::parser_helpers::{bchar, bws};
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::tuple;

pub(crate) fn parse_assignment_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Try to parse a conditional expression first
    let (input, left) =
        conditional_expression_parser::parse_conditional_expression_or_higher(input)?;

    // Check for assignment operators - order matters, longer operators first
    let (input, assignment_op) = opt(bws(alt((
        // Multi-character assignment operators first
        map(tuple((bchar('>'), bchar('>'), bchar('>'), bchar('='))), |_| {
            BinaryOperator::UnsignedRightShiftAssign
        }),
        map(tuple((bchar('?'), bchar('?'), bchar('='))), |_| {
            BinaryOperator::NullCoalescingAssign
        }),
        map(tuple((bchar('<'), bchar('<'), bchar('='))), |_| {
            BinaryOperator::LeftShiftAssign
        }),
        map(tuple((bchar('>'), bchar('>'), bchar('='))), |_| {
            BinaryOperator::RightShiftAssign
        }),
        map(tuple((bchar('+'), bchar('='))), |_| {
            BinaryOperator::AddAssign
        }),
        map(tuple((bchar('-'), bchar('='))), |_| {
            BinaryOperator::SubtractAssign
        }),
        map(tuple((bchar('*'), bchar('='))), |_| {
            BinaryOperator::MultiplyAssign
        }),
        map(tuple((bchar('/'), bchar('='))), |_| {
            BinaryOperator::DivideAssign
        }),
        map(tuple((bchar('%'), bchar('='))), |_| {
            BinaryOperator::ModuloAssign
        }),
        map(tuple((bchar('&'), bchar('='))), |_| {
            BinaryOperator::AndAssign
        }),
        map(tuple((bchar('|'), bchar('='))), |_| {
            BinaryOperator::OrAssign
        }),
        map(tuple((bchar('^'), bchar('='))), |_| {
            BinaryOperator::XorAssign
        }),
        // Simple assignment last
        map(bchar('='), |_| BinaryOperator::Assign),
    ))))(input)?;

    if let Some(op) = assignment_op {
        // Parse the right side of the assignment (right-associative)
        let (input, right) = parse_assignment_expression_or_higher(input)?;

        Ok((
            input,
            Expression::Assignment(Box::new(AssignmentExpression {
                target: Box::new(left),
                op,
                value: Box::new(right),
            })),
        ))
    } else {
        Ok((input, left))
    }
}
