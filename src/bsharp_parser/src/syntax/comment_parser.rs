use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::Parser;
use nom_supreme::ParserExt;

/// Parse a C-style block comment /* ... */ and return the recognized Span
pub fn parse_block_comment(input: Span) -> BResult<Span> {
    recognize((tag("/*"), take_until("*/"), tag("*/"))).parse(input.into())
}

/// Parse a C# line comment // ... and return the recognized Span
pub fn parse_line_comment(input: Span) -> BResult<Span> {
    recognize((
        tag("//"),
        take_until("\n"),
        alt((tag("\n"), recognize(multispace0))),
    ))
        .context("line comment")
        .parse(input.into())
}

/// Parse any whitespace including comments, returns the consumed whitespace string
pub fn parse_whitespace_or_comments<'a>(input: Span<'a>) -> BResult<'a, &str> {
    map(
        recognize(many0(alt((
            recognize(multispace1),
            parse_block_comment,
            parse_line_comment,
        )))),
        |matched: Span<'a>| {
            let m = matched.fragment();
            if m.len() == input.fragment().len() { "" } else { m }
        },
    )
        .context("whitespace or comments")
        .parse(input.into())
}

/// Parses optional whitespace and comments, returns the consumed string
pub fn ws(input: Span) -> BResult<&str> {
    match parse_whitespace_or_comments(input.into()) {
        Ok((rest, _matched)) if rest.fragment().len() == input.fragment().len() => Ok((rest, "")),
        other => other,
    }
}