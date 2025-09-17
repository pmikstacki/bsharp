use crate::syntax::errors::BResult;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bws, context};
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char as nom_char},
    combinator::{map, recognize, verify},
    multi::many0,
    sequence::{pair, preceded},
};

// Parse a C# identifier (letters, digits, underscore, but must start with letter or underscore)
pub fn parse_identifier(input: &str) -> BResult<&str, Identifier> {
    context(
        "identifier (expected valid C# identifier starting with letter or underscore)",
        |input| {
            // C# identifiers can start with a letter or underscore, followed by
            // letters, digits, or underscores. Unicode support would be more complex.
            let identifier_start = alt((alpha1, recognize(nom_char('_'))));

            let identifier_chars = recognize(pair(
                identifier_start,
                many0(alt((alphanumeric1, recognize(nom_char('_'))))),
            ));

            // Verify the identifier is not a reserved keyword
            let identifier_parser = verify(identifier_chars, |s: &str| !is_keyword(s));

            // Map to the IdentifierNameSyntax struct
            map(bws(identifier_parser), |name: &str| Identifier {
                name: name.to_string(),
            })(input)
        },
    )(input)
}

// Function to check if a string is a C# keyword
fn is_keyword(word: &str) -> bool {
    crate::syntax::keywords::KEYWORDS.contains(&word)
}

// Parse a qualified name (e.g., System.Collections.Generic)
pub fn parse_qualified_name(input: &str) -> BResult<&str, Vec<Identifier>> {
    context(
        "qualified name (expected dot-separated identifiers like 'System.Collections.Generic')",
        |input| {
            let dot = bws(nom_char('.'));
            let mut identifier = bws(parse_identifier);

            // An identifier followed by zero or more .identifier segments
            let (rest, first) = identifier(input)?;
            let (rest, others) = many0(preceded(dot, identifier))(rest)?;

            let mut result = vec![first];
            result.extend(others);

            Ok((rest, result))
        },
    )(input)
}
