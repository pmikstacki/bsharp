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
    use crate::parser::keywords::preprocessor_keywords::*;
    use crate::parser::identifier_parser::parse_identifier;
    use nom::combinator::cut;

    context("preprocessor directive", |i| {
        let (i, _) = bws(bchar('#'))(i)?;
        let (i, directive) = alt((
            // #define SYMBOL
            map(
                tuple((kw_define(), space0, cut(bws(parse_identifier)))),
                |(_, _, sym)| PreprocessorDirective::Define { symbol: sym },
            ),
            // #undef SYMBOL
            map(
                tuple((kw_undef(), space0, cut(bws(parse_identifier)))),
                |(_, _, sym)| PreprocessorDirective::Undef { symbol: sym },
            ),
            // #if CONDITION (rest of line)
            map(tuple((kw_if(), space0, opt(is_not("\r\n")))), |(_, _, cond)| {
                PreprocessorDirective::If {
                    condition: cond.unwrap_or("").trim().to_string(),
                }
            }),
            // #elif CONDITION (rest of line)
            map(tuple((kw_elif(), space0, opt(is_not("\r\n")))), |(_, _, cond)| {
                PreprocessorDirective::Elif {
                    condition: cond.unwrap_or("").trim().to_string(),
                }
            }),
            // #else
            map(kw_else(), |_| PreprocessorDirective::Else),
            // #endif
            map(kw_endif(), |_| PreprocessorDirective::Endif),
            // #region [name]
            map(
                tuple((kw_region(), space0, opt(is_not("\r\n")))),
                |(_, _, name)| PreprocessorDirective::Region {
                    name: name.map(|s| s.trim().to_string()),
                },
            ),
            // #endregion
            map(kw_endregion(), |_| PreprocessorDirective::EndRegion),
            // #error message
            map(
                tuple((kw_error(), space0, opt(is_not("\r\n")))),
                |(_, _, msg)| PreprocessorDirective::Error {
                    message: msg.unwrap_or("").trim().to_string(),
                },
            ),
            // #warning message
            map(
                tuple((kw_warning(), space0, opt(is_not("\r\n")))),
                |(_, _, msg)| PreprocessorDirective::Warning {
                    message: msg.unwrap_or("").trim().to_string(),
                },
            ),
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
