//! Helper utilities for nom parser error conversion to BSharpParseError
use crate::parser::errors::{BResult, BSharpParseError, CustomErrorKind};
use nom::error::Error as NomError;
use nom::{error::ParseError, Err, IResult, Parser};

use nom::branch;
use nom::bytes::complete as bytes_complete;
// Import nom modules we'll use directly
use nom::character::complete as char_complete;
use nom::sequence;
//use nom::combinator::value;

// TypeSyntax alias for standard nom error that we'll convert from
type StdNomError<I> = NomError<I>;

/// Primary helper to convert standard nom parsers that use nom::error::Error<I> to BSharpParseError<I>
/// Use this when wrapping basic nom combinators like `tag`, `char`, etc.
pub fn nom_to_bs<I, O, E, F>(mut parser: F) -> impl FnMut(I) -> BResult<I, O>
where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: Clone + std::fmt::Display,
    BSharpParseError<I>: From<E>,
{
    move |input: I| {
        parser(input).map_err(|e| match e {
            Err::Error(e) => Err::Error(BSharpParseError::from(e)),
            Err::Failure(e) => Err::Failure(BSharpParseError::from(e)),
            Err::Incomplete(n) => Err::Incomplete(n),
        })
    }
}

/// Helper for adding context to parser
pub fn bs_context<I, O, F>(
    ctx: &'static str,
    mut parser: F
) -> impl FnMut(I) -> BResult<I, O>
where
    F: FnMut(I) -> BResult<I, O>,
    I: Clone + std::fmt::Display,
{
    move |input: I| {
        let i = input.clone(); // Clone early, only once
        parser(input).map_err(|e| match e {
            Err::Error(err) => Err::Error(BSharpParseError::add_context_static(i.clone(), ctx, err)),
            Err::Failure(err) => Err::Failure(BSharpParseError::add_context_static(i.clone(), ctx, err)),
            Err::Incomplete(n) => Err::Incomplete(n),
        })
    }
}

/// Helper to make a standard nom parser (that returns IResult<I, O, nom::error::Error<I>>)
/// compatible with our parsers that use BSharpParseError. This is useful when you need to
/// pass a nom combinator to a parser that expects BResult.
pub fn wrap_std_parser<I, O, F>(mut parser: F) -> impl FnMut(I) -> BResult<I, O>
where
    F: Parser<I, O, NomError<I>>,
    I: Clone + std::fmt::Display,
{
    move |input| {
        parser.parse(input).map_err(|e| match e {
            Err::Error(e) => Err::Error(BSharpParseError::from_nom_error(e)),
            Err::Failure(e) => Err::Failure(BSharpParseError::from_nom_error(e)),
            Err::Incomplete(n) => Err::Incomplete(n),
        })
    }
}

// Add a From implementation to convert from standard nom errors
impl<I: Clone + std::fmt::Display> From<NomError<I>> for BSharpParseError<I> {
    fn from(e: NomError<I>) -> Self {
        BSharpParseError::from_nom_error(e)
    }
}

// TypeSyntax alias for parser result (just for reference/documentation)
type _BResult<I, O> = nom::IResult<I, O, BSharpParseError<I>>; // For reference

//----------------------------------------------------------------------
// Common wrappers for standard nom parsers with explicit error handling
//----------------------------------------------------------------------

/// Wrap tag with explicit error type
pub fn btag<'a>(t: &'a str) -> impl FnMut(&'a str) -> BResult<&'a str, &'a str> {
    move |i: &'a str| {
        bytes_complete::tag::<&'a str, &'a str, StdNomError<&'a str>>(t)(i)
            .map_err(|e| e.map(BSharpParseError::from_nom_error))
    }
}

/// Wrap char with explicit error type
pub fn bchar(c: char) -> impl FnMut(&str) -> BResult<&str, char> {
    move |i: &str| {
        char_complete::char::<&str, StdNomError<&str>>(c)(i)
            .map_err(|e| e.map(BSharpParseError::from_nom_error))
    }
}

/// Wrap opt with explicit error type
pub fn bopt<I: Clone, O, F>(mut f: F) -> impl FnMut(I) -> BResult<I, Option<O>>
where
    F: FnMut(I) -> BResult<I, O>,
    I: Clone + std::fmt::Display,
{
    move |i: I| {
        let i_clone = i.clone();
        match f(i) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(Err::Error(_)) => Ok((i_clone, None)),
            Err(e) => Err(e),
        }
    }
}

/// Wrap tuple with explicit error type
pub fn btuple<I, O, List>(l: List) -> impl FnMut(I) -> BResult<I, O>
where
    I: Clone + std::fmt::Display,
    List: sequence::Tuple<I, O, BSharpParseError<I>> + Clone,
{
    move |i: I| {
        sequence::tuple::<I, O, BSharpParseError<I>, List>(l.clone())(i)
    }
}

/// Wrap alt with explicit error type
pub fn balt<I, O, List>(l: List) -> impl FnMut(I) -> BResult<I, O>
where
    I: Clone + std::fmt::Display,
    List: branch::Alt<I, O, BSharpParseError<I>> + Clone,
{
    move |i: I| {
        branch::alt::<I, O, BSharpParseError<I>, List>(l.clone())(i)
    }
}

/// Maps the result of a parser.
pub fn bmap<I, O1, O2, F, G>(mut parser: F, mut map_fn: G) -> impl FnMut(I) -> BResult<I, O2>
where
    I: Clone + nom::InputLength,
    F: FnMut(I) -> BResult<I, O1>,
    G: FnMut(O1) -> O2,
    BSharpParseError<I>: nom::error::ParseError<I>,
{
    move |input: I| {
        match parser(input.clone()) {
            Ok((remaining, output)) => Ok((remaining, map_fn(output))),
            Err(e) => Err(e),
        }
    }
}

