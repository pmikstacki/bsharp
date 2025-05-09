use crate::parser::errors::BResult;
use crate::parser::parser_helpers::{bs_context, nom_to_bs};
use nom::bytes::complete::{take_until, tag};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{recognize, map};
use nom::sequence::{tuple, delimited, preceded};
use nom::branch::alt;
use nom::multi::many0;

/// Parse a C-style block comment /* ... */
pub fn parse_block_comment(input: &str) -> BResult<&str, &str> {
    bs_context(
        "block comment",
        map(
            recognize(tuple((
                tag("/*"),
                take_until("*/"),
                tag("*/")
            ))),
            |s| s
        )
    )(input)
}

/// Parse a C# line comment // ...
pub fn parse_line_comment(input: &str) -> BResult<&str, &str> {
    bs_context(
        "line comment",
        recognize(tuple((
            tag("//"),
            take_until("\n"),
            alt((tag("\n"), recognize(multispace0)))
        )))
    )(input)
}

/// Parse any whitespace including comments, returns the consumed whitespace string
pub fn parse_whitespace_or_comments(input: &str) -> BResult<&str, &str> {
    bs_context(
        "whitespace or comments",
        recognize(many0(alt((
            multispace1,
            parse_block_comment,
            parse_line_comment
        ))))
    )(input)
}

/// Parses optional whitespace and comments, returns the consumed string
pub fn ws(input: &str) -> BResult<&str, &str> {
    parse_whitespace_or_comments(input)
}

/// Wraps a parser with whitespace and comment handling
pub fn with_ws<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    nom_to_bs(delimited(ws, nom_to_bs(parser), ws))
}

/// Precedes a parser with whitespace and comment handling
pub fn preceded_ws<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    nom_to_bs(preceded(ws, nom_to_bs(parser)))
}
