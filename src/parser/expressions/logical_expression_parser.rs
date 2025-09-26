use crate::parser::expressions::precedence::left_chain;
use crate::parser::expressions::range_expression_parser::parse_range_expression_or_higher;
use crate::parser::expressions::pattern_parser::parse_pattern;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{BinaryOperator, Expression};
use crate::syntax::parser_helpers::{bchar, bws};
use nom::branch::alt;
use nom::combinator::{map, recognize};
use nom::sequence::{pair, tuple, preceded};
use crate::parser::keywords::expression_keywords::{kw_as, kw_is};

pub(crate) fn parse_logical_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_logical_and_expression_or_higher, |i| {
        map(bws(recognize(pair(bchar('|'), bchar('|')))), |_| {
            BinaryOperator::LogicalOr
        })(i)
    })(input)
}

fn parse_logical_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_bitwise_or_expression_or_higher, |i| {
        map(bws(recognize(pair(bchar('&'), bchar('&')))), |_| {
            BinaryOperator::LogicalAnd
        })(i)
    })(input)
}

fn parse_bitwise_or_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_bitwise_xor_expression_or_higher, |i| {
        map(
            bws(tuple((
                bchar('|'),
                nom::combinator::not(alt((bchar('='), bchar('|')))),
            ))),
            |_| BinaryOperator::BitwiseOr,
        )(i)
    })(input)
}

fn parse_bitwise_xor_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_bitwise_and_expression_or_higher, |i| {
        map(
            bws(tuple((bchar('^'), nom::combinator::not(bchar('='))))),
            |_| BinaryOperator::BitwiseXor,
        )(i)
    })(input)
}

fn parse_bitwise_and_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_equality_expression_or_higher, |i| {
        map(
            bws(tuple((
                bchar('&'),
                nom::combinator::not(alt((bchar('='), bchar('&')))),
            ))),
            |_| BinaryOperator::BitwiseAnd,
        )(i)
    })(input)
}

fn parse_equality_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_type_test_expression_or_higher, |i| {
        bws(alt((
            map(recognize(pair(bchar('='), bchar('='))), |_| {
                BinaryOperator::Equal
            }),
            map(recognize(pair(bchar('!'), bchar('='))), |_| {
                BinaryOperator::NotEqual
            }),
        )))(i)
    })(input)
}

fn parse_relational_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_shift_expression_or_higher, |i| {
        // Guard: if the next tokens begin a shift or shift-assign sequence, do not match relational here
        let guard = nom::combinator::not(alt((
            recognize(tuple((bchar('<'), bchar('<'), bchar('=')))), // <<=
            recognize(tuple((bchar('>'), bchar('>'), bchar('=')))), // >>=
            recognize(tuple((bchar('>'), bchar('>'), bchar('>')))), // >>>
            recognize(tuple((bchar('<'), bchar('<')))),            // <<
            recognize(tuple((bchar('>'), bchar('>')))),            // >>
        )));

        bws(map(
            preceded(
                guard,
                alt((
                    map(recognize(pair(bchar('<'), bchar('='))), |_| {
                        BinaryOperator::LessEqual
                    }),
                    map(recognize(pair(bchar('>'), bchar('='))), |_| {
                        BinaryOperator::GreaterEqual
                    }),
                    // Single '<' relational: ensure it's not the start of '<<' or '<='
                    map(
                        tuple((bchar('<'), nom::combinator::not(alt((bchar('<'), bchar('=')))))),
                        |_| BinaryOperator::LessThan,
                    ),
                    // Single '>' relational: ensure it's not the start of '>>' or '>='
                    map(
                        tuple((bchar('>'), nom::combinator::not(alt((bchar('>'), bchar('=')))))),
                        |_| BinaryOperator::GreaterThan,
                    ),
                )),
            ),
            |op| op,
        ))(i)
    })(input)
}

/// Relational and type-testing stage: handles is-pattern and as-operator at same precedence as relational
fn parse_type_test_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Start with relational expression
    let (mut input, mut left) = parse_relational_expression_or_higher(input)?;

    loop {
        // Try 'is' pattern first
        if let Ok((after_is, _)) = bws(kw_is())(input) {
            let (after_pat, pat) = bws(parse_pattern)(after_is)?;
            left = Expression::IsPattern {
                expression: Box::new(left),
                pattern: Box::new(pat),
            };
            input = after_pat;
            continue;
        }

        // Try 'as' type cast-like operator
        if let Ok((after_as, _)) = bws(kw_as())(input) {
            let (after_ty, ty) = bws(parse_type_expression)(after_as)?;
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

fn parse_shift_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_additive_expression_or_higher, |i| {
        bws(alt((
            // >>> unsigned right shift (ensure not followed by '=')
            map(
                tuple((
                    bchar('>'),
                    bchar('>'),
                    bchar('>'),
                    nom::combinator::not(bchar('=')),
                )),
                |_| BinaryOperator::UnsignedRightShift,
            ),
            map(
                tuple((bchar('<'), bchar('<'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::LeftShift,
            ),
            map(
                tuple((bchar('>'), bchar('>'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::RightShift,
            ),
        )))(i)
    })(input)
}

fn parse_additive_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_multiplicative_expression_or_higher, |i| {
        bws(alt((
            map(
                tuple((bchar('+'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::Add,
            ),
            map(
                tuple((bchar('-'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::Subtract,
            ),
        )))(i)
    })(input)
}

fn parse_multiplicative_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    // Use the generic left-associative chain builder with the same operator lookahead rules
    left_chain(parse_range_expression_or_higher, |i| {
        bws(alt((
            map(
                tuple((bchar('*'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::Multiply,
            ),
            map(
                tuple((bchar('/'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::Divide,
            ),
            map(
                tuple((bchar('%'), nom::combinator::not(bchar('=')))),
                |_| BinaryOperator::Modulo,
            ),
        )))(i)
    })(input)
}
