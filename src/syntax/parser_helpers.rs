use nom::{
    Parser,
    character::complete::char,
    combinator::{opt, verify},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::delimited,
};
use nom_supreme::tag::complete::tag;

use nom_supreme::ParserExt;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws as ws_comments;

/// Parse a keyword with proper word boundaries and enhanced error reporting
pub fn keyword(kw: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| {
        verify(
            tag(kw),
            |_: &str| is_word_boundary_after(input, kw.len())
        )
        .context("keyword")
        .parse(input)
    }
}

/// Parse a single character with enhanced error reporting (wrapper for nom's char)
pub fn nom_char(c: char) -> impl Fn(&str) -> BResult<&str, char> {
    move |input: &str| {
        char(c).parse(input)
    }
}

/// Parse a single character with enhanced error reporting
pub fn bchar(c: char) -> impl Fn(&str) -> BResult<&str, char> {
    move |input: &str| {
        char(c)
        .context("character")
        .parse(input)
    }
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
    move |input: &'a str| {
        delimited(&mut open, &mut inner, &mut close)(input)
    }
}

/// Enhanced optional syntax with context
pub fn bopt<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Option<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        opt(&mut parser)(input)
    }
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
    move |input: &'a str| {
        separated_list0(&mut sep, &mut element)(input)
    }
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
    move |input: &'a str| {
        separated_list1(&mut sep, &mut element)(input)
    }
}

/// Enhanced many0 syntax with context
pub fn bmany0<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        many0(&mut parser)(input)
    }
}

/// Enhanced many1 syntax with context
pub fn bmany1<'a, F, O>(mut parser: F) -> impl FnMut(&'a str) -> BResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> BResult<&'a str, O>,
{
    move |input: &'a str| {
        many1(&mut parser)(input)
    }
}

/// Tag syntax with enhanced error reporting
pub fn btag(tag_str: &'static str) -> impl Fn(&str) -> BResult<&str, &str> {
    move |input: &str| {
        tag(tag_str)
        .context("tag")
        .parse(input)
    }
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
                use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
                let error_tree = ErrorTree::Base {
                    location: input,
                    kind: BaseErrorKind::Expected(Expectation::Something),
                };
                Err(nom::Err::Error(error_tree))
            }
            Err(nom::Err::Failure(_e)) => {
                // Convert nom failure to nom-supreme failure
                use nom_supreme::error::{ErrorTree, BaseErrorKind, Expectation};
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