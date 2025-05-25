use crate::parser::errors::BResult;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char as nom_char},
    combinator::{map, recognize, verify},
    multi::many0,
    sequence::{pair, preceded},
};

// Parse a C# identifier (letters, digits, underscore, but must start with letter or underscore)
pub fn parse_identifier(input: &str) -> BResult<&str, Identifier> {
    // C# identifiers can start with a letter or underscore, followed by
    // letters, digits, or underscores. Unicode support would be more complex.
    let identifier_start = alt((alpha1::<&str, nom::error::Error<&str>>, recognize(nom_char::<&str, nom::error::Error<&str>>('_'))));
    
    let identifier_chars = recognize(pair(
        identifier_start,
        many0(alt((alphanumeric1::<&str, nom::error::Error<&str>>, recognize(nom_char::<&str, nom::error::Error<&str>>('_')))))
    ));
    
    // Verify the identifier is not a reserved keyword
    let identifier_parser = verify(identifier_chars, |s: &str| !is_keyword(s));
    
    // Map to the IdentifierNameSyntax struct
    map(bws(nom_to_bs(identifier_parser)), |name: &str| Identifier { name: name.to_string() })(input)
}

// Function to check if a string is a C# keyword
fn is_keyword(word: &str) -> bool {
    // List of C# keywords
    const KEYWORDS: &[&str] = &[
        "abstract", "as", "base", "bool", "break", "byte", "case", "catch", "char", "checked",
        "class", "const", "continue", "decimal", "default", "delegate", "do", "double", "else",
        "enum", "event", "explicit", "extern", "false", "finally", "fixed", "float", "for",
        "foreach", "goto", "if", "implicit", "in", "int", "interface", "internal", "is", "lock",
        "long", "namespace", "new", "null", "object", "operator", "out", "override", "params",
        "private", "protected", "public", "readonly", "ref", "return", "sbyte", "sealed",
        "short", "sizeof", "stackalloc", "static", "string", "struct", "switch", "this", "throw",
        "true", "try", "typeof", "uint", "ulong", "unchecked", "unsafe", "ushort", "using",
        "virtual", "void", "volatile", "while"
    ];
    
    KEYWORDS.contains(&word)
}

// Parse a qualified name (e.g., System.Collections.Generic)
pub fn parse_qualified_name(input: &str) -> BResult<&str, Vec<Identifier>> {
    let dot = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('.')));
    let mut identifier = bws(parse_identifier);
    
    // An identifier followed by zero or more .identifier segments
    let (rest, first) = identifier(input)?;
    let (rest, mut others) = many0(preceded(dot, identifier))(rest)?;
    
    let mut result = vec![first];
    result.append(&mut others);
    
    Ok((rest, result))
}
