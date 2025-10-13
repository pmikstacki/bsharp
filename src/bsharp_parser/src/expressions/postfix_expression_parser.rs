use crate::syntax::span::Span;
use crate::parser::expressions::primary_expression_parser::{parse_expression, parse_primary_expression};
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_with;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;

use nom::{
    branch::alt,
    combinator::{cut, map},
    sequence::{delimited, preceded, tuple},
};
use nom::character::complete::char as nom_char;
use nom::Parser;
use syntax::expressions::expression::WithInitializerEntry;
use syntax::expressions::indexing_expression::IndexingExpression;
use syntax::expressions::invocation_expression::{Argument, ArgumentModifier};
use syntax::expressions::{
    Expression, InvocationExpression, MemberAccessExpression, NullConditionalExpression,
    UnaryOperator,
};
use syntax::Identifier;
use crate::syntax::list_parser::parse_delimited_list0;
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
fn parse_invocation_argument(input: Span) -> BResult<Argument> {
    use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_ref};
    use nom::branch::alt;
    use nom::combinator::map;
    use nom::combinator::opt;

    // Optional modifier
    let (input, modifier) = delimited(ws, opt(alt((
        map(kw_ref(), |_| ArgumentModifier::Ref),
        map(kw_out(), |_| ArgumentModifier::Out),
        map(kw_in(), |_| ArgumentModifier::In),
    ))), ws).parse(input)?;

    // Optional name label: identifier:
    let (input, name) = if let Ok((i2, (id, _))) =
        nom::sequence::delimited(ws, nom::sequence::tuple((parse_identifier, nom_char(':'))), ws).parse(input)
    {
        (i2, Some(id))
    } else {
        (input, None)
    };

    // Expression
    let (input, expr) = delimited(ws, parse_expression, ws).parse(input)?;

    Ok((
        input,
        Argument {
            name,
            modifier,
            expr,
        },
    ))
}

/// Enhanced method invocation parsing
fn enhanced_method_invocation(input: Span) -> BResult<PostfixOpKind> {
    fn parse_args(i: Span) -> BResult<Vec<Argument>> {
        parse_delimited_list0::<_, _, _, _, char, Argument, char, char, Argument>(
            |i| delimited(ws, nom_char('('), ws).parse(i),
            |i| delimited(ws, parse_invocation_argument, ws).parse(i),
            |i| delimited(ws, nom_char(','), ws).parse(i),
            |i| delimited(ws, nom_char(')'), ws).parse(i),
            false,
            true,
        )
        .parse(i)
    }
    map(parse_args, PostfixOpKind::Invocation).parse(input)
}

/// Enhanced member access parsing
fn enhanced_member_access(input: Span) -> BResult<PostfixOpKind> {
    map(
        preceded(
            tuple((delimited(ws, nom_char('.'), ws), nom::combinator::not(nom_char('.')))),
            cut(delimited(ws, parse_identifier, ws)),
        ),
        PostfixOpKind::MemberAccess,
    )
    .parse(input)
}

/// Enhanced indexing parsing
fn enhanced_indexing(input: Span) -> BResult<PostfixOpKind> {
    map(
        delimited(
            delimited(ws, nom_char('['), ws),
            cut(delimited(ws, parse_expression, ws)),
            cut(delimited(ws, nom_char(']'), ws)),
        ),
        |expr| PostfixOpKind::Indexing(Box::new(expr)),
    )
    .parse(input)
}

/// Enhanced null conditional access parsing
fn enhanced_null_conditional_access(input: Span) -> BResult<PostfixOpKind> {
    nom::combinator::map(alt((
        // ?. member access
        map(
            preceded(
                delimited(ws, tuple((nom_char('?'), nom_char('.'))), ws),
                cut(delimited(ws, parse_identifier, ws)),
            ),
            PostfixOpKind::NullConditionalMemberAccess,
        ),
        // ?[ indexing
        map(
            delimited(
                delimited(ws, tuple((nom_char('?'), nom_char('['))), ws),
                cut(delimited(ws, parse_expression, ws)),
                cut(delimited(ws, nom_char(']'), ws)),
            ),
            |expr| PostfixOpKind::NullConditionalIndexing(Box::new(expr)),
        ),
    )), |v| v)
    .parse(input)
}

