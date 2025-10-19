use crate::parser::expressions::conditional_expression_parser;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use crate::syntax::span::Span;
use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::Parser;
// tuples implement Parser directly in nom 8
use syntax::expressions::{AssignmentExpression, BinaryOperator, Expression};
use crate::tokens::assignment::{tok_add_assign, tok_and_assign, tok_assign, tok_div_assign, tok_mod_assign, tok_mul_assign, tok_null_coalescing_assign, tok_or_assign, tok_shl_assign, tok_shr_assign, tok_sub_assign, tok_ushr_assign, tok_xor_assign};

pub(crate) fn parse_assignment_expression_or_higher(input: Span) -> BResult<Expression> {
    // Try to parse a conditional expression first
    let (input, left) =
        conditional_expression_parser::parse_conditional_expression_or_higher(input.into())?;

    // Check for assignment operators - order matters, longer operators first
    let (input, assignment_op) = opt(
        nom::sequence::delimited(ws, alt((
            // Multi-character assignment operators first
            map(tok_ushr_assign(), |_| BinaryOperator::UnsignedRightShiftAssign),
            map(tok_null_coalescing_assign(), |_| {
                BinaryOperator::NullCoalescingAssign
            }),
            map(tok_shl_assign(), |_| {
                BinaryOperator::LeftShiftAssign
            }),
            map(tok_shr_assign(), |_| {
                BinaryOperator::RightShiftAssign
            }),
            map(tok_add_assign(), |_| {
                BinaryOperator::AddAssign
            }),
            map(tok_sub_assign(), |_| {
                BinaryOperator::SubtractAssign
            }),
            map(tok_mul_assign(), |_| {
                BinaryOperator::MultiplyAssign
            }),
            map(tok_div_assign(), |_| {
                BinaryOperator::DivideAssign
            }),
            map(tok_mod_assign(), |_| {
                BinaryOperator::ModuloAssign
            }),
            map(tok_and_assign(), |_| {
                BinaryOperator::AndAssign
            }),
            map(tok_or_assign(), |_| {
                BinaryOperator::OrAssign
            }),
            map(tok_xor_assign(), |_| {
                BinaryOperator::XorAssign
            }),
            // Simple assignment last
            map(tok_assign(), |_| BinaryOperator::Assign),
        )), ws))
        .parse(input.into())?;

    if let Some(op) = assignment_op {
        // Parse the right side of the assignment (right-associative)
        let (input, right) = parse_assignment_expression_or_higher(input.into())?;

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
