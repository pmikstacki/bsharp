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
        map(
            tuple((bchar('>'), bchar('>'), bchar('>'), bchar('='))),
            |_| BinaryOperator::UnsignedRightShiftAssign,
        ),
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

        // If the left is a bitwise binary and op is the corresponding compound assignment,
        // restructure to preserve precedence: a & b &= c => a & (b &= c)
        let rebuilt = match (&left, &op) {
            (
                Expression::Binary {
                    left: l,
                    op: BinaryOperator::BitwiseAnd,
                    right: r,
                },
                BinaryOperator::AndAssign,
            ) => Expression::Binary {
                left: l.clone(),
                op: BinaryOperator::BitwiseAnd,
                right: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
                    target: r.clone(),
                    op,
                    value: Box::new(right),
                }))),
            },
            (
                Expression::Binary {
                    left: l,
                    op: BinaryOperator::BitwiseOr,
                    right: r,
                },
                BinaryOperator::OrAssign,
            ) => Expression::Binary {
                left: l.clone(),
                op: BinaryOperator::BitwiseOr,
                right: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
                    target: r.clone(),
                    op,
                    value: Box::new(right),
                }))),
            },
            (
                Expression::Binary {
                    left: l,
                    op: BinaryOperator::BitwiseXor,
                    right: r,
                },
                BinaryOperator::XorAssign,
            ) => Expression::Binary {
                left: l.clone(),
                op: BinaryOperator::BitwiseXor,
                right: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
                    target: r.clone(),
                    op,
                    value: Box::new(right),
                }))),
            },
            (
                Expression::Binary {
                    left: l,
                    op: BinaryOperator::LeftShift,
                    right: r,
                },
                BinaryOperator::LeftShiftAssign,
            ) => Expression::Binary {
                left: l.clone(),
                op: BinaryOperator::LeftShift,
                right: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
                    target: r.clone(),
                    op,
                    value: Box::new(right),
                }))),
            },
            (
                Expression::Binary {
                    left: l,
                    op: BinaryOperator::RightShift,
                    right: r,
                },
                BinaryOperator::RightShiftAssign,
            ) => Expression::Binary {
                left: l.clone(),
                op: BinaryOperator::RightShift,
                right: Box::new(Expression::Assignment(Box::new(AssignmentExpression {
                    target: r.clone(),
                    op,
                    value: Box::new(right),
                }))),
            },
            _ => Expression::Assignment(Box::new(AssignmentExpression {
                target: Box::new(left),
                op,
                value: Box::new(right),
            })),
        };

        Ok((input, rebuilt))
    } else {
        Ok((input, left))
    }
}
