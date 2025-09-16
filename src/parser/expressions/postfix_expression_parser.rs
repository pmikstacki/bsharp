use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::nodes::expressions::indexing_expression::IndexingExpression;
use crate::syntax::nodes::expressions::invocation_expression::InvocationExpression;
use crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::syntax::nodes::expressions::null_conditional_expression::NullConditionalExpression;
use crate::syntax::nodes::expressions::UnaryOperator;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws};
use crate::parser::expressions::expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;

use nom::{
    branch::alt,
    combinator::{cut, map},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, Clone)]
enum PostfixOpKind { 
    Invocation(Vec<Expression>), 
    MemberAccess(Identifier),
    NullConditionalMemberAccess(Identifier),
    Indexing(Box<Expression>), 
    NullConditionalIndexing(Box<Expression>),
    PostfixIncrement,
    PostfixDecrement,
    NullForgiving,
}

/// Enhanced method invocation parsing
fn enhanced_method_invocation(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        delimited(
            bws(bchar('(')),
            separated_list0(bws(bchar(',')), bws(parse_expression)),
            cut(bws(bchar(')')))
        ),
        PostfixOpKind::Invocation
    )(input)
}

/// Enhanced member access parsing
fn enhanced_member_access(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        preceded(
            tuple((bws(bchar('.')), nom::combinator::not(bchar('.')))),
            cut(bws(parse_identifier))
        ),
        PostfixOpKind::MemberAccess
    )(input)
}

/// Enhanced indexing parsing
fn enhanced_indexing(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        delimited(
            bws(bchar('[')),
            cut(bws(parse_expression)),
            cut(bws(bchar(']')))
        ),
        |expr| PostfixOpKind::Indexing(Box::new(expr))
    )(input)
}

/// Enhanced null conditional access parsing
fn enhanced_null_conditional_access(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        // ?. member access
        map(
            preceded(
                bws(tuple((bchar('?'), bchar('.')))),
                cut(bws(parse_identifier))
            ),
            PostfixOpKind::NullConditionalMemberAccess
        ),
        // ?[ indexing
        map(
            delimited(
                bws(tuple((bchar('?'), bchar('[')))),
                cut(bws(parse_expression)),
                cut(bws(bchar(']')))
            ),
            |expr| PostfixOpKind::NullConditionalIndexing(Box::new(expr))
        ),
    ))(input)
}

/// Simple postfix operations as fallback
fn simple_postfix_operations(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        map(bws(tuple((bchar('+'), bchar('+')))), |_| PostfixOpKind::PostfixIncrement),
        map(bws(tuple((bchar('-'), bchar('-')))), |_| PostfixOpKind::PostfixDecrement),
        map(bws(bchar('!')), |_| PostfixOpKind::NullForgiving),
    ))(input)
}

/// Enhanced postfix operation syntax with better error recovery
fn enhanced_postfix_operation(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        enhanced_member_access,
        enhanced_method_invocation,
        enhanced_indexing,
        enhanced_null_conditional_access,
        simple_postfix_operations,
    ))(input)
}

/// Apply a postfix operation to an expression
fn apply_postfix_operation(expr: Expression, op: PostfixOpKind) -> Expression {
    match op {
        PostfixOpKind::MemberAccess(member) => Expression::MemberAccess(Box::new(MemberAccessExpression {
            object: Box::new(expr),
            member,
        })),
        PostfixOpKind::NullConditionalMemberAccess(member) => Expression::NullConditional(Box::new(NullConditionalExpression {
            target: Box::new(expr),
            member,
            is_element_access: false,
            argument: None,
        })),
        PostfixOpKind::Invocation(args) => Expression::Invocation(Box::new(InvocationExpression {
            callee: Box::new(expr),
            arguments: args,
        })),
        PostfixOpKind::Indexing(index) => Expression::Indexing(Box::new(IndexingExpression {
            target: Box::new(expr),
            index,
        })),
        PostfixOpKind::NullConditionalIndexing(index) => Expression::NullConditional(Box::new(NullConditionalExpression {
            target: Box::new(expr),
            member: Identifier { name: String::new() },
            is_element_access: true,
            argument: Some(index),
        })),
        PostfixOpKind::PostfixIncrement => Expression::PostfixUnary {
            op: UnaryOperator::Increment,
            expr: Box::new(expr),
        },
        PostfixOpKind::PostfixDecrement => Expression::PostfixUnary {
            op: UnaryOperator::Decrement,
            expr: Box::new(expr),
        },
        PostfixOpKind::NullForgiving => Expression::PostfixUnary {
            op: UnaryOperator::NullForgiving,
            expr: Box::new(expr),
        },
    }
}

/// Enhanced postfix expression syntax using many0 for robust method chaining
/// Handles complex patterns like _userRepository.GetByEmailAsync(email).Result
pub(crate) fn parse_postfix_expression_or_higher(input: &str) -> BResult<&str, Expression> { 
    let (input, mut expr) = crate::parser::expressions::expression_parser::parse_primary_expression(input)?;
    let (input, postfix_ops) = nom::multi::many0(enhanced_postfix_operation)(input)?;
    for op in postfix_ops {
        expr = apply_postfix_operation(expr, op);
    }
    Ok((input, expr))
}
