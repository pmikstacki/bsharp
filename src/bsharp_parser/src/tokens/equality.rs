use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;
use syntax::expressions::BinaryOperator;

define_token_pair_str!(tok_equal, tok_peek_equal, "==");
define_token_pair_str!(tok_not_equal, tok_peek_not_equal, "!=");

pub fn parse_equality_op(input: Span) -> BResult<BinaryOperator> {
    alt((
        map(tok_equal(), |_| BinaryOperator::Equal),
        map(tok_not_equal(), |_| BinaryOperator::NotEqual),
    ))
    .parse(input.into())
}
