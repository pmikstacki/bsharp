use crate::parser::expressions::declarations::attribute_parser::parse_attribute;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;

use nom::Parser;
use nom::branch::alt;
use nom::character::complete::satisfy;
use nom::sequence::delimited;
use nom_supreme::ParserExt;

use crate::keywords::global_keywords::{kw_assembly, kw_module};
use crate::tokens::separators::tok_colon;
use nom::combinator::map;
use syntax::Identifier;
use syntax::declarations::GlobalAttribute;

/// Parse a global attribute with target specification
/// Examples:
/// - `[assembly: System.Reflection.AssemblyVersion("1.0.0.0")]`
/// - `[module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]`
pub fn parse_global_attribute(input: Span) -> BResult<GlobalAttribute> {
    // [target: Attribute]
    let (input, _) = delimited(ws, satisfy(|c| c == '['), ws).parse(input)?;
    let (input, target) = parse_attribute_target(input)?;
    let (input, _) = delimited(ws, tok_colon(), ws).parse(input)?;
    let (input, attribute) = delimited(ws, parse_attribute, ws).parse(input)?;
    let (input, _) = delimited(ws, satisfy(|c| c == ']'), ws).parse(input)?;

    Ok((input, GlobalAttribute { target, attribute }))
}

/// Parse attribute target (assembly, module, etc.)
fn parse_attribute_target(input: Span) -> BResult<Identifier> {
    alt((
        // Parse specific known targets first
        map(delimited(ws, kw_assembly(), ws), |_| {
            Identifier::Simple("assembly".to_string())
        }),
        map(delimited(ws, kw_module(), ws), |_| {
            Identifier::Simple("module".to_string())
        }),
        // For any other identifier targets, parse as identifier directly
        delimited(ws, crate::parser::identifier_parser::parse_identifier, ws),
    ))
    .context("attribute target")
    .parse(input)
}

/// Parse multiple global attributes that might appear at the top of a file
/// Examples:
/// ```csharp
/// [assembly: System.Reflection.AssemblyVersion("1.0.0.0")]
/// [assembly: System.Reflection.AssemblyFileVersion("1.0.0.0")]
/// [module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]
/// ```
pub fn parse_global_attributes(input: Span) -> BResult<Vec<GlobalAttribute>> {
    // According to Nom docs, many0 should handle failure gracefully
    // First skip any leading whitespace
    let (mut remaining, _) = ws(input)?;

    let mut attributes = Vec::new();

    // Parse global attributes one by one until we can't find any more
    while let Ok((rest, attr)) = parse_global_attribute(remaining) {
        attributes.push(attr);
        // Skip whitespace after the attribute
        let (after_ws, _) = ws(rest)?;
        remaining = after_ws;
    }

    Ok((remaining, attributes))
}
