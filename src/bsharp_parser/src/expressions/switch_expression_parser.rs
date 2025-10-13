use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::keywords::expression_keywords::kw_is;
use crate::parser::keywords::selection_and_switch_keywords::{
    kw_case, kw_default, kw_switch, kw_when,
};
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom::Parser;

use nom::combinator::cut;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
};
use syntax::expressions::expression::{SwitchExpression, SwitchExpressionArm};
use syntax::expressions::{
    BinaryOperator, Expression, InvocationExpression, MemberAccessExpression, Pattern, PatternCase,
};
use crate::syntax::list_parser::parse_delimited_list1;

/// Parse a switch expression (value switch { pattern1 => expr1, pattern2 => expr2, ... })
pub fn parse_switch_expression(input: Span) -> BResult<Expression> {
    fn parse_arms(i: Span) -> BResult<Vec<SwitchExpressionArm>> {
        parse_delimited_list1::<_, _, _, _, char, SwitchExpressionArm, char, char, SwitchExpressionArm>(
            nom_char('{'),
            |i| delimited(ws, parse_switch_expression_arm, ws).parse(i),
            nom_char(','),
            nom_char('}'),
            true,
            true,
        )
        .parse(i)
    }
    map(
        tuple((
            parse_basic_expression, // Use basic expression to avoid recursion
            delimited(ws, kw_switch(), ws),
            delimited(ws, parse_arms, ws),
        )),
        |(expression, _, arms)| {
            Expression::SwitchExpression(Box::new(SwitchExpression { expression, arms }))
        },
    )
    .parse(input)
}

/// Parse a single switch expression arm (pattern [when condition] => expression)
fn parse_switch_expression_arm(input: Span) -> BResult<SwitchExpressionArm> {
    map(
        tuple((
            parse_pattern,
            opt(preceded(
                delimited(ws, kw_when(), ws),
                delimited(ws, parse_relational_basic_expression, ws),
            )), // Use relational expression
            delimited(ws, tag("=>"), ws),
            delimited(ws, parse_basic_expression, ws), // Use basic expression for the result
        )),
        |(pattern, when_clause, _, expression)| SwitchExpressionArm {
            pattern,
            when_clause,
            expression,
        },
    )
    .parse(input)
}

/// Parse relational expressions for when clauses
fn parse_relational_basic_expression(input: Span) -> BResult<Expression> {
    let (input, first) = parse_additive_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        delimited(ws, alt((
            map(tag(">="), |_| BinaryOperator::GreaterEqual),
            map(tag("<="), |_| BinaryOperator::LessEqual),
            map(tag("=="), |_| BinaryOperator::Equal),
            map(tag("!="), |_| BinaryOperator::NotEqual),
            map(nom_char('>'), |_| BinaryOperator::GreaterThan),
            map(nom_char('<'), |_| BinaryOperator::LessThan),
        )), ws),
        parse_additive_basic_expression,
    )))
    .parse(input)?;

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
    let (input, expr) = parse_basic_expression(input)?;

    // Then check for "is" keyword followed by pattern
    if let Ok((input, _)) = delimited(ws, kw_is(), ws).parse(input) {
        let (input, pattern) = delimited(ws, parse_pattern, ws).parse(input)?;
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
    parse_additive_basic_expression(input)
}

/// Parse additive expressions (+ and -)
fn parse_additive_basic_expression(input: Span) -> BResult<Expression> {
    let (input, first) = parse_multiplicative_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        delimited(ws, alt((
            map(nom_char('+'), |_| BinaryOperator::Add),
            map(nom_char('-'), |_| BinaryOperator::Subtract),
        )), ws),
        parse_multiplicative_basic_expression,
    )))
    .parse(input)?;

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
    let (input, first) = parse_primary_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        delimited(ws, alt((
            map(nom_char('*'), |_| BinaryOperator::Multiply),
            map(nom_char('/'), |_| BinaryOperator::Divide),
            map(nom_char('%'), |_| BinaryOperator::Modulo),
        )), ws),
        parse_primary_basic_expression,
    )))
    .parse(input)?;

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
    nom::combinator::map(alt((
        // Literals first (most specific)
        map(parse_literal, Expression::Literal),
        // Identifiers (variables)
        map(parse_identifier, Expression::Variable),
        // Parenthesized basic expressions
        delimited(
            delimited(ws, nom_char('('), ws),
            parse_basic_expression,
            cut(delimited(ws, nom_char(')'), ws)),
        ),
        // Member access: obj.member (but no further nesting)
        map(
            tuple((
                parse_identifier,
                preceded(nom_char('.'), parse_identifier),
            )),
            |(obj, member)| {
                Expression::MemberAccess(Box::new(MemberAccessExpression {
                    object: Box::new(Expression::Variable(obj)),
                    member,
                }))
            },
        ),
        // Method calls on basic expressions: obj.Method()
        map(
            tuple((
                parse_identifier,
                preceded(nom_char('.'), parse_identifier),
                delimited(
                    delimited(ws, nom_char('('), ws),
                    nom::multi::separated_list0(
                        delimited(ws, nom_char(','), ws),
                        parse_basic_expression,
                    ),
                    cut(delimited(ws, nom_char(')'), ws)),
                ),
            )),
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
    .parse(input)
}

/// Parse switch statement cases for traditional switch statements
pub fn parse_switch_case(input: Span) -> BResult<PatternCase> {
    nom::combinator::map(alt((
        // case pattern [when condition]:
        map(
            tuple((
                delimited(ws, kw_case(), ws),
                delimited(ws, parse_pattern, ws),
                opt(preceded(
                    delimited(ws, kw_when(), ws),
                    delimited(ws, parse_basic_expression, ws),
                )),
                delimited(ws, nom_char(':'), ws),
                // For simplicity, we'll parse the body as a single expression
                // In a real implementation, this would parse a list of statements
                delimited(ws, parse_basic_expression, ws),
            )),
            |(_, pattern, when_clause, _, body)| PatternCase {
                pattern,
                when_clause,
                body: vec![body],
            },
        ),
        // default:
        map(
            tuple((
                delimited(ws, kw_default(), ws),
                delimited(ws, nom_char(':'), ws),
                delimited(ws, parse_basic_expression, ws),
            )),
            |(_, _, body)| PatternCase {
                pattern: Pattern::Discard, // default is like a discard pattern
                when_clause: None,
                body: vec![body],
            },
        ),
    )), |v| v)
    .parse(input)
}
use crate::syntax::span::Span;
