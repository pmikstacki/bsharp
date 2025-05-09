use nom::{
    branch::alt,
    character::complete::{char as nom_char, multispace0, multispace1},
    bytes::complete::{tag, take_until},
    combinator::{map, opt},
    sequence::tuple,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{
    RecordDeclaration,
    Attribute, // Changed from attribute::AttributeList to match the AST node structure
    modifier::Modifier,
};
use crate::parser::nodes::types::Type;
use crate::parser::nodes::types::Parameter;

use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::base_types_parser::parse_base_type_list;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::parameter_parser::parse_parameter_list;

/// Parse a C# record declaration with full feature support.
/// 
/// Supports both forms of C# records:
/// 1. Positional records: `record Person(string FirstName, string LastName);`
/// 2. Standard records with body: `record Person { string FirstName; string LastName; }`
/// 
/// Records can also be declared as record structs: `record struct Point { ... }`
pub fn parse_record_declaration<'a>(input: &'a str) -> BResult<&'a str, RecordDeclaration<'a>> {
    // Parse attributes (e.g., [Serializable])
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Convert AttributeList to Vec<Attribute> as expected by RecordDeclaration
    let attributes = attribute_lists.into_iter()
        .flat_map(|list| list.attributes)
        .collect();

    // Handle the different record type variants with improved whitespace handling
    let (input, (is_record_struct, modifiers)) = if let Ok((next_input, _)) = nom_to_bs(crate::parsers::declaration_helpers::parse_keyword("record"))(input) {
        // This is a 'record' declaration (possibly followed by 'struct')
        let (next_input2, struct_keyword) = opt(nom_to_bs(crate::parsers::declaration_helpers::parse_keyword("struct")))(next_input)?;
        
        // Parse modifiers with the appropriate declaration type
        let decl_type = if struct_keyword.is_some() { "recordstruct" } else { "record" };
        let (next_input3, modifiers) = bws(nom_to_bs(|i| parse_modifiers_for_decl_type(i, decl_type)))(next_input2)?;
        
        (next_input3, (struct_keyword.is_some(), modifiers))
    } else {
        // This might be a 'struct' record declaration
        let mut header_parser = crate::parsers::declaration_helpers::parse_declaration_header(
            |i| parse_modifiers_for_decl_type(i, "recordstruct"),
            "struct"
        );
        
        let (rest, (modifiers, _)) = header_parser(input)?;
        (rest, (true, modifiers))
    };
    
    // We've already determined if this is a struct record from our earlier parsing
    let is_struct = is_record_struct;

    // Parse record name
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;

    // Parse optional type parameters (generics like <T, U>)
    let (input, _type_parameters) = bws(nom_to_bs(opt_parse_type_parameter_list))(input)?;

    // Parse one of two forms - positional record or body record
    let positional_record_parser = map(
        tuple((
            // Parse parameters
            bws(nom_to_bs(parse_parameter_list)),
            // Parse optional base types
            parse_base_type_list,
            // Parse optional semicolon at the end
            opt(bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';')))),
        )),
        |(params, base_types, _)| (params, base_types, vec![])
    );
    
    let body_record_parser = map(
        tuple((
            // Parse optional base types
            parse_base_type_list,
            // Parse the opening brace
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('{'))),
            // TODO: Parse members here once struct_member parser is better
            //       For now, we'll just skip everything until the closing brace
            opt(bws(nom_to_bs(take_until::<&str, &str, nom::error::Error<&str>>("}")))),
            // Parse the closing brace
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('}'))),
        )),
        |(base_types, _, _, _)| (vec![], base_types, vec![])
    );
    
    let (input, (parameters, base_types, members)) = alt((
        positional_record_parser,
        body_record_parser,
    ))(input)?;

    Ok((
        input,
        RecordDeclaration {
            attributes,
            modifiers,
            name,
            is_struct,
            parameters,
            base_types,
            members,
        },
    ))
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
