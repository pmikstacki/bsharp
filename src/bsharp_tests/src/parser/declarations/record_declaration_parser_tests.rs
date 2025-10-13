#![cfg(test)]
use parser::expressions::declarations::type_declaration_parser::parse_record_declaration;
use syntax::declarations::Modifier;
use syntax::identifier::Identifier;
use syntax::types::Type;

// Local test helper to avoid import issues
#[allow(dead_code)]
fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
where
    F: FnOnce(&'a str) -> parser::syntax::errors::BResult<&'a str, O>,
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
    assert!(result.parameters.is_none());
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
    let parameters_unwrapped = result.parameters.unwrap();

    assert_eq!(result.name.name, "Person");
    assert_eq!(parameters_unwrapped.len(), 2);
    // Check parameter names
    assert_eq!(parameters_unwrapped[0].name.name, "FirstName");
    assert_eq!(parameters_unwrapped[1].name.name, "LastName");

    // Check parameter types
    if let Type::Primitive(prim_type) = &parameters_unwrapped[0].parameter_type {
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
    assert_eq!(
        result.attributes[0].attributes[0].name,
        Identifier::new("Serializable")
    );

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
