use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_is;
use crate::parser::keywords::selection_and_switch_keywords::{
    kw_case, kw_default, kw_switch, kw_when,
};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::Parser;

use crate::syntax::list_parser::parse_delimited_list1;
use nom::combinator::cut;
use nom::{
    branch::alt,
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::{delimited, preceded},
};
use syntax::expressions::expression::{SwitchExpression, SwitchExpressionArm};
use syntax::expressions::{
    BinaryOperator, Expression, InvocationExpression, MemberAccessExpression, Pattern, PatternCase,
};

/// Parse a switch expression (value switch { pattern1 => expr1, pattern2 => expr2, ... })
pub fn parse_switch_expression(input: Span) -> BResult<Expression> {
    fn parse_arms(i: Span) -> BResult<Vec<SwitchExpressionArm>> {
        parse_delimited_list1::<_, _, _, _, char, char, char, SwitchExpressionArm>(
            tok_l_brace(),
            |i| delimited(ws, parse_switch_expression_arm, ws).parse(i),
            tok_comma(),
           tok_r_brace(),
            true,
            true,
        )
            .parse(i)
    }
    map(
        (
            parse_basic_expression, // Use basic expression to avoid recursion
            delimited(ws, kw_switch(), ws),
            delimited(ws, parse_arms, ws),
        ),
        |(expression, _, arms)| {
            Expression::SwitchExpression(Box::new(SwitchExpression { expression, arms }))
        },
    )
        .parse(input.into())
}

/// Parse a single switch expression arm (pattern [when condition] => expression)
fn parse_switch_expression_arm(input: Span) -> BResult<SwitchExpressionArm> {
    map(
        (
            parse_pattern,
            opt(preceded(
                delimited(ws, kw_when(), ws),
                delimited(ws, parse_relational_basic_expression, ws),
            )), // Use relational expression
            delimited(ws, tok_lambda(), ws),
            delimited(ws, parse_basic_expression, ws), // Use basic expression for the result
        ),
        |(pattern, when_clause, _, expression)| SwitchExpressionArm {
            pattern,
            when_clause,
            expression,
        },
    )
        .parse(input.into())
}

/// Parse relational expressions for when clauses
fn parse_relational_basic_expression(input: Span) -> BResult<Expression> {
    let (input, first) = parse_additive_basic_expression(input.into())?;
    let (input, rest) = nom::multi::many0((
        delimited(ws, alt((
            map(tok_ge(), |_| BinaryOperator::GreaterEqual),
            map(tok_le(), |_| BinaryOperator::LessEqual),
            map(tok_equal(), |_| BinaryOperator::Equal),
            map(tok_not_equal(), |_| BinaryOperator::NotEqual),
            map(tok_gt(), |_| BinaryOperator::GreaterThan),
            map(tok_lt(), |_| BinaryOperator::LessThan),
        )), ws),
        parse_additive_basic_expression,
    ))
        .parse(input.into())?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, (op, next)| Expression::Binary {
                left: Box::new(acc),
                op,
                right: Box::new(next),
            }),
    ))
}

/// Parse an "is" pattern expression (expression is pattern)
pub fn parse_is_pattern_expression(input: Span) -> BResult<Expression> {
    // First try to parse a basic expression (no recursion here)
    let (input, expr) = parse_basic_expression(input.into())?;

    // Then check for "is" keyword followed by pattern
    if let Ok((input, _)) = delimited(ws, kw_is(), ws).parse(input.into()) {
        let (input, pattern) = delimited(ws, parse_pattern, ws).parse(input.into())?;
        Ok((
            input,
            Expression::IsPattern {
                expression: Box::new(expr),
                pattern: Box::new(pattern),
            },
        ))
    } else {
        Ok((input, expr))
    }
}

/// Parse basic expressions for switch/is patterns (NO RECURSION)
/// This follows Nom's principle of small, specific parser
fn parse_basic_expression(input: Span) -> BResult<Expression> {
    parse_additive_basic_expression(input.into())
}

/// Parse additive expressions (+ and -)
fn parse_additive_basic_expression(input: Span) -> BResult<Expression> {
    let (input, first) = parse_multiplicative_basic_expression(input.into())?;
    let (input, rest) = nom::multi::many0((
        delimited(ws, alt((
            map(nom_char('+'), |_| BinaryOperator::Add),
            map(nom_char('-'), |_| BinaryOperator::Subtract),
        )), ws),
        parse_multiplicative_basic_expression,
    ))
        .parse(input.into())?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, (op, next)| Expression::Binary {
                left: Box::new(acc),
                op,
                right: Box::new(next),
            }),
    ))
}

