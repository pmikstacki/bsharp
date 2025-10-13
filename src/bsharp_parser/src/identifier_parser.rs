use crate::parser::keywords::is_keyword;
use crate::syntax::errors::BResult;
use crate::syntax::comment_parser::ws;
use nom_supreme::ParserExt;
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char as nom_char},
    combinator::{map_opt, recognize},
    multi::many0,
    sequence::{pair, preceded},
    Parser,
};
use syntax::Identifier;

// Parse a C# identifier (letters, digits, underscore, but must start with letter or underscore)
pub fn parse_identifier<'a>(input: Span<'a>) -> BResult<'a, Identifier> {
    (|input| {
        // C# identifiers can start with a letter or underscore, followed by letters, digits, or underscores.
        let identifier_start = alt((alpha1, recognize(nom_char('_'))));
        let identifier_chars = recognize(pair(
            identifier_start,
            many0(alt((alphanumeric1, recognize(nom_char('_'))))),
        ));
        map_opt(
            nom::sequence::delimited(ws, identifier_chars, ws),
            |span: Span<'a>| {
                let s = span.fragment();
                if !is_keyword(s) {
                    Some(Identifier { name: s.to_string() })
                } else {
                    None
                }
            },
        )
        .parse(input)
    })
    .context("identifier")
    .parse(input)
}

// Parse a qualified name (e.g., System.Collections.Generic)
pub fn parse_qualified_name<'a>(input: Span<'a>) -> BResult<'a, Vec<Identifier>> {
    (|input| {
        use nom::sequence::delimited;
        let dot = delimited(ws, nom_char('.'), ws);
        let identifier = parse_identifier;
        let (rest, first) = identifier(input)?;
        let (rest, others) = many0(preceded(dot, identifier)).parse(rest)?;
        let mut result = vec![first];
        result.extend(others);
        Ok((rest, result))
    })
    .context("qualified name")
    .parse(input)
}
use crate::syntax::span::Span;
