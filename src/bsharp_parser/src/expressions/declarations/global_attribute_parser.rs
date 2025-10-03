use crate::parser::expressions::declarations::attribute_parser::parse_attribute;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::global_attribute::GlobalAttribute;
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::parser_helpers::{bchar, context, keyword};

use nom::combinator::map;

/// Parse a global attribute with target specification
/// Examples:
/// - `[assembly: System.Reflection.AssemblyVersion("1.0.0.0")]`
/// - `[module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]`
pub fn parse_global_attribute(input: &str) -> BResult<&str, GlobalAttribute> {
    // According to Nom docs, use tuple to parse elements in sequence
    let (input, _) = ws(input)?;
    let (input, _) = bchar('[')(input)?;

    let (input, _) = ws(input)?;
    let (input, target) = parse_attribute_target(input)?;

    let (input, _) = ws(input)?;
    let (input, _) = bchar(':')(input)?;

    let (input, _) = ws(input)?;
    let (input, attribute) = parse_attribute(input)?;

    let (input, _) = ws(input)?;
    let (input, _) = bchar(']')(input)?;

    let global_attr = GlobalAttribute { target, attribute };

    Ok((input, global_attr))
}

/// Parse attribute target (assembly, module, etc.)
fn parse_attribute_target(input: &str) -> BResult<&str, Identifier> {
    context(
        "attribute target",
        nom::branch::alt((
            // Parse specific known targets first
            map(keyword("assembly"), |_| Identifier {
                name: "assembly".to_string(),
            }),
            map(keyword("module"), |_| Identifier {
                name: "module".to_string(),
            }),
            // For any other identifier targets, parse as identifier directly
            crate::parser::identifier_parser::parse_identifier,
        )),
    )(input)
}

/// Parse multiple global attributes that might appear at the top of a file
/// Examples:
/// ```csharp
/// [assembly: System.Reflection.AssemblyVersion("1.0.0.0")]
/// [assembly: System.Reflection.AssemblyFileVersion("1.0.0.0")]
/// [module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]
/// ```
pub fn parse_global_attributes(input: &str) -> BResult<&str, Vec<GlobalAttribute>> {
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
