#![cfg(test)]
use bsharp::parsers::declarations::record_declaration_parser::parse_record_declaration;
use bsharp::parser::nodes::declarations::Modifier;
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::Type;

// Local test helper to avoid import issues
fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
where
    F: FnOnce(&'a str) -> bsharp::parser::errors::BResult<&'a str, O>,
{
    match parser(input) {
        Ok((remaining, result)) => Ok((remaining, result)),
        Err(err) => Err(format!("Parse error: {:?}", err)),
    }
}

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
