use nom::character::complete::char;
use nom::combinator::opt;

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::InterfaceDeclaration;
use crate::parser::parser_helpers::{bws, keyword, nom_to_bs};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::base_types_parser::parse_base_type_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::type_parameter_parser::parse_type_parameter_list;
use crate::parsers::identifier_parser::parse_identifier;

/// Parse an interface declaration
/// Example in C#:
/// ```csharp
/// public interface IDisposable {
///     void Dispose();
/// }
/// ```
pub fn parse_interface_declaration<'a>(input: &'a str) -> BResult<&'a str, InterfaceDeclaration<'a>> {
    // Parse attributes (e.g., [Serializable])
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Convert AttributeList to Vec<Attribute> as expected by InterfaceDeclaration
    let attributes = attribute_lists.into_iter()
        .flat_map(|list| list.attributes)
        .collect();
    
    // Use the improved declaration header parser to handle whitespace and modifiers
    let mut header_parser = crate::parsers::declaration_helpers::parse_declaration_header(
        |i| parse_modifiers_for_decl_type(i, "interface"),
        "interface"
    );
    
    let (input, (modifiers, _)) = header_parser(input)?;
    
    // Parse interface name with proper whitespace handling
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional type parameters for generic interfaces
    let (input, type_parameters) = opt(bws(parse_type_parameter_list))(input)?;
    
    // Parse optional base type list (interfaces can inherit from other interfaces)
    let (input, base_types) = opt(bws(parse_base_type_list))(input)?;
    
    // Parse the interface body
    let (input, _) = bws(nom_to_bs(char::<&str, nom::error::Error<&str>>('{')))(input)?;
    
    // TODO: Add proper interface member parsing here
    // For now, we'll just parse until closing brace
    let (input, _) = bws(nom_to_bs(char::<&str, nom::error::Error<&str>>('}')))(input)?;
    
    Ok((input, InterfaceDeclaration {
        attributes,
        modifiers,
        name,
        type_parameters: type_parameters.unwrap_or_default(),
        base_types: base_types.unwrap_or_default(),
        members: vec![], // Will be filled in later when we implement interface member parsing
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::declarations::Modifier;
    use crate::parser::nodes::types::Type;
    
    // Local test helper to avoid import issues
    fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
    where
        F: FnOnce(&'a str) -> crate::parser::errors::BResult<&'a str, O>,
    {
        match parser(input) {
            Ok((remaining, result)) => Ok((remaining, result)),
            Err(err) => Err(format!("Parse error: {:?}", err)),
        }
    }

    #[test]
    fn test_simple_interface_declaration() {
        let input = "interface IMyInterface { }";
        let result = parse_full_input(input, parse_interface_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "IMyInterface");
        assert!(decl.attributes.is_empty());
        assert!(decl.modifiers.is_empty());
        assert!(decl.type_parameters.is_empty());
        assert!(decl.base_types.is_empty());
    }
    
    #[test]
    fn test_interface_with_modifiers() {
        let input = "public interface IDisposable { }";
        let result = parse_full_input(input, parse_interface_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "IDisposable");
        assert_eq!(decl.modifiers.len(), 1);
        assert_eq!(decl.modifiers[0], Modifier::Public);
    }
    
    #[test]
    fn test_generic_interface() {
        let input = "interface IEnumerable<T> { }";
        let result = parse_full_input(input, parse_interface_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "IEnumerable");
        assert_eq!(decl.type_parameters.len(), 1);
        assert_eq!(decl.type_parameters[0].name.name, "T");
    }
    
    #[test]
    fn test_interface_with_base_types() {
        let input = "interface IList<T> : ICollection<T>, IEnumerable<T> { }";
        let result = parse_full_input(input, parse_interface_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "IList");
        assert_eq!(decl.base_types.len(), 2);
        // Check first base type
        if let Type::Generic { base, args: _ } = &decl.base_types[0] {
            assert_eq!(base.name, "ICollection");
        } else {
            panic!("Expected generic type");
        }
        // Check second base type
        if let Type::Generic { base, args: _ } = &decl.base_types[1] {
            assert_eq!(base.name, "IEnumerable");
        } else {
            panic!("Expected generic type");
        }
    }
    
    #[test]
    fn test_interface_with_attributes() {
        let input = "[Serializable] interface ISerializable { }";
        let result = parse_full_input(input, parse_interface_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "ISerializable");
        assert_eq!(decl.attributes.len(), 1);
        assert_eq!(decl.attributes[0].name.name, "Serializable");
    }
}
