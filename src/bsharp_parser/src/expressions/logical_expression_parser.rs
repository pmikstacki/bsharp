use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::expressions::precedence::left_chain;
use crate::parser::expressions::range_expression_parser::parse_range_expression_or_higher;
use crate::parser::keywords::expression_keywords::{kw_as, kw_is};
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use nom::branch::alt;
use nom::combinator::{map, recognize, not};
use nom::sequence::{pair, preceded, tuple};
use nom::character::complete::char as nom_char;
use nom::Parser;
use syntax::expressions::{BinaryOperator, Expression};
use crate::syntax::span::Span;

pub(crate) fn parse_logical_or_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_logical_and_expression_or_higher, |i| {
        map(
            nom::sequence::delimited(ws, recognize(pair(nom_char('|'), nom_char('|'))), ws),
            |_| BinaryOperator::LogicalOr,
        )
        .parse(i)
    })(input)
}

fn parse_logical_and_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_bitwise_or_expression_or_higher, |i| {
        map(
            nom::sequence::delimited(ws, recognize(pair(nom_char('&'), nom_char('&'))), ws),
            |_| BinaryOperator::LogicalAnd,
        )
        .parse(i)
    })(input)
}

fn parse_bitwise_or_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_bitwise_xor_expression_or_higher, |i| {
        map(
            nom::sequence::delimited(
                ws,
                tuple((nom_char('|'), not(alt((nom_char('='), nom_char('|')))))),
                ws,
            ),
            |_| BinaryOperator::BitwiseOr,
        )
        .parse(i)
    })(input)
}

fn parse_bitwise_xor_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_bitwise_and_expression_or_higher, |i| {
        map(
            nom::sequence::delimited(ws, tuple((nom_char('^'), not(nom_char('=')))), ws),
            |_| BinaryOperator::BitwiseXor,
        )
        .parse(i)
    })(input)
}

fn parse_bitwise_and_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_equality_expression_or_higher, |i| {
        map(
            nom::sequence::delimited(
                ws,
                tuple((nom_char('&'), not(alt((nom_char('='), nom_char('&')))))),
                ws,
            ),
            |_| BinaryOperator::BitwiseAnd,
        )
        .parse(i)
    })(input)
}

fn parse_equality_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_type_test_expression_or_higher, |i| {
        nom::sequence::delimited(
            ws,
            alt((
                map(recognize(pair(nom_char('='), nom_char('='))), |_| BinaryOperator::Equal),
                map(recognize(pair(nom_char('!'), nom_char('='))), |_| BinaryOperator::NotEqual),
            )),
            ws,
        )
        .parse(i)
    })(input)
}

fn parse_relational_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_shift_expression_or_higher, |i| {
        // Guard: if the next tokens begin a shift or shift-assign sequence, do not match relational here
        let guard = not(alt((
            recognize(tuple((nom_char('<'), nom_char('<'), nom_char('=')))), // <<=
            recognize(tuple((nom_char('>'), nom_char('>'), nom_char('=')))), // >>=
            recognize(tuple((nom_char('>'), nom_char('>'), nom_char('>')))), // >>>
            recognize(tuple((nom_char('<'), nom_char('<')))),                // <<
            recognize(tuple((nom_char('>'), nom_char('>')))),                // >>
        )));

        nom::sequence::delimited(
            ws,
            map(
                preceded(
                    guard,
                    alt((
                        map(recognize(pair(nom_char('<'), nom_char('='))), |_| BinaryOperator::LessEqual),
                        map(recognize(pair(nom_char('>'), nom_char('='))), |_| BinaryOperator::GreaterEqual),
                        // Single '<'
                        map(tuple((nom_char('<'), not(alt((nom_char('<'), nom_char('=')))))), |_| BinaryOperator::LessThan),
                        // Single '>'
                        map(tuple((nom_char('>'), not(alt((nom_char('>'), nom_char('=')))))), |_| BinaryOperator::GreaterThan),
                    )),
                ),
                |op| op,
            ),
            ws,
        )
        .parse(i)
    })(input)
}

/// Relational and type-testing stage: handles is-pattern and as-operator at same precedence as relational
fn parse_type_test_expression_or_higher(input: Span) -> BResult<Expression> {
    // Start with relational expression
    let (mut input, mut left) = parse_relational_expression_or_higher(input)?;

    loop {
        // Try 'is' pattern first
        if let Ok((after_is, _)) = nom::sequence::delimited(ws, kw_is(), ws).parse(input) {
            let (after_pat, pat) = nom::sequence::delimited(ws, parse_pattern, ws).parse(after_is)?;
            left = Expression::IsPattern {
                expression: Box::new(left),
                pattern: Box::new(pat),
            };
            input = after_pat;
            continue;
        }

        // Try 'as' type cast-like operator
        if let Ok((after_as, _)) = nom::sequence::delimited(ws, kw_as(), ws).parse(input) {
            let (after_ty, ty) = nom::sequence::delimited(ws, parse_type_expression, ws).parse(after_as)?;
            left = Expression::As {
                expression: Box::new(left),
                target_type: ty,
            };
            input = after_ty;
            continue;
        }

        break;
    }

    Ok((input, left))
}

fn parse_shift_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_additive_expression_or_higher, |i| {
        nom::sequence::delimited(ws, alt((
            // >>> unsigned right shift (ensure not followed by '=')
            map(
                tuple((
                    nom_char('>'),
                    nom_char('>'),
                    nom_char('>'),
                    not(nom_char('=')),
                )),
                |_| BinaryOperator::UnsignedRightShift,
            ),
            map(
                tuple((nom_char('<'), nom_char('<'), not(nom_char('=')))),
                |_| BinaryOperator::LeftShift,
            ),
            map(
                tuple((nom_char('>'), nom_char('>'), not(nom_char('=')))),
                |_| BinaryOperator::RightShift,
            ),
        )), ws)
        .parse(i)
    })(input)
}

fn parse_additive_expression_or_higher(input: Span) -> BResult<Expression> {
    left_chain(parse_multiplicative_expression_or_higher, |i| {
        nom::sequence::delimited(ws, alt((
            map(
                tuple((nom_char('+'), not(nom_char('=')))),
                |_| BinaryOperator::Add,
            ),
            map(
                tuple((nom_char('-'), not(nom_char('=')))),
                |_| BinaryOperator::Subtract,
            ),
        )), ws)
        .parse(i)
    })(input)
}

fn parse_multiplicative_expression_or_higher(input: Span) -> BResult<Expression> {
    // Use the generic left-associative chain builder with the same operator lookahead rules
    left_chain(parse_range_expression_or_higher, |i| {
        nom::sequence::delimited(ws, alt((
            map(
                tuple((nom_char('*'), not(nom_char('=')))),
                |_| BinaryOperator::Multiply,
            ),
            map(
                tuple((nom_char('/'), not(nom_char('=')))),
                |_| BinaryOperator::Divide,
            ),
            map(
                tuple((nom_char('%'), not(nom_char('=')))),
                |_| BinaryOperator::Modulo,
            ),
        )), ws)
        .parse(i)
    })(input)
}
