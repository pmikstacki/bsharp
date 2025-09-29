use nom::{
    character::complete::char,
    combinator::{opt, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::delimited,
    Parser,
};
use nom_supreme::tag::complete::tag;

use crate::syntax::comment_parser::ws as ws_comments;
use crate::syntax::errors::BResult;
use nom_supreme::ParserExt;
use nom::Offset;
use nom::combinator::{cut, peek};

/// Parse a keyword with proper word boundaries and enhanced error reporting
pub fn keyword(kw: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| {
        verify(tag(kw), |_: &str| is_word_boundary_after(input, kw.len()))
            .context("keyword")
            .parse(input)
    }
}

/// Parse a single character with enhanced error reporting (wrapper for nom's char)
pub fn nom_char(c: char) -> impl Fn(&str) -> BResult<&str, char> {
    move |input: &str| char(c).parse(input)
}

/// Parse a single character with enhanced error reporting
pub fn bchar(c: char) -> impl Fn(&str) -> BResult<&str, char> {
    move |input: &str| char(c).context("character").parse(input)
}

/// Context function for adding error context
pub fn context<'a, F, O>(
    ctx: &'static str,
    mut parser: F,
) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        parser(input).map_err(|err| {
            // Add context to the error using nom-supreme's error tree
            match err {
                nom::Err::Error(e) => {
                    use nom_supreme::error::{ErrorTree, StackContext};
                    let error_with_context = ErrorTree::Stack {
                        base: Box::new(e),
                        contexts: vec![(input, StackContext::Context(ctx))],
                    };
                    nom::Err::Error(error_with_context)
                }
                nom::Err::Failure(e) => {
                    use nom_supreme::error::{ErrorTree, StackContext};
                    let error_with_context = ErrorTree::Stack {
                        base: Box::new(e),
                        contexts: vec![(input, StackContext::Context(ctx))],
                    };
                    nom::Err::Failure(error_with_context)
                }
                nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
            }
        })
    }
}

/// Helper for optional whitespace (wrapper around nom's multispace0)
pub fn bws<'a, F, O>(mut inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        let (input, _) = ws_comments(input)?;
        let (input, result) = inner(input)?;
        let (input, _) = ws_comments(input)?;
        Ok((input, result))
    }
}

/// Enhanced delimited syntax with context
pub fn bdelimited<'a, F, G, H, O1, O2, O3>(
    mut open: F,
    mut inner: G,
    mut close: H,
) -> impl FnMut(&'a str) -> BResult<&'a str, O2>
where
    F: FnMut(&'a str) -> BResult<&'a str, O1>,
    G: FnMut(&'a str) -> BResult<&'a str, O2>,
    H: FnMut(&'a str) -> BResult<&'a str, O3>,
{
    move |input: &'a str| delimited(&mut open, &mut inner, &mut close)(input)
}

/// Enhanced optional syntax with context
pub fn bopt<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Option<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| opt(&mut parser)(input)
}

/// Enhanced separated_list0 syntax with context
pub fn bseparated_list0<'a, F, G, O1, O2>(
    mut sep: F,
    mut element: G,
) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O2>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O1>,
    G: FnMut(&'a str) -> BResult<&'a str, O2>,
{
    move |input: &'a str| separated_list0(&mut sep, &mut element)(input)
}

/// Enhanced separated_list1 syntax with context
pub fn bseparated_list1<'a, F, G, O1, O2>(
    mut sep: F,
    mut element: G,
) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O2>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O1>,
    G: FnMut(&'a str) -> BResult<&'a str, O2>,
{
    move |input: &'a str| separated_list1(&mut sep, &mut element)(input)
}

/// Enhanced many0 syntax with context
pub fn bmany0<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| many0(&mut parser)(input)
}

/// Enhanced many1 syntax with context
pub fn bmany1<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| many1(&mut parser)(input)
}

/// Tag syntax with enhanced error reporting
pub fn btag(tag_str: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| tag(tag_str).context("tag").parse(input)
}

/// Peek a parser through whitespace/comments without consuming input
pub fn bpeek<'a, F, O>(mut inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        // Wrap the inner parser with bws so peek sees past comments/whitespace
        peek(bws(&mut inner))(input)
    }
}

