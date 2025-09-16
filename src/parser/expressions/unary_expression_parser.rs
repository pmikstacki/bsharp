use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::range_expression::IndexExpression;
use crate::syntax::nodes::expressions::UnaryOperator;
use crate::syntax::parser_helpers::{bchar, bws};
use crate::parser::expressions::await_expression_parser::parse_await_expression;
use crate::parser::expressions::ref_expression_parser::parse_ref_expression;
use crate::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parser::expressions::sizeof_expression_parser::parse_sizeof_expression;
use crate::parser::expressions::typeof_expression_parser::parse_typeof_expression;
use crate::parser::types::type_parser::parse_type_expression;

use nom::{
    branch::alt,
    combinator::{map, recognize},
    sequence::pair,
};

/// Parse a unary expression or higher precedence constructs
pub(crate) fn parse_unary_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Try ref expression first
    if let Ok((input, ref_expr)) = parse_ref_expression(input) {
        return Ok((input, ref_expr));
    }

    // Try enhanced await expression first (handles complex patterns)
    if let Ok((input, await_expr)) = parse_await_expression(input) {
        return Ok((input, await_expr));
    }

    // Try prefix unary operators
    if let Ok((input, op)) = bws(alt((
        map(bchar('+'), |_| UnaryOperator::Plus),
        map(bchar('-'), |_| UnaryOperator::Minus),
        map(bchar('!'), |_| UnaryOperator::LogicalNot),
        map(bchar('~'), |_| UnaryOperator::BitwiseNot),
        map(recognize(pair(bchar('+'), bchar('+'))), |_| UnaryOperator::Increment),
        map(recognize(pair(bchar('-'), bchar('-'))), |_| UnaryOperator::Decrement),
        map(bchar('&'), |_| UnaryOperator::AddressOf),
        map(bchar('*'), |_| UnaryOperator::PointerIndirection),
        // ^ (index from end) operator as unary
        map(bchar('^'), |_| UnaryOperator::IndexFromEnd),
    )))(input) {
        let (input, operand) = parse_unary_expression_or_higher(input)?;
        // If the operator is IndexFromEnd, wrap it in Expression::Index
        if op == UnaryOperator::IndexFromEnd {
            return Ok((input, Expression::Index(Box::new(IndexExpression {
                value: Box::new(operand),
            }))));
        }
        return Ok((input, Expression::Unary {
            op,
            expr: Box::new(operand),
        }));
    }

    // Try cast expression: (Type)expression - but be more careful to avoid conflicts with parenthesized expressions
    if let Ok((input_after_paren, _)) = bws(bchar('('))(input) {
        // Try to parse as a type, but only if it's followed by something that looks like an expression
        if let Ok((input_after_type, _ty)) = parse_type_expression(input_after_paren) {
            if let Ok((input_after_close_paren, _)) = bws(bchar(')'))(input_after_type) {
                // Only treat as cast if there's actually something after the closing parenthesis
                // that could be an expression (not end of input)
                if !input_after_close_paren.trim().is_empty() {
                    let (input, operand) = parse_unary_expression_or_higher(input_after_close_paren)?;
                    return Ok((input, Expression::Unary {
                        op: UnaryOperator::Cast,
                        expr: Box::new(operand),
                    }));
                }
            }
        }
    }

    // Try stackalloc expression
    if let Ok((input, stackalloc_expr)) = parse_stackalloc_expression(input) {
        return Ok((input, stackalloc_expr));
    }

    // Try sizeof expression
    if let Ok((input, sizeof_expr)) = parse_sizeof_expression(input) {
        return Ok((input, sizeof_expr));
    }

    // Try typeof expression
    if let Ok((input, typeof_expr)) = parse_typeof_expression(input) {
        return Ok((input, typeof_expr));
    }

    // If none of the above work, try postfix expressions
    crate::parser::expressions::postfix_expression_parser::parse_postfix_expression_or_higher(input)
}
