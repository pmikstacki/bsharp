use crate::parser::expressions::literal_parser::parse_literal;
use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::expression::{
    Expression, SwitchExpression, SwitchExpressionArm,
};
use crate::syntax::nodes::expressions::pattern::{Pattern, PatternCase};
use crate::syntax::parser_helpers::{bws, keyword, parse_delimited_list1};

use nom::combinator::cut;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char as nom_char,
    combinator::{map, opt},
    sequence::{delimited, preceded, tuple},
};

/// Parse a switch expression (value switch { pattern1 => expr1, pattern2 => expr2, ... })
pub fn parse_switch_expression(input: &str) -> BResult<&str, Expression> {
    map(
        tuple((
            parse_basic_expression, // Use basic expression to avoid recursion
            bws(keyword("switch")),
            bws(parse_delimited_list1::<_, _, _, _, char, SwitchExpressionArm, char, char, SwitchExpressionArm>(
                nom_char('{'),
                |i| bws(parse_switch_expression_arm)(i),
                nom_char(','),
                nom_char('}'),
                true,
                true,
            )),
        )),
        |(expression, _, arms)| {
            Expression::SwitchExpression(Box::new(SwitchExpression { expression, arms }))
        },
    )(input)
}

/// Parse a single switch expression arm (pattern [when condition] => expression)
fn parse_switch_expression_arm(input: &str) -> BResult<&str, SwitchExpressionArm> {
    map(
        tuple((
            parse_pattern,
            opt(preceded(
                bws(keyword("when")),
                bws(parse_relational_basic_expression),
            )), // Use relational expression
            bws(tag("=>")),
            bws(parse_basic_expression), // Use basic expression for the result
        )),
        |(pattern, when_clause, _, expression)| SwitchExpressionArm {
            pattern,
            when_clause,
            expression,
        },
    )(input)
}

/// Parse relational expressions for when clauses
fn parse_relational_basic_expression(input: &str) -> BResult<&str, Expression> {
    let (input, first) = parse_additive_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        bws(alt((
            map(tag(">="), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::GreaterEqual
            }),
            map(tag("<="), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::LessEqual
            }),
            map(tag("=="), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Equal
            }),
            map(tag("!="), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::NotEqual
            }),
            map(nom_char('>'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::GreaterThan
            }),
            map(nom_char('<'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::LessThan
            }),
        ))),
        parse_additive_basic_expression,
    )))(input)?;

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
pub fn parse_is_pattern_expression(input: &str) -> BResult<&str, Expression> {
    // First try to parse a basic expression (no recursion here)
    let (input, expr) = parse_basic_expression(input)?;

    // Then check for "is" keyword followed by pattern
    if let Ok((input, _)) = bws(keyword("is"))(input) {
        let (input, pattern) = bws(parse_pattern)(input)?;
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
fn parse_basic_expression(input: &str) -> BResult<&str, Expression> {
    parse_additive_basic_expression(input)
}

/// Parse additive expressions (+ and -)
fn parse_additive_basic_expression(input: &str) -> BResult<&str, Expression> {
    let (input, first) = parse_multiplicative_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        bws(alt((
            map(nom_char('+'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Add
            }),
            map(nom_char('-'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Subtract
            }),
        ))),
        parse_multiplicative_basic_expression,
    )))(input)?;

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
fn parse_multiplicative_basic_expression(input: &str) -> BResult<&str, Expression> {
    let (input, first) = parse_primary_basic_expression(input)?;
    let (input, rest) = nom::multi::many0(tuple((
        bws(alt((
            map(nom_char('*'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Multiply
            }),
            map(nom_char('/'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Divide
            }),
            map(nom_char('%'), |_| {
                crate::syntax::nodes::expressions::BinaryOperator::Modulo
            }),
        ))),
        parse_primary_basic_expression,
    )))(input)?;

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
fn parse_primary_basic_expression(input: &str) -> BResult<&str, Expression> {
    alt((
        // Literals first (most specific)
        map(parse_literal, Expression::Literal),
        // Identifiers (variables)
        map(parse_identifier, Expression::Variable),
        // Parenthesized basic expressions
        delimited(
            bws(nom_char('(')),
            parse_basic_expression,
            cut(bws(nom_char(')'))),
        ),
        // Member access: obj.member (but no further nesting)
        map(
            tuple((parse_identifier, preceded(nom_char('.'), parse_identifier))),
            |(obj, member)| {
                Expression::MemberAccess(Box::new(
                crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                    object: Box::new(Expression::Variable(obj)),
                    member,
                }
            ))
            },
        ),
        // Method calls on basic expressions: obj.Method()
        map(
            tuple((
                parse_identifier,
                preceded(nom_char('.'), parse_identifier),
                delimited(
                    bws(nom_char('(')),
                    nom::multi::separated_list0(bws(nom_char(',')), parse_basic_expression),
                    cut(bws(nom_char(')'))),
                ),
            )),
            |(obj, method, args)| {
                Expression::Invocation(Box::new(
                crate::syntax::nodes::expressions::invocation_expression::InvocationExpression {
                    callee: Box::new(Expression::MemberAccess(Box::new(
                        crate::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                            object: Box::new(Expression::Variable(obj)),
                            member: method,
                        }
                    ))),
                    arguments: args,
                }
            ))
            },
        ),
    ))(input)
}

/// Parse switch statement cases for traditional switch statements
pub fn parse_switch_case(input: &str) -> BResult<&str, PatternCase> {
    alt((
        // case pattern [when condition]:
        map(
            tuple((
                keyword("case"),
                bws(parse_pattern),
                opt(preceded(bws(keyword("when")), bws(parse_basic_expression))),
                bws(nom_char(':')),
                // For simplicity, we'll parse the body as a single expression
                // In a real implementation, this would parse a list of statements
                bws(parse_basic_expression),
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
                keyword("default"),
                bws(nom_char(':')),
                bws(parse_basic_expression),
            )),
            |(_, _, body)| PatternCase {
                pattern: Pattern::Discard, // default is like a discard pattern
                when_clause: None,
                body: vec![body],
            },
        ),
    ))(input)
}
