use crate::parser::keywords::preprocessor_keywords::{kw_line, kw_pragma};
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{line_ending, space0};
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use syntax::trivia::preprocessor::PreprocessorDirective;

/// Parse a preprocessor directive line starting with '#'
/// Supports: #pragma <rest> and #line <rest> (stored as strings)
/// Consumes the directive and its trailing newline (if any)
pub fn parse_preprocessor_directive(input: &str) -> BResult<&str, PreprocessorDirective> {
    context("preprocessor directive", |i| {
        let (i, _) = bws(bchar('#'))(i)?;
        let (i, directive) = alt((
            // #pragma ...
            map(
                tuple((kw_pragma(), space0, opt(is_not("\r\n")))),
                |(_, _, rest_of_line)| PreprocessorDirective::Pragma {
                    pragma: rest_of_line.unwrap_or("").trim().to_string(),
                },
            ),
            // #line ...
            map(
                tuple((kw_line(), space0, opt(is_not("\r\n")))),
                |(_, _, line_rest)| PreprocessorDirective::Line {
                    line: line_rest.unwrap_or("").trim().to_string(),
                },
            ),
            // Fallback: any other directive -> Unknown { text }
            map(opt(is_not("\r\n")), |rest: Option<&str>| {
                PreprocessorDirective::Unknown {
                    text: rest.unwrap_or("").trim().to_string(),
                }
            }),
        ))(i)?;

        // Consume optional trailing newline to fully skip the directive line
        let (i, _) = opt(line_ending)(i)?;

        Ok((i, directive))
    })(input)
}
