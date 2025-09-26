use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::{Expression, WithInitializerEntry};
use crate::syntax::nodes::expressions::indexing_expression::IndexingExpression;
use crate::syntax::nodes::expressions::invocation_expression::{InvocationExpression, Argument, ArgumentModifier};
use crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression;
use crate::syntax::nodes::expressions::null_conditional_expression::NullConditionalExpression;
use crate::syntax::nodes::expressions::UnaryOperator;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, bws, parse_delimited_list0};
use crate::parser::keywords::expression_keywords::kw_with;

use nom::{
    branch::alt,
    combinator::{cut, map},
    sequence::{delimited, preceded, tuple},
};

#[derive(Debug, Clone)]
enum PostfixOpKind {
    Invocation(Vec<Argument>),
    MemberAccess(Identifier),
    NullConditionalMemberAccess(Identifier),
    Indexing(Box<Expression>),
    NullConditionalIndexing(Box<Expression>),
    PostfixIncrement,
    PostfixDecrement,
    NullForgiving,
    With(Vec<WithInitializerEntry>),
}

/// Parse a single invocation argument supporting optional modifier and name:
/// [ref|out|in] [name: ] expr
fn parse_invocation_argument(input: &str) -> BResult<&str, Argument> {
    use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_ref};
    use nom::branch::alt;
    use nom::combinator::map;
    use nom::combinator::opt;

    // Optional modifier
    let (input, modifier) = bws(opt(alt((
        map(kw_ref(), |_| ArgumentModifier::Ref),
        map(kw_out(), |_| ArgumentModifier::Out),
        map(kw_in(), |_| ArgumentModifier::In),
    ))))(input)?;

    // Optional name label: identifier:
    let (input, name) = if let Ok((i2, (id, _))) = bws(nom::sequence::tuple((parse_identifier, bchar(':'))))(input) {
        (i2, Some(id))
    } else {
        (input, None)
    };

    // Expression
    let (input, expr) = bws(parse_expression)(input)?;

    Ok((input, Argument { name, modifier, expr }))
}

/// Enhanced method invocation parsing
fn enhanced_method_invocation(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        parse_delimited_list0::<_, _, _, _, char, Argument, char, char, Argument>(
            bchar('('),
            parse_invocation_argument,
            bchar(','),
            bchar(')'),
            false, // no trailing comma by default for invocation args (adjust if needed)
            true,  // cut on close
        ),
        PostfixOpKind::Invocation,
    )(input)
}

/// Enhanced member access parsing
fn enhanced_member_access(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        preceded(
            tuple((bws(bchar('.')), nom::combinator::not(bchar('.')))),
            cut(bws(parse_identifier)),
        ),
        PostfixOpKind::MemberAccess,
    )(input)
}

/// Enhanced indexing parsing
fn enhanced_indexing(input: &str) -> BResult<&str, PostfixOpKind> {
    map(
        delimited(
            bws(bchar('[')),
            cut(bws(parse_expression)),
            cut(bws(bchar(']'))),
        ),
        |expr| PostfixOpKind::Indexing(Box::new(expr)),
    )(input)
}

/// Enhanced null conditional access parsing
fn enhanced_null_conditional_access(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        // ?. member access
        map(
            preceded(
                bws(tuple((bchar('?'), bchar('.')))),
                cut(bws(parse_identifier)),
            ),
            PostfixOpKind::NullConditionalMemberAccess,
        ),
        // ?[ indexing
        map(
            delimited(
                bws(tuple((bchar('?'), bchar('[')))),
                cut(bws(parse_expression)),
                cut(bws(bchar(']'))),
            ),
            |expr| PostfixOpKind::NullConditionalIndexing(Box::new(expr)),
        ),
    ))(input)
}

