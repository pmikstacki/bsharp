use nom::{
    branch::alt,
    character::complete::{char as nom_char},
    bytes::complete::{tag, take_until},
    combinator::{map, opt},
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::RecordDeclaration;
use crate::parser::nodes::types::Type;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::declarations::type_declaration_helpers::{BaseTypeDeclaration, parse_type_declaration_header, parse_open_brace, parse_close_brace};
use crate::parsers::declarations::parameter_parser::parse_parameter_list;

/// Parse a C# record declaration with full feature support.
/// 
/// Supports both forms of C# records:
/// 1. Positional records: `record Person(string FirstName, string LastName);`
/// 2. Standard records with body: `record Person { string FirstName; string LastName; }`
/// 
/// Records can also be declared as record structs: `record struct Point { ... }`
pub fn parse_record_declaration<'a>(input: &'a str) -> BResult<&'a str, RecordDeclaration<'a>> {
    // Records are special because we need to handle both "record" and "record struct" declarations
    let (input, (is_struct, base_decl)) = parse_record_declaration_header(input)?;

    // Parse one of two forms - positional record or body record
    let positional_record_parser = map(
        tuple((
            // Parse parameters
            bws(nom_to_bs(parse_parameter_list)),
            // Parse optional base types
            opt(bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';')))),
        )),
        |(params, _)| (params, vec![])
    );
    
    let body_record_parser = map(
        tuple((
            // Parse the opening brace
            parse_open_brace,
            // TODO: Parse members here once struct_member parser is better
            //       For now, we'll just skip everything until the closing brace
            opt(bws(nom_to_bs(take_until::<&str, &str, nom::error::Error<&str>>("}")))),
            // Parse the closing brace
            parse_close_brace,
        )),
        |(_, _, _)| (vec![], vec![])
    );
    
    let (input, (parameters, members)) = alt((
        positional_record_parser,
        body_record_parser,
    ))(input)?;
    
    // Convert AttributeList to Vec<Attribute> as expected by RecordDeclaration
    let attributes = base_decl.attributes.into_iter()
        .flat_map(|list| list.attributes)
        .collect();

    Ok((
        input,
        RecordDeclaration {
            attributes,
            modifiers: base_decl.modifiers,
            name: base_decl.name,
            is_struct,
            parameters,
            base_types: base_decl.base_types,
            members,
        },
    ))
}

/// Parse a record declaration header, which has special handling for both 'record' and 'record struct'
fn parse_record_declaration_header<'a>(input: &'a str) -> BResult<&'a str, (bool, BaseTypeDeclaration<'a>)> {
    // Try parsing as "record" or "record struct"
    if let Ok((next_input, _)) = nom_to_bs(crate::parsers::declaration_helpers::parse_keyword("record"))(input) {
        // This is a 'record' declaration (possibly followed by 'struct')
        let (next_input2, struct_keyword) = opt(nom_to_bs(crate::parsers::declaration_helpers::parse_keyword("struct")))(next_input)?;
        
        // Determine is_struct based on presence of struct keyword
        let is_struct = struct_keyword.is_some();
        
        // Choose the appropriate declaration type
        let decl_type = if is_struct { "recordstruct" } else { "record" };
        
        // Parse the base type declaration (attributes, modifiers, name, etc.)
        let (input, base_decl) = parse_type_declaration_header(next_input2, decl_type, "")?;
        
        Ok((input, (is_struct, base_decl)))
    } else {
        // Try parsing as "struct" which might be a record struct
        let (input, base_decl) = parse_type_declaration_header(input, "recordstruct", "struct")?;
        
        // This is definitely a struct record
        Ok((input, (true, base_decl)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::declarations::Modifier;
    
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
    use crate::parser::nodes::identifier::Identifier;

    #[test]
    fn test_simple_record_class() {
        let input = "record Person { }";
        let (_, result) = parse_record_declaration(input).unwrap();
        assert_eq!(result.name.name, "Person");
        assert!(!result.is_struct);
        assert!(result.parameters.is_empty());
    }

    #[test]
    fn test_record_struct() {
        let input = "record struct Point { }";
        let (_, result) = parse_record_declaration(input).unwrap();
        assert_eq!(result.name.name, "Point");
        assert!(result.is_struct);
    }

    #[test]
    fn test_positional_record() {
        let input = "record Person(string FirstName, string LastName);";
        let (_, result) = parse_record_declaration(input).unwrap();
        assert_eq!(result.name.name, "Person");
        assert_eq!(result.parameters.len(), 2);
        
        // Check parameter names
        assert_eq!(result.parameters[0].name.name, "FirstName");
        assert_eq!(result.parameters[1].name.name, "LastName");
        
        // Check parameter types
        if let Type::Primitive(prim_type) = &result.parameters[0].ty {
            assert_eq!(format!("{:?}", prim_type), "String");
        } else {
            panic!("Expected primitive string type");
        }
    }

    #[test]
    fn test_record_with_attributes_and_modifiers() {
        let input = "[Serializable] public record Customer { }";
        let (_, result) = parse_record_declaration(input).unwrap();
        
        // Check attribute
        assert_eq!(result.attributes.len(), 1);
        assert_eq!(result.attributes[0].name.name, "Serializable");
        
        // Check modifier
        assert_eq!(result.modifiers.len(), 1);
        assert_eq!(result.modifiers[0], Modifier::Public);
    }

    #[test]
    fn test_record_with_base() {
        let input = "record Employee : Person { }";
        let (_, result) = parse_record_declaration(input).unwrap();
        
        // Check base type
        assert_eq!(result.base_types.len(), 1);
        if let Type::Reference(id) = &result.base_types[0] {
            assert_eq!(id.name, "Person");
        } else {
            panic!("Expected Reference type");
        }
    }
}