/// Simple postfix operations as fallback
fn simple_postfix_operations(input: Span) -> BResult<PostfixOpKind> {
    nom::combinator::map(alt((
        map(delimited(ws, tuple((nom_char('+'), nom_char('+'))), ws), |_| {
            PostfixOpKind::PostfixIncrement
        }),
        map(delimited(ws, tuple((nom_char('-'), nom_char('-'))), ws), |_| {
            PostfixOpKind::PostfixDecrement
        }),
        map(delimited(ws, nom_char('!'), ws), |_| PostfixOpKind::NullForgiving),
    )), |v| v)
    .parse(input)
}

/// Parse with-expression postfix: `with { Name = expr, ... }`
fn enhanced_with_expression(input: Span) -> BResult<PostfixOpKind> {
    use nom::branch::alt;
    map(
        tuple((
            delimited(ws, kw_with(), ws),
            delimited(ws, nom_char('{'), ws),
            // zero or more initializers (property or indexer) separated by commas
            nom::multi::separated_list0(
                delimited(ws, nom_char(','), ws),
                alt((
                    parse_with_indexer_assignment,
                    map(
                        tuple((
                            delimited(ws, parse_identifier, ws),
                            delimited(ws, nom_char('='), ws),
                            delimited(ws, parse_expression, ws),
                        )),
                        |(id, _, expr)| WithInitializerEntry::Property { name: id.name, value: expr },
                    ),
                )),
            ),
            delimited(ws, nom_char('}'), ws),
        )),
        |(_, _, inits, _)| PostfixOpKind::With(inits),
    )
    .parse(input)
}

/// Indexer assignment inside with-initializer: [expr (, expr)* ] = expr
fn parse_with_indexer_assignment(input: Span) -> BResult<WithInitializerEntry> {
    use nom::combinator::cut;
    use nom::multi::separated_list1;
    map(
        (
            delimited(ws, nom_char('['), ws),
            separated_list1(delimited(ws, nom_char(','), ws), delimited(ws, parse_expression, ws)),
            cut(delimited(ws, nom_char(']'), ws)),
            cut(delimited(ws, nom_char('='), ws)),
            cut(delimited(ws, parse_expression, ws)),
        ),
        |(_, indices, _, _, value)| WithInitializerEntry::Indexer { indices, value },
    )
    .parse(input)
}

/// Enhanced postfix operation syntax with better error recovery
fn enhanced_postfix_operation(input: Span) -> BResult<PostfixOpKind> {
    nom::combinator::map(alt((
        enhanced_member_access,
        enhanced_method_invocation,
        enhanced_indexing,
        enhanced_null_conditional_access,
        enhanced_with_expression,
        simple_postfix_operations,
    )), |v| v)
    .parse(input)
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
        PostfixOpKind::With(inits) => Expression::With {
            target: Box::new(expr),
            initializers: inits,
        },
    }
}

/// Parse with-expression postfix: `with { Name = expr, ... }`
pub(crate) fn parse_dotted_member_expression(input: Span) -> BResult<Expression> {
    let (input, first) = delimited(ws, parse_identifier, ws).parse(input)?;
    let (input, rest) = nom::multi::many0(preceded(delimited(ws, nom_char('.'), ws), delimited(ws, parse_identifier, ws))).parse(input)?;
    if rest.is_empty() {
        Ok((input, Expression::Variable(first)))
    } else {
        let mut expr = Expression::Variable(first);
        for id in rest {
            expr = apply_postfix_operation(expr, PostfixOpKind::MemberAccess(id));
        }
        Ok((input, expr))
    }
}

/// Parse a postfix-expression-or-higher: primary-expression followed by zero or more postfix operations
pub(crate) fn parse_postfix_expression_or_higher(input: Span) -> BResult<Expression> {
    let (mut cur, base) = delimited(ws, parse_primary_expression, ws).parse(input)?;
    let mut expr = base;
    loop {
        match delimited(ws, enhanced_postfix_operation, ws).parse(cur) {
            Ok((next, op)) => {
                expr = apply_postfix_operation(expr, op);
                cur = next;
            }
            Err(_) => break,
        }
    }
    Ok((cur, expr))
}