/// Simple postfix operations as fallback
fn simple_postfix_operations(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        map(bws(tuple((bchar('+'), bchar('+')))), |_| {
            PostfixOpKind::PostfixIncrement
        }),
        map(bws(tuple((bchar('-'), bchar('-')))), |_| {
            PostfixOpKind::PostfixDecrement
        }),
        map(bws(bchar('!')), |_| PostfixOpKind::NullForgiving),
    ))(input)
}

/// Parse with-expression postfix: `with { Name = expr, ... }`
fn enhanced_with_expression(input: &str) -> BResult<&str, PostfixOpKind> {
    use nom::branch::alt;
    map(
        tuple((
            bws(kw_with()),
            bws(bchar('{')),
            // zero or more initializers (property or indexer) separated by commas
            nom::multi::separated_list0(
                bws(bchar(',')),
                alt((
                    parse_with_indexer_assignment,
                    map(
                        tuple((bws(parse_identifier), bws(bchar('=')), bws(parse_expression))),
                        |(id, _, expr)| WithInitializerEntry::Property { name: id.name, value: expr },
                    ),
                )),
            ),
            bws(bchar('}')),
        )),
        |(_, _, inits, _)| PostfixOpKind::With(inits),
    )(input)
}

/// Indexer assignment inside with-initializer: [expr (, expr)* ] = expr
fn parse_with_indexer_assignment(input: &str) -> BResult<&str, WithInitializerEntry> {
    use nom::combinator::cut;
    use nom::multi::separated_list1;
    map(
        tuple((
            bws(bchar('[')),
            separated_list1(bws(bchar(',')), bws(parse_expression)),
            cut(bws(bchar(']'))),
            cut(bws(bchar('='))),
            cut(bws(parse_expression)),
        )),
        |(_, indices, _, _, value)| WithInitializerEntry::Indexer { indices, value },
    )(input)
}

/// Enhanced postfix operation syntax with better error recovery
fn enhanced_postfix_operation(input: &str) -> BResult<&str, PostfixOpKind> {
    alt((
        enhanced_member_access,
        enhanced_method_invocation,
        enhanced_indexing,
        enhanced_null_conditional_access,
        enhanced_with_expression,
        simple_postfix_operations,
    ))(input)
}

/// Apply a postfix operation to an expression
fn apply_postfix_operation(expr: Expression, op: PostfixOpKind) -> Expression {
    match op {
        PostfixOpKind::MemberAccess(member) => {
            Expression::MemberAccess(Box::new(MemberAccessExpression {
                object: Box::new(expr),
                member,
            }))
        }
        PostfixOpKind::NullConditionalMemberAccess(member) => {
            Expression::NullConditional(Box::new(NullConditionalExpression {
                target: Box::new(expr),
                member,
                is_element_access: false,
                argument: None,
            }))
        }
        PostfixOpKind::Invocation(args) => Expression::Invocation(Box::new(InvocationExpression {
            callee: Box::new(expr),
            arguments: args,
        })),
        PostfixOpKind::Indexing(index) => Expression::Indexing(Box::new(IndexingExpression {
            target: Box::new(expr),
            index,
        })),
        PostfixOpKind::NullConditionalIndexing(index) => {
            Expression::NullConditional(Box::new(NullConditionalExpression {
                target: Box::new(expr),
                member: Identifier {
                    name: String::new(),
                },
                is_element_access: true,
                argument: Some(index),
            }))
        }
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
        PostfixOpKind::With(inits) => Expression::With { target: Box::new(expr), initializers: inits },
    }
}

/// Enhanced postfix expression syntax using many0 for robust method chaining
/// Handles complex patterns like _userRepository.GetByEmailAsync(email).Result
pub(crate) fn parse_postfix_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    let (input, mut expr) =
        crate::parser::expressions::primary_expression_parser::parse_primary_expression(input)?;
    let (input, postfix_ops) = nom::multi::many0(enhanced_postfix_operation)(input)?;
    for op in postfix_ops {
        expr = apply_postfix_operation(expr, op);
    }
    Ok((input, expr))
}
