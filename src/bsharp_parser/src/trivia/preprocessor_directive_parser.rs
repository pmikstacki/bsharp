use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::{char as nom_char, line_ending, space0};
use nom::combinator::{map, opt};
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::trivia::preprocessor::PreprocessorDirective;

/// Parse a preprocessor directive line starting with '#'
/// Supports: #pragma <rest> and #line <rest> (stored as strings)
/// Consumes the directive and its trailing newline (if any)
pub fn parse_preprocessor_directive(input: Span) -> BResult<PreprocessorDirective> {
    use crate::parser::identifier_parser::parse_identifier;
    use crate::parser::keywords::preprocessor_keywords::*;
    use nom::combinator::cut;

    (|i| {
        let (i, _) = delimited(ws, nom_char('#'), ws).parse(i)?;
        let (i, directive) = cut(alt((
            // #define SYMBOL
            map(
                (kw_define(), space0, cut(delimited(ws, parse_identifier, ws))),
                |(_, _, sym)| PreprocessorDirective::Define { symbol: sym },
            ),
            // #undef SYMBOL
            map(
                (kw_undef(), space0, cut(delimited(ws, parse_identifier, ws))),
                |(_, _, sym)| PreprocessorDirective::Undef { symbol: sym },
            ),
            // #if CONDITION (rest of line)
            map(
                (kw_if(), space0, opt(is_not("\r\n"))),
                |(_, _, cond)| PreprocessorDirective::If {
                    condition: cond
                        .map(|s: Span| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // #elif CONDITION (rest of line)
            map(
                (kw_elif(), space0, opt(is_not("\r\n"))),
                |(_, _, cond)| PreprocessorDirective::Elif {
                    condition: cond
                        .map(|s| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // #else
            map(kw_else(), |_| PreprocessorDirective::Else),
            // #endif
            map(kw_endif(), |_| PreprocessorDirective::Endif),
            // #region [name]
            map(
                (kw_region(), space0, opt(is_not("\r\n"))),
                |(_, _, name)| PreprocessorDirective::Region {
                    name: name.map(|s: Span| (*s.fragment()).trim().to_string()),
                },
            ),
            // #endregion
            map(kw_endregion(), |_| PreprocessorDirective::EndRegion),
            // #error message
            map(
                (kw_error(), space0, opt(is_not("\r\n"))),
                |(_, _, msg)| PreprocessorDirective::Error {
                    message: msg
                        .map(|s: Span| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // #warning message
            map(
                (kw_warning(), space0, opt(is_not("\r\n"))),
                |(_, _, msg)| PreprocessorDirective::Warning {
                    message: msg
                        .map(|s| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // #pragma ...
            map(
                (kw_pragma(), space0, opt(is_not("\r\n"))),
                |(_, _, rest_of_line)| PreprocessorDirective::Pragma {
                    pragma: rest_of_line
                        .map(|s: Span| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // #line ...
            map(
                (kw_line(), space0, opt(is_not("\r\n"))),
                |(_, _, line_rest)| PreprocessorDirective::Line {
                    line: line_rest
                        .map(|s: Span| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                },
            ),
            // Fallback: any other directive -> Unknown { text }
            map(opt(is_not("\r\n")), |rest: Option<Span>| {
                PreprocessorDirective::Unknown {
                    text: rest
                        .map(|s: Span| *s.fragment())
                        .unwrap_or("")
                        .trim()
                        .to_string(),
                }
            }),
        )))
            .parse(i)?;

        // Consume optional trailing newline to fully skip the directive line
        let (i, _) = opt(line_ending).parse(i)?;

        Ok((i, directive))
    })
        .context("preprocessor directive")
        .parse(input)
}
use crate::syntax::span::Span;
