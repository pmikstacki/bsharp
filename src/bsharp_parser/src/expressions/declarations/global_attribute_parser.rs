use crate::parser::expressions::declarations::attribute_parser::parse_attribute;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;

use nom::character::complete::satisfy;
use nom::Parser;
use nom::branch::alt;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use nom_supreme::tag::complete::tag;

use nom::combinator::map;
use syntax::declarations::GlobalAttribute;
use syntax::Identifier;

/// Parse a global attribute with target specification
/// Examples:
/// - `[assembly: System.Reflection.AssemblyVersion("1.0.0.0")]`
/// - `[module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]`
pub fn parse_global_attribute<'a>(input: Span<'a>) -> BResult<'a, GlobalAttribute> {
    // [target: Attribute]
    let (input, _) = delimited(ws, satisfy(|c| c == '['), ws).parse(input)?;
    let (input, target) = parse_attribute_target(input)?;
    let (input, _) = delimited(ws, satisfy(|c| c == ':'), ws).parse(input)?;
    let (input, attribute) = delimited(ws, parse_attribute, ws).parse(input)?;
    let (input, _) = delimited(ws, satisfy(|c| c == ']'), ws).parse(input)?;

    Ok((input, GlobalAttribute { target, attribute }))
}

/// Parse attribute target (assembly, module, etc.)
fn parse_attribute_target(input: Span) -> BResult<Identifier> {
    alt((
        // Parse specific known targets first
        map(delimited(ws, tag("assembly"), ws), |_| Identifier {
            name: "assembly".to_string(),
        }),
        map(delimited(ws, tag("module"), ws), |_| Identifier {
            name: "module".to_string(),
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
use crate::syntax::span::Span;
