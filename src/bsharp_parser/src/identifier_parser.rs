use crate::parser::keywords::is_keyword;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::{
    Parser,
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char as nom_char},
    combinator::{map_opt, recognize},
    multi::many0,
    sequence::{pair, preceded},
};
use syntax::Identifier;
use crate::span::Spanned;
use crate::span_ext::ParserExt as _;

// Parse a C# identifier (letters, digits, underscore, but must start with letter or underscore)
pub fn parse_identifier(input: Span<'_>) -> BResult<'_, Identifier> {
    (|input| {
        // C# identifiers can start with a letter or underscore, followed by letters, digits, or underscores.
        let identifier_start = alt((alpha1, recognize(nom_char('_'))));
        let identifier_chars = recognize(pair(
            identifier_start,
            many0(alt((alphanumeric1, recognize(nom_char('_'))))),
        ));
        map_opt(
            nom::sequence::delimited(ws, identifier_chars, ws),
            |span: Span<'_>| {
                let s = span.fragment();
                if !is_keyword(s) {
                    Some(Identifier::Simple(s.to_string()))
                } else {
                    None
                }
            },
        )
        .parse(input)
    })
    .parse(input)
}

// Parse a qualified name (e.g., System.Collections.Generic) into Identifier segments
pub fn parse_qualified_name(input: Span) -> BResult<Vec<Identifier>> {
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
    .parse(input)
}

pub fn parse_identifier_spanned(input: Span<'_>) -> BResult<'_, Spanned<Identifier>> {
    (|input| {
        let identifier_start = alt((alpha1, recognize(nom_char('_'))));
        let identifier_chars = recognize(pair(
            identifier_start,
            many0(alt((alphanumeric1, recognize(nom_char('_'))))),
        ));
        let core_spanned = identifier_chars.spanned();
        nom::combinator::map_opt(
            nom::sequence::delimited(ws, core_spanned, ws),
            |s: Spanned<Span<'_>>| {
                let frag = s.node.fragment();
                if !is_keyword(frag) {
                    Some(s.map(|span| Identifier::Simple(span.fragment().to_string())))
                } else {
                    None
                }
            },
        )
        .parse(input)
    })
    .parse(input)
}
use syntax::span::Span;

