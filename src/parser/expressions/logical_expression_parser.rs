use crate::parser::expressions::precedence::left_chain;
use crate::parser::expressions::range_expression_parser::parse_range_expression_or_higher;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::expressions::{BinaryOperator, Expression};
use crate::syntax::parser_helpers::{bchar, bws};
use nom::branch::alt;
use nom::combinator::{map, recognize};
use nom::sequence::{pair, tuple};

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
    left_chain(parse_relational_expression_or_higher, |i| {
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
        bws(alt((
            map(recognize(pair(bchar('<'), bchar('='))), |_| {
                BinaryOperator::LessEqual
            }),
            map(recognize(pair(bchar('>'), bchar('='))), |_| {
                BinaryOperator::GreaterEqual
            }),
            map(bchar('<'), |_| BinaryOperator::LessThan),
            map(bchar('>'), |_| BinaryOperator::GreaterThan),
        )))(i)
    })(input)
}

fn parse_shift_expression_or_higher(input: &str) -> BResult<&str, Expression> {
    left_chain(parse_additive_expression_or_higher, |i| {
        bws(alt((
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