/// Returns a constant value if the parser succeeds.
pub fn bvalue<I, O1, O2, F>(val: O2, mut parser: F) -> impl FnMut(I) -> BResult<I, O2>
where
    I: Clone + nom::InputLength,
    O2: Clone, // The value to return must be Clone
    F: FnMut(I) -> BResult<I, O1>, // Require parser to be Clone
    BSharpParseError<I>: nom::error::ParseError<I>,
{
    move |input: I| {
        match parser(input.clone()) {
            Ok((remaining, _)) => Ok((remaining, val.clone())), // Clone the value
            Err(e) => Err(e),
        }
    }
}

/// Applies three parsers sequentially and collects the results in a tuple.
pub fn bdelimited<I, O1, O2, O3, F, G, H>(
    mut left: F,
    mut middle: G,
    mut right: H,
) -> impl FnMut(I) -> BResult<I, O2>
where
    I: Clone + std::fmt::Display,
    F: FnMut(I) -> BResult<I, O1>,
    G: FnMut(I) -> BResult<I, O2>,
    H: FnMut(I) -> BResult<I, O3>,
{
    move |i: I| {
        let (i, _) = left(i)?;
        let (i, o2) = middle(i)?;
        let (i, _) = right(i)?;
        Ok((i, o2))
    }
}

/// Wrap preceded with explicit error type
pub fn bpreceded<I, O1, O2, F, G>(
    mut first: F,
    mut second: G,
) -> impl FnMut(I) -> BResult<I, O2>
where
    I: Clone + std::fmt::Display,
    F: FnMut(I) -> BResult<I, O1>,
    G: FnMut(I) -> BResult<I, O2>,
{
    move |i: I| {
        let (i, _) = first(i)?;
        second(i)
    }
}

/// Wrap a parser with optional whitespace using bwhitespace_ornone
pub fn bws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> BResult<&'a str, O>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    bdelimited(bwhitespace_ornone, inner, bwhitespace_ornone)
}

/// Wrap separated_list0 with explicit error type
pub fn bseparated_list0<I, O, OSep, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> BResult<I, Vec<O>>
where
    I: Clone + nom::InputLength + std::fmt::Display + std::fmt::Debug, // Add Debug bound
    F: FnMut(I) -> BResult<I, O>,
    G: FnMut(I) -> BResult<I, OSep>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        match f(i.clone()) {
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
            Err(Err::Error(_)) => return Ok((i, res)), // Empty list is ok
            Err(e) => return Err(e),
        }

        loop {
            let len = i.input_len();
            match sep(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)), // No more separators is ok
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check
                    if i1.input_len() == len {
                        return Err(Err::Error(BSharpParseError::from_error_kind(
                            i1,
                            nom::error::ErrorKind::SeparatedList, // Or a custom kind
                        )));
                    }

                    match f(i1.clone()) {
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                        Err(Err::Error(e)) => {
                            println!("Separated list item failed: {:?}", e);
                            // Propagate the error from the item parser
                            return Err(Err::Error(e));
                        }
                        Err(e) => return Err(e), // Handles Failure/Incomplete
                    }
                }
            }
        }
    }
}

/// Wrap separated_list1 with explicit error type
pub fn bseparated_list1<I, O, OSep, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> BResult<I, Vec<O>>
where
    I: Clone + nom::InputLength + std::fmt::Display + std::fmt::Debug,
    F: FnMut(I) -> BResult<I, O>,
    G: FnMut(I) -> BResult<I, OSep>,
{
    move |input: I| {
        let mut results = Vec::new();
        let (input, first) = f(input)?;
        results.push(first);
        
        let mut current = input;
        loop {
            match sep(current.clone()) {
                Ok((after_sep, _)) => {
                    match f(after_sep) {
                        Ok((after_item, item)) => {
                            results.push(item);
                            current = after_item;
                        },
                        Err(_) => break,
                    }
                },
                Err(_) => break,
            }
        }
        
        Ok((current, results))
    }
}

/// Wrap multispace0 (0 or more whitespace chars) with explicit error type
pub fn bwhitespace_ornone(input: &str) -> BResult<&str, &str> {
    char_complete::multispace0::<&str, StdNomError<&str>>(input)
        .map_err(|e| e.map(BSharpParseError::from_nom_error))
}

/// Wrapper for nom's terminated combinator
pub fn bterminated<I, O1, O2, F, G>(mut first: F, mut second: G) -> impl FnMut(I) -> BResult<I, O1>
where
    I: Clone + nom::InputLength + std::fmt::Display + std::fmt::Debug,
    F: FnMut(I) -> BResult<I, O1>,
    G: FnMut(I) -> BResult<I, O2>,
    O1: std::fmt::Debug,
    O2: std::fmt::Debug,
{
    move |i: I| {
        let (i, o1) = first(i)?;
        let (i, _) = second(i)?;
        Ok((i, o1))
    }
}

/// Converts a standard nom Error<I> into our BSharpParseError<I>.
/// Useful for adapting parsers returning the standard nom::IResult.
pub fn make_generic_nom_err<I: Clone + std::fmt::Display>(e: NomError<I>) -> BSharpParseError<I> {
    BSharpParseError::new(e.input, CustomErrorKind::Nom(e.code))
}

/// Parses a keyword. It matches the given keyword string.
/// It's intended to be used with `bws` which handles surrounding whitespace
/// and error conversion.
pub fn keyword<'a>(kw: &'static str) -> impl FnMut(&'a str) -> BResult<&'a str, &'a str> {
    nom_to_bs(bytes_complete::tag::<&'static str, &'a str, StdNomError<&'a str>>(kw))
}