/// Parse multiplicative expressions (*, /, %)
fn parse_multiplicative_basic_expression(input: Span) -> BResult<Expression> {
    let (input, first) = parse_primary_basic_expression(input.into())?;
    let (input, rest) = nom::multi::many0((
        delimited(ws, alt((
            map(nom_char('*'), |_| BinaryOperator::Multiply),
            map(nom_char('/'), |_| BinaryOperator::Divide),
            map(nom_char('%'), |_| BinaryOperator::Modulo),
        )), ws),
        parse_primary_basic_expression,
    ))
        .parse(input.into())?;

    // Fold the results into a left-associative tree
    Ok((
        input,
        rest.into_iter()
            .fold(first, |acc, (op, next)| Expression::Binary {
                left: Box::new(acc),
                op,
                right: Box::new(next),
            }),
    ))
}

/// Parse primary basic expressions
fn parse_primary_basic_expression(input: Span) -> BResult<Expression> {
    map(alt((
        // Literals first (most specific)
        map(parse_literal, Expression::Literal),
        // Identifiers (variables)
        map(parse_identifier, Expression::Variable),
        // Parenthesized basic expressions
        delimited(
            delimited(ws, tok_l_paren(), ws),
            parse_basic_expression,
            cut(delimited(ws, tok_r_paren(), ws)),
        ),
        // Member access: obj.member (but no further nesting)
        map(
            (
                parse_identifier,
                preceded(nom_char('.'), parse_identifier),
            ),
            |(obj, member)| {
                Expression::MemberAccess(Box::new(MemberAccessExpression {
                    object: Box::new(Expression::Variable(obj)),
                    member,
                }))
            },
        ),
        // Method calls on basic expressions: obj.Method()
        map(
            (
                parse_identifier,
                preceded(nom_char('.'), parse_identifier),
                delimited(
                    delimited(ws, tok_l_paren(), ws),
                    nom::multi::separated_list0(
                        delimited(ws, tok_comma(), ws),
                        parse_basic_expression,
                    ),
                    cut(delimited(ws, tok_r_paren(), ws)),
                ),
            ),
            |(obj, method, args)| {
                use syntax::expressions::invocation_expression::Argument;
                Expression::Invocation(Box::new(InvocationExpression {
                    callee: Box::new(Expression::MemberAccess(Box::new(MemberAccessExpression {
                        object: Box::new(Expression::Variable(obj)),
                        member: method,
                    }))),
                    arguments: args
                        .into_iter()
                        .map(|expr| Argument {
                            name: None,
                            modifier: None,
                            expr,
                        })
                        .collect(),
                }))
            },
        ),
    )), |v| v)
        .parse(input.into())
}

/// Parse switch statement cases for traditional switch statements
pub fn parse_switch_case(input: Span) -> BResult<PatternCase> {
    map(alt((
        // case pattern [when condition]:
        map(
            (
                delimited(ws, kw_case(), ws),
                delimited(ws, parse_pattern, ws),
                opt(preceded(
                    delimited(ws, kw_when(), ws),
                    delimited(ws, parse_basic_expression, ws),
                )),
                delimited(ws, tok_colon(), ws),
                // For simplicity, we'll parse the body as a single expression
                // In a real implementation, this would parse a list of statements
                delimited(ws, parse_basic_expression, ws),
            ),
            |(_, pattern, when_clause, _, body)| PatternCase {
                pattern,
                when_clause,
                body: vec![body],
            },
        ),
        // default:
        map(
            (
                delimited(ws, kw_default(), ws),
                delimited(ws, tok_colon(), ws),
                delimited(ws, parse_basic_expression, ws),
            ),
            |(_, _, body)| PatternCase {
                pattern: Pattern::Discard, // default is like a discard pattern
                when_clause: None,
                body: vec![body],
            },
        ),
    )), |v| v)
        .parse(input.into())
}
use crate::syntax::span::Span;
use crate::tokens::delimiters::{tok_l_brace, tok_l_paren, tok_r_brace, tok_r_paren};
use crate::tokens::equality::{tok_equal, tok_not_equal};
use crate::tokens::lambda::tok_lambda;
use crate::tokens::relational::{tok_ge, tok_gt, tok_le, tok_lt};
use crate::tokens::separators::{tok_colon, tok_comma};
