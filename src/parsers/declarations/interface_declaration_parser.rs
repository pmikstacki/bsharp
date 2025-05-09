use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::InterfaceDeclaration;
use crate::parser::parser_helpers::nom_to_bs;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::type_declaration_helpers::{parse_type_declaration_header, parse_open_brace, parse_close_brace};

/// Parse an interface declaration
/// Example in C#:
/// ```csharp
/// public interface IDisposable {
///     void Dispose();
/// }
/// ```
pub fn parse_interface_declaration<'a>(input: &'a str) -> BResult<&'a str, InterfaceDeclaration<'a>> {
    // Use the common type declaration header parser
    let (input, base_decl) = parse_type_declaration_header(input, "interface", "interface")?;
    
    // Parse the interface body
    let (input, _) = parse_open_brace(input)?;
    
    // TODO: Add proper interface member parsing here
    // For now, we'll just parse until closing brace
    let (input, _) = parse_close_brace(input)?;
    
    // Convert AttributeList to Vec<Attribute> as expected by InterfaceDeclaration
    let attributes = base_decl.attributes.into_iter()
        .flat_map(|list| list.attributes)
        .collect();
    
    Ok((input, InterfaceDeclaration {
        attributes,
        modifiers: base_decl.modifiers,
        name: base_decl.name,
        type_parameters: base_decl.type_parameters.unwrap_or_default(),
        base_types: base_decl.base_types,
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
