use crate::parser::expressions::await_expression_parser::parse_await_expression;
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::expressions::ref_expression_parser::parse_ref_expression;
use crate::parser::expressions::sizeof_expression_parser::parse_sizeof_expression;
use crate::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use crate::parser::expressions::typeof_expression_parser::parse_typeof_expression;
use crate::parser::keywords::exception_and_safety_keywords::{kw_checked, kw_unchecked};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use crate::syntax::span::Span;
use nom::{
    branch::alt,
    character::complete::char as nom_char,
    combinator::{map, recognize},
    sequence::{delimited, pair},
    Parser,
};
use syntax::expressions::{Expression, IndexExpression, UnaryOperator, UncheckedExpression};
use crate::tokens::delimiters::{tok_l_paren, tok_r_paren};
use crate::tokens::nullish::tok_not;

/// Parse a unary expression or higher precedence constructs
pub(crate) fn parse_unary_expression_or_higher(input: Span) -> BResult<Expression> {
    // checked(expr)
    if let Ok((input_after_kw, _)) = delimited(ws, kw_checked(), ws).parse(input) {
        if let Ok((rest, _)) = delimited(ws, tok_l_paren(), ws).parse(input_after_kw) {
            let (rest, inner) = parse_expression(rest)?;
            let (rest, _) = delimited(ws, tok_r_paren(), ws).parse(rest)?;
            return Ok((
                rest,
                Expression::Checked(Box::new(
                    syntax::expressions::checked_expression::CheckedExpression {
                        expr: Box::new(inner),
                    },
                )),
            ));
        }
    }

    // unchecked(expr)
    if let Ok((input_after_kw, _)) = delimited(ws, kw_unchecked(), ws).parse(input) {
        if let Ok((rest, _)) = delimited(ws, tok_l_paren(), ws).parse(input_after_kw) {
            let (rest, inner) = parse_expression(rest)?;
            let (rest, _) = delimited(ws, tok_r_paren(), ws).parse(rest)?;
            return Ok((
                rest,
                Expression::Unchecked(Box::new(UncheckedExpression {
                    expr: Box::new(inner),
                })),
            ));
        }
    }
    // Try ref expression first
    if let Ok((input, ref_expr)) = parse_ref_expression(input) {
        return Ok((input, ref_expr));
    }

    // Try enhanced await expression first (handles complex patterns)
    if let Ok((input, await_expr)) = parse_await_expression(input) {
        return Ok((input, await_expr));
    }

    // Try prefix unary operators
    if let Ok((input, op)) = delimited(ws, alt((
        // Try multi-char operators first to avoid consuming a single '+' or '-' too early
        map(recognize(pair(nom_char('+'), nom_char('+'))), |_| UnaryOperator::Increment),
        map(recognize(pair(nom_char('-'), nom_char('-'))), |_| UnaryOperator::Decrement),
        map(nom_char('+'), |_| UnaryOperator::Plus),
        map(nom_char('-'), |_| UnaryOperator::Minus),
        map(tok_not(), |_| UnaryOperator::LogicalNot),
        map(nom_char('~'), |_| UnaryOperator::BitwiseNot),
        map(nom_char('&'), |_| UnaryOperator::AddressOf),
        map(nom_char('*'), |_| UnaryOperator::PointerIndirection),
        // ^ (index from end) operator as unary
        map(nom_char('^'), |_| UnaryOperator::IndexFromEnd),
    )), ws)
        .parse(input)
    {
        let (input, operand) = parse_unary_expression_or_higher(input)?;
        // If the operator is IndexFromEnd, wrap it in Expression::Index
        if op == UnaryOperator::IndexFromEnd {
            return Ok((
                input,
                Expression::Index(Box::new(IndexExpression {
                    value: Box::new(operand),
                })),
            ));
        }
        return Ok((
            input,
            Expression::Unary {
                op,
                expr: Box::new(operand),
            },
        ));
    }

    // Try cast expression: (Type)expression - but be more careful to avoid conflicts with parenthesized expressions
    if let Ok((input_after_paren, _)) = delimited(ws, tok_l_paren(), ws).parse(input) {
        // Try to parse as a type, but only if it's followed by something that looks like an expression
        if let Ok((input_after_type, ty)) = parse_type_expression(input_after_paren) {
            if let Ok((input_after_close_paren, _)) = delimited(ws, tok_r_paren(), ws).parse(input_after_type) {
                // Only treat as cast if the operand parses successfully; otherwise, backtrack.
                if let Ok((input, operand)) =
                    parse_unary_expression_or_higher(input_after_close_paren)
                {
                    return Ok((
                        input,
                        Expression::Cast {
                            expression: Box::new(operand),
                            target_type: ty,
                        },
                    ));
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
