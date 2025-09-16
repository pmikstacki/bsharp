use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::global_attribute::GlobalAttribute;
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bchar, bs_context, keyword};
use crate::parsers::declarations::attribute_parser::parse_attribute;

use nom::combinator::map;

/// Parse a global attribute with target specification
/// Examples: 
/// - `[assembly: System.Reflection.AssemblyVersion("1.0.0.0")]`
/// - `[module: System.Diagnostics.CodeAnalysis.SuppressMessage("Style", "IDE0161")]`
pub fn parse_global_attribute(input: &str) -> BResult<&str, GlobalAttribute> {
    use nom::character::complete::multispace0;
    
    // According to Nom docs, use tuple to parse elements in sequence
    let (input, _) = multispace0(input)?;
    let (input, _) = bchar('[')(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, target) = parse_attribute_target(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, _) = bchar(':')(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, attribute) = parse_attribute(input)?;
    
    let (input, _) = multispace0(input)?;
    let (input, _) = bchar(']')(input)?;
    
    let global_attr = GlobalAttribute {
        target,
        attribute,
    };
    
    Ok((input, global_attr))
}

/// Parse attribute target (assembly, module, etc.)
fn parse_attribute_target(input: &str) -> BResult<&str, Identifier> {
    bs_context(
        "attribute target",
        nom::branch::alt((
            // Parse specific known targets first
            map(keyword("assembly"), |_| Identifier { name: "assembly".to_string() }),
            map(keyword("module"), |_| Identifier { name: "module".to_string() }),
            // For any other identifier targets, parse as identifier directly
            crate::parsers::identifier_parser::parse_identifier,
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
    use nom::character::complete::multispace0;
    
    // According to Nom docs, many0 should handle failure gracefully
    // First skip any leading whitespace
    let (mut remaining, _) = multispace0(input)?;
    
    let mut attributes = Vec::new();
    
    // Parse global attributes one by one until we can't find any more
    loop {
        // Try to parse a global attribute
        match parse_global_attribute(remaining) {
            Ok((rest, attr)) => {
                attributes.push(attr);
                // Skip whitespace after the attribute
                let (after_ws, _) = multispace0(rest)?;
                remaining = after_ws;
            }
            Err(_e) => {
                // No more global attributes found, break the loop
                break;
            }
        }
    }
    
    Ok((remaining, attributes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assembly_attribute() {
        let input = "[assembly: MyAttribute]";
        let result = parse_global_attribute(input);
        assert!(result.is_ok(), "Failed to parse assembly attribute: {:?}", result);
        
        let (remaining, attr) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(attr.target.name, "assembly");
        assert_eq!(attr.attribute.name.name, "MyAttribute");
        assert!(attr.attribute.arguments.is_empty());
    }

    #[test]
    fn test_parse_module_attribute() {
        let input = "[module: TestAttribute]";
        let result = parse_global_attribute(input);
        assert!(result.is_ok(), "Failed to parse module attribute: {:?}", result);
        
        let (remaining, attr) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(attr.target.name, "module");
        assert_eq!(attr.attribute.name.name, "TestAttribute");
    }

    #[test]
    fn test_parse_assembly_attribute_with_arguments() {
        let input = "[assembly: AssemblyVersion(\"1.0.0.0\")]";
        let result = parse_global_attribute(input);
        assert!(result.is_ok(), "Failed to parse assembly attribute with arguments: {:?}", result);
        
        let (remaining, attr) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(attr.target.name, "assembly");
        assert_eq!(attr.attribute.name.name, "AssemblyVersion");
        assert_eq!(attr.attribute.arguments.len(), 1);
    }

    #[test]
    fn test_parse_multiple_global_attributes() {
        let input = r#"[assembly: MyAttribute]
[module: AnotherAttribute]
"#;
        let result = parse_global_attributes(input);
        assert!(result.is_ok(), "Failed to parse multiple global attributes: {:?}", result);
        
        let (_remaining, attrs) = result.unwrap();
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs[0].target.name, "assembly");
        assert_eq!(attrs[0].attribute.name.name, "MyAttribute");
        assert_eq!(attrs[1].target.name, "module");
        assert_eq!(attrs[1].attribute.name.name, "AnotherAttribute");
    }

    #[test]
    fn test_parse_no_global_attributes() {
        let input = "using System;";
        let result = parse_global_attributes(input);
        assert!(result.is_ok(), "Failed to parse empty global attributes: {:?}", result);
        
        let (remaining, attrs) = result.unwrap();
        assert_eq!(remaining, "using System;");
        assert!(attrs.is_empty());
    }
} 