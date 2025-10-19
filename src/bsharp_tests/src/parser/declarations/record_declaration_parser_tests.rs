#![cfg(test)]
use parser::expressions::declarations::type_declaration_parser::parse_record_declaration;
use syntax::declarations::Modifier;
use syntax::declarations::RecordDeclaration;
use syntax::identifier::Identifier;
use syntax::types::Type;

fn parse_record_decl_test(code: &str) -> Result<RecordDeclaration, String> {
    match parse_record_declaration(code.into()) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_simple_record_class() {
    let input = "record Person { }";
    let result = parse_record_decl_test(input).unwrap();
    assert_eq!(result.name.to_string(), "Person");
    assert!(!result.is_struct);
    assert!(result.parameters.is_none());
}

#[test]
fn test_record_struct() {
    let input = "record struct Point { }";
    let result = parse_record_decl_test(input).unwrap();
    assert_eq!(result.name.to_string(), "Point");
    assert!(result.is_struct);
}

#[test]
fn test_positional_record() {
    let input = "record Person(string FirstName, string LastName);";
    let result = parse_record_decl_test(input).unwrap();
    let parameters_unwrapped = result.parameters.unwrap();

    assert_eq!(result.name.to_string(), "Person");
    assert_eq!(parameters_unwrapped.len(), 2);
    // Check parameter names
    assert_eq!(parameters_unwrapped[0].name.to_string(), "FirstName");
    assert_eq!(parameters_unwrapped[1].name.to_string(), "LastName");

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
    let result = parse_record_decl_test(input).unwrap();

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
    let result = parse_record_decl_test(input).unwrap();

    // Check base type
    assert_eq!(result.base_types.len(), 1);
    if let Type::Reference(id) = &result.base_types[0] {
        assert_eq!(id.to_string(), "Person");
    } else {
        panic!("Expected Reference type");
    }
}
