use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::context;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};

/// Parse a C-style block comment /* ... */
pub fn parse_block_comment(input: &str) -> BResult<&str, &str> {
    context(
        "block comment",
        map(
            recognize(tuple((tag("/*"), take_until("*/"), tag("*/")))),
            |s| s,
        ),
    )(input)
}

/// Parse a C# line comment // ...
pub fn parse_line_comment(input: &str) -> BResult<&str, &str> {
    context(
        "line comment",
        recognize(tuple((
            tag("//"),
            take_until("\n"),
            alt((tag("\n"), recognize(multispace0))),
        ))),
    )(input)
}

/// Parse any whitespace including comments, returns the consumed whitespace string
pub fn parse_whitespace_or_comments(input: &str) -> BResult<&str, &str> {
    context(
        "whitespace or comments",
        map(
            recognize(many0(alt((
                multispace1,
                parse_block_comment,
                parse_line_comment,
            )))),
            |matched: &str| {
                // When nothing matches, `recognize(many0(...))` returns the original slice.
                // For callers relying on progress (like top-level statement loops), normalize
                // this to the empty string so the input state reflects zero-width consumption.
                if matched.len() == input.len() {
                    ""
                } else {
                    matched
                }
            },
        ),
    )(input)
}

/// Parses optional whitespace and comments, returns the consumed string
pub fn ws(input: &str) -> BResult<&str, &str> {
    match parse_whitespace_or_comments(input) {
        Ok((rest, _matched)) if rest.len() == input.len() => Ok((rest, "")),
        other => other,
    }
}

/// Wraps a syntax with whitespace and comment handling
pub fn with_ws<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    delimited(ws, parser, ws)
}

/// Precedes a syntax with whitespace and comment handling
pub fn preceded_ws<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    preceded(ws, parser)
}