/// Peek for a specific character without consuming input (whitespace/comment aware)
pub fn peek_bchar(c: char) -> impl Fn(&str) -> BResult<&str, char> {
    move |input: &str| bpeek(bchar(c))(input)
}

/// Peek for a keyword with word-boundary handling without consuming input (whitespace/comment aware)
pub fn peek_keyword(kw: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| bpeek(keyword(kw))(input)
}

/// Peek for an exact tag without consuming input (whitespace/comment aware)
pub fn peek_tag(tag_str: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| bpeek(btag(tag_str))(input)
}

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
pub fn parse_delimited_list_or_singleton<'a, FO, FF, FS, FR, FC, OOpen, OFirst, OSep, ORest, OClose, T>(
    mut open: FO,
    mut first: FF,
    mut sep: FS,
    mut rest_elem: FR,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close_on_many: bool,
) -> impl FnMut(&'a str) -> BResult<&'a str, OneOrMany<T>>
where
    FO: FnMut(&'a str) -> BResult<&'a str, OOpen>,
    FF: FnMut(&'a str) -> BResult<&'a str, T>,
    FS: FnMut(&'a str) -> BResult<&'a str, OSep>,
    FR: FnMut(&'a str) -> BResult<&'a str, T>,
    FC: FnMut(&'a str) -> BResult<&'a str, OClose>,
{
    move |input: &'a str| {
        // Open delimiter (with whitespace/comments)
        let (input, _) = bws(&mut open)(input)?;

        // Parse the first element
        let (input, first_val) = first(input)?;

        // Disambiguate by peeking the closing delimiter.
        // If close is next -> singleton; otherwise it's a list (expect a separator and parse rest).
        if bpeek(&mut close)(input).is_ok() {
            // Singleton path: close without cut
            // Once we've seen 'open' and successfully parsed a first element,
            // commit to the closing delimiter as part of a parenthesized singleton.
            // Guard the close with cut to avoid misleading backtracking.
            let (input, _) = cut(bws(&mut close))(input)?;
            Ok((input, OneOrMany::Single(first_val)))
        } else {
            // List path: require a separator, then parse the remaining elements
            let (input, _) = bws(&mut sep)(input)?;
            let (input, mut rest) = separated_list0(bws(&mut sep), bws(&mut rest_elem))(input)?;

            // Optional trailing separator
            let (input, _) = if allow_trailing_sep {
                opt(bws(&mut sep))(input)?
            } else {
                (input, None)
            };

            // Close (guard with cut when committed to list)
            let (input, _) = if cut_close_on_many {
                cut(bws(&mut close))(input)?
            } else {
                bws(&mut close)(input)?
            };

            // Build full list with the first element at the front
            rest.insert(0, first_val);
            Ok((input, OneOrMany::Many(rest)))
        }
    }
}

/// Parse zero-or-more elements between delimiters, with optional trailing separator and optional cut on close.
pub fn parse_delimited_list0<'a, FO, FE, FS, FC, OOpen, OElem, OSep, OClose, T>(
    mut open: FO,
    mut elem: FE,
    mut sep: FS,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close: bool,
) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<T>>
where
    FO: FnMut(&'a str) -> BResult<&'a str, OOpen>,
    FE: FnMut(&'a str) -> BResult<&'a str, T>,
    FS: FnMut(&'a str) -> BResult<&'a str, OSep>,
    FC: FnMut(&'a str) -> BResult<&'a str, OClose>,
{
    move |input: &'a str| {
        let (input, _) = bws(&mut open)(input)?;

        // Empty list if immediately closed
        if bpeek(&mut close)(input).is_ok() {
            let (input, _) = if cut_close { cut(bws(&mut close))(input)? } else { bws(&mut close)(input)? };
            return Ok((input, Vec::new()));
        }

        // Parse first element, then the rest
        let (input, first) = bws(&mut elem)(input)?;
        let (input, mut rest) = many0(|i| {
            let (i, _) = bws(&mut sep)(i)?;
            bws(&mut elem)(i)
        })(input)?;

        // Optional trailing separator
        let (input, _) = if allow_trailing_sep { opt(bws(&mut sep))(input)? } else { (input, None) };

        // Close
        let (input, _) = if cut_close { cut(bws(&mut close))(input)? } else { bws(&mut close)(input)? };

        rest.insert(0, first);
        Ok((input, rest))
    }
}

/// Parse one-or-more elements between delimiters, with optional trailing separator and optional cut on close.
pub fn parse_delimited_list1<'a, FO, FE, FS, FC, OOpen, OElem, OSep, OClose, T>(
    mut open: FO,
    mut elem: FE,
    mut sep: FS,
    mut close: FC,
    allow_trailing_sep: bool,
    cut_close: bool,
) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<T>>
where
    FO: FnMut(&'a str) -> BResult<&'a str, OOpen>,
    FE: FnMut(&'a str) -> BResult<&'a str, T>,
    FS: FnMut(&'a str) -> BResult<&'a str, OSep>,
    FC: FnMut(&'a str) -> BResult<&'a str, OClose>,
{
    move |input: &'a str| {
        let (input, _) = bws(&mut open)(input)?;

        // Require first element
        let (input, first) = bws(&mut elem)(input)?;
        // Parse zero or more pairs of (separator then element)
        let (input, mut rest) = many0(|i| {
            let (i, _) = bws(&mut sep)(i)?;
            bws(&mut elem)(i)
        })(input)?;
        // Optional trailing separator
        let (input, _) = if allow_trailing_sep { opt(bws(&mut sep))(input)? } else { (input, None) };
        let (input, _) = if cut_close { cut(bws(&mut close))(input)? } else { bws(&mut close)(input)? };
        rest.insert(0, first);
        Ok((input, rest))
    }
}

/// Parse a non-delimited list of zero-or-more elements separated by a separator.
/// Whitespace/comments are handled via bws for both element and separator.
pub fn parse_list0<'a, FE, FS, OSep, T>(
    mut elem: FE,
    mut sep: FS,
) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<T>>
where
    FE: FnMut(&'a str) -> BResult<&'a str, T>,
    FS: FnMut(&'a str) -> BResult<&'a str, OSep>,
{
    move |input: &'a str| separated_list0(bws(&mut sep), bws(&mut elem))(input)
}

/// Convert nom syntax to BSharp syntax (adapter)
pub fn nom_to_bs<F, O>(mut parser: F) -> impl FnMut(&str) -> BResult<&str, O>
where
    F: FnMut(&str) -> nom::IResult<&str, O, nom::error::Error<&str>>,
{
    move |input: &str| {
        match parser(input) {
            Ok((remaining, output)) => Ok((remaining, output)),
            Err(nom::Err::Error(_e)) => {
                // Convert nom error to nom-supreme error
                use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Something),
                };
                Err(nom::Err::Error(error_tree))
            }
            Err(nom::Err::Failure(_e)) => {
                // Convert nom failure to nom-supreme failure
                use nom_supreme::error::{BaseErrorKind, ErrorTree, Expectation};
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Something),
                };
                Err(nom::Err::Failure(error_tree))
            }
            Err(nom::Err::Incomplete(needed)) => Err(nom::Err::Incomplete(needed)),
        }
    }
}

/// Helper function to check if we're at a word boundary
fn is_word_boundary_after(input: &str, keyword_len: usize) -> bool {
    if keyword_len >= input.len() {
        return true; // End of input is a word boundary
    }

    let next_char = input.chars().nth(keyword_len);
    match next_char {
        Some(c) => !c.is_alphanumeric() && c != '_',
        None => true,
    }
}

/// Wrap a parser and return its output paired with the exact byte span it consumed
/// in the original `whole` input slice. This uses nom-supreme's `.with_recognized()`
/// and nom's `Offset` trait, so no extra dependencies are required.
pub fn with_recognized_span<'a, F, O>(
    whole: &'a str,
    parser: F,
) -> impl FnMut(&'a str) -> BResult<&'a str, (O, std::ops::Range<usize>)>
where
    F: Clone + FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        // Clone the parser per-call since with_recognized() takes self by value
        let p = parser.clone();
        // Get (recognized_slice, parsed_output)
        let (rest, (recognized, out)) = p.with_recognized().parse(input)?;
        // Compute absolute byte offsets into the original source
        let start = whole.offset(recognized);
        let end = start + recognized.len();
        Ok((rest, (out, start..end)))
    }
}
