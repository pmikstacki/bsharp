use crate::parser::expressions::conditional_expression_parser;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use syntax::expressions::{AssignmentExpression, BinaryOperator, Expression};
use crate::syntax::span::Span;
use nom::character::complete::char as nom_char;
use nom::Parser;

pub(crate) fn parse_assignment_expression_or_higher(input: Span) -> BResult<Expression> {
    // Try to parse a conditional expression first
    let (input, left) =
        conditional_expression_parser::parse_conditional_expression_or_higher(input)?;

    // Check for assignment operators - order matters, longer operators first
    let (input, assignment_op) = opt(
        nom::sequence::delimited(ws, alt((
        // Multi-character assignment operators first
        map(tuple((nom_char('>'), nom_char('>'), nom_char('>'), nom_char('='))), |_| BinaryOperator::UnsignedRightShiftAssign),
        map(tuple((nom_char('?'), nom_char('?'), nom_char('='))), |_| {
            BinaryOperator::NullCoalescingAssign
        }),
        map(tuple((nom_char('<'), nom_char('<'), nom_char('='))), |_| {
            BinaryOperator::LeftShiftAssign
        }),
        map(tuple((nom_char('>'), nom_char('>'), nom_char('='))), |_| {
            BinaryOperator::RightShiftAssign
        }),
        map(tuple((nom_char('+'), nom_char('='))), |_| {
            BinaryOperator::AddAssign
        }),
        map(tuple((nom_char('-'), nom_char('='))), |_| {
            BinaryOperator::SubtractAssign
        }),
        map(tuple((nom_char('*'), nom_char('='))), |_| {
            BinaryOperator::MultiplyAssign
        }),
        map(tuple((nom_char('/'), nom_char('='))), |_| {
            BinaryOperator::DivideAssign
        }),
        map(tuple((nom_char('%'), nom_char('='))), |_| {
            BinaryOperator::ModuloAssign
        }),
        map(tuple((nom_char('&'), nom_char('='))), |_| {
            BinaryOperator::AndAssign
        }),
        map(tuple((nom_char('|'), nom_char('='))), |_| {
            BinaryOperator::OrAssign
        }),
        map(tuple((nom_char('^'), nom_char('='))), |_| {
            BinaryOperator::XorAssign
        }),
        // Simple assignment last
        map(nom_char('='), |_| BinaryOperator::Assign),
    )), ws))
    .parse(input)?;

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
