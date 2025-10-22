use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;
use nom::Parser;
use nom::combinator::{cut, opt, peek};
use nom::multi::{many0, separated_list0};
use nom::sequence::delimited;

/// Result of parsing either a singleton value or a delimited list of values
#[derive(Debug, Clone, PartialEq)]
pub enum OneOrMany<T> {
    Single(T),
    Many(Vec<T>),
}

/// Generic helper to parse a delimited list using first-element disambiguation.
/// It consumes `open`, parses a first element with `first`, then peeks `sep` to
/// decide if more elements follow. If yes, it parses the rest separated by `sep`,
/// optionally a trailing separator, and then consumes `close` (guarded with cut).
/// If not, it consumes `close` (no cut) and returns Single(first).
pub fn parse_delimited_list_or_singleton<'a, FO, FF, FS, FR, FC, OOpen, OSep, OClose, T>(
    mut open: FO,
    mut first: FF,
    mut sep: FS,
    mut rest_elem: FR,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close_on_many: bool,
) -> impl FnMut(Span<'a>) -> BResult<'a, OneOrMany<T>>
where
    FO: FnMut(Span<'a>) -> BResult<'a, OOpen>,
    FF: FnMut(Span<'a>) -> BResult<'a, T>,
    FS: FnMut(Span<'a>) -> BResult<'a, OSep>,
    FR: FnMut(Span<'a>) -> BResult<'a, T>,
    FC: FnMut(Span<'a>) -> BResult<'a, OClose>,
{
    move |input: Span<'a>| {
        // Open delimiter (with whitespace/comments)
        let (input, _) = delimited(ws, &mut open, ws).parse(input)?;

        // Parse the first element
        let (input, first_val) = first(input)?;

        // Disambiguate by peeking the closing delimiter.
        // If close is next -> singleton; otherwise it's a list (expect a separator and parse rest).
        if peek(delimited(ws, &mut close, ws)).parse(input).is_ok() {
            let (input, _) = cut(delimited(ws, &mut close, ws)).parse(input)?;
            Ok((input, OneOrMany::Single(first_val)))
        } else {
            // List path: require a separator, then parse the remaining elements
            let (input, _) = delimited(ws, &mut sep, ws).parse(input)?;
            let (input, mut rest) = separated_list0(
                delimited(ws, &mut sep, ws),
                delimited(ws, &mut rest_elem, ws),
            )
            .parse(input)?;

            // Optional trailing separator
            let (input, _) = if allow_trailing_sep {
                opt(delimited(ws, &mut sep, ws)).parse(input)?
            } else {
                (input, None)
            };

            // Close (guard with cut when committed to list)
            let (input, _) = if cut_close_on_many {
                cut(delimited(ws, &mut close, ws)).parse(input)?
            } else {
                delimited(ws, &mut close, ws).parse(input)?
            };

            rest.insert(0, first_val);
            Ok((input, OneOrMany::Many(rest)))
        }
    }
}

/// Parse zero-or-more elements between delimiters, with optional trailing separator and optional cut on close.
pub fn parse_delimited_list0<'a, FO, FE, FS, FC, OOpen, OSep, OClose, T>(
    mut open: FO,
    mut elem: FE,
    mut sep: FS,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close: bool,
) -> impl FnMut(Span<'a>) -> BResult<'a, Vec<T>>
where
    FO: FnMut(Span<'a>) -> BResult<'a, OOpen>,
    FE: FnMut(Span<'a>) -> BResult<'a, T>,
    FS: FnMut(Span<'a>) -> BResult<'a, OSep>,
    FC: FnMut(Span<'a>) -> BResult<'a, OClose>,
{
    move |input: Span<'a>| {
        let (input, _) = delimited(ws, &mut open, ws).parse(input)?;

        // Empty list if immediately closed
        if peek(delimited(ws, &mut close, ws)).parse(input).is_ok() {
            let (input, _) = if cut_close {
                cut(delimited(ws, &mut close, ws)).parse(input)?
            } else {
                delimited(ws, &mut close, ws).parse(input)?
            };
            return Ok((input, Vec::new()));
        }

        // Parse first element, then the rest
        let (input, first) = delimited(ws, &mut elem, ws).parse(input)?;
        let (input, mut rest) = many0(|i| {
            let (i, _) = delimited(ws, &mut sep, ws).parse(i)?;
            delimited(ws, &mut elem, ws).parse(i)
        })
        .parse(input)?;

        // Optional trailing separator
        let (input, _) = if allow_trailing_sep {
            opt(delimited(ws, &mut sep, ws)).parse(input)?
        } else {
            (input, None)
        };

        // Close
        let (input, _) = if cut_close {
            cut(delimited(ws, &mut close, ws)).parse(input)?
        } else {
            delimited(ws, &mut close, ws).parse(input)?
        };

        rest.insert(0, first);
        Ok((input, rest))
    }
}

/// Parse one-or-more elements between delimiters, with optional trailing separator and optional cut on close.
pub fn parse_delimited_list1<'a, FO, FE, FS, FC, OOpen, OSep, OClose, T>(
    mut open: FO,
    mut elem: FE,
    mut sep: FS,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close: bool,
) -> impl FnMut(Span<'a>) -> BResult<'a, Vec<T>>
where
    FO: FnMut(Span<'a>) -> BResult<'a, OOpen>,
    FE: FnMut(Span<'a>) -> BResult<'a, T>,
    FS: FnMut(Span<'a>) -> BResult<'a, OSep>,
    FC: FnMut(Span<'a>) -> BResult<'a, OClose>,
{
    move |input: Span<'a>| {
        let (input, _) = delimited(ws, &mut open, ws).parse(input)?;

        // Require first element
        let (input, first) = delimited(ws, &mut elem, ws).parse(input)?;
        // Parse zero or more pairs of (separator then element)
        let (input, mut rest) = many0(|i| {
            let (i, _) = delimited(ws, &mut sep, ws).parse(i)?;
            delimited(ws, &mut elem, ws).parse(i)
        })
        .parse(input)?;
        // Optional trailing separator
        let (input, _) = if allow_trailing_sep {
            opt(delimited(ws, &mut sep, ws)).parse(input)?
        } else {
            (input, None)
        };
        let (input, _) = if cut_close {
            cut(delimited(ws, &mut close, ws)).parse(input)?
        } else {
            delimited(ws, &mut close, ws).parse(input)?
        };
        rest.insert(0, first);
        Ok((input, rest))
    }
}

/// Parse a non-delimited list of zero-or-more elements separated by a separator.
/// Whitespace/comments are handled via bws for both element and separator.
pub fn parse_list0<'a, FE, FS, OSep, T>(
    mut elem: FE,
    mut sep: FS,
) -> impl FnMut(Span<'a>) -> BResult<'a, Vec<T>>
where
    FE: FnMut(Span<'a>) -> BResult<'a, T>,
    FS: FnMut(Span<'a>) -> BResult<'a, OSep>,
{
    move |input: Span<'a>| {
        separated_list0(delimited(ws, &mut sep, ws), delimited(ws, &mut elem, ws)).parse(input)
    }
}
