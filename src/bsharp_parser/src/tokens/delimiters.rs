use crate::errors::BResult;
use syntax::span::Span;

use nom::Parser;
use nom::character::complete::char as nom_char;

define_token_pair_chr!(tok_l_paren, tok_peek_l_paren, '(');
define_token_pair_chr!(tok_r_paren, tok_peek_r_paren, ')');
define_token_pair_chr!(tok_l_brack, tok_peek_l_brack, '[');
define_token_pair_chr!(tok_r_brack, tok_peek_r_brack, ']');

pub fn tok_l_brace() -> impl FnMut(Span) -> BResult<char> {
    move |i: Span| {
        let (rest, ch) = nom_char('{')(i)?;
        crate::helpers::brace_tracker::on_char(i, '{');
        Ok((rest, ch))
    }
}

pub fn tok_peek_l_brace() -> impl FnMut(Span) -> BResult<char> {
    move |input: Span| nom::combinator::peek(tok_l_brace()).parse(input)
}

pub fn tok_r_brace() -> impl FnMut(Span) -> BResult<char> {
    move |i: Span| {
        let (rest, ch) = nom_char('}')(i)?;
        crate::helpers::brace_tracker::on_char(i, '}');
        Ok((rest, ch))
    }
}

pub fn tok_peek_r_brace() -> impl FnMut(Span) -> BResult<char> {
    move |input: Span| nom::combinator::peek(tok_r_brace()).parse(input)
}
