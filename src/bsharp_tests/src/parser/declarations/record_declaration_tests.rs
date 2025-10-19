// Tests for parsing record declarations

use parser::expressions::declarations::type_declaration_parser::parse_record_declaration;
use syntax::declarations::{Modifier, RecordDeclaration};
use syntax::identifier::Identifier;
use syntax::types::{Parameter, PrimitiveType, Type};

fn parse_record_decl_test(code: &str) -> Result<RecordDeclaration, String> {
    match parse_record_declaration(code.into()) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => {
            println!("Parse error for '{}': {:?}", code, e);
            Err(format!("Parse error: {:?}", e))
        }
    }
}

#[test]
fn test_parse_positional_record() {
    let code = "record Person(string Name);";
    let expected = RecordDeclaration {
        attributes: vec![],
        modifiers: vec![],
        name: Identifier::new("Person"),
        is_struct: false,
        parameters: Some(vec![Parameter {
            attributes: vec![],
            modifier: None,
            parameter_type: Type::Primitive(PrimitiveType::String),
            name: Identifier::new("Name"),
            default_value: None,
        }]),
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };
    assert_eq!(parse_record_decl_test(code.into()), Ok(expected));
}

#[test]
fn test_parse_positional_record_with_multiple_parameters() {
    let code = "record Person(string FirstName, string LastName, int Age);";
    let result = parse_record_decl_test(code.into());
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.to_string(), "Person");
    assert!(record.parameters.is_some());
    let params = record.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 3);
    assert_eq!(params[0].name.to_string(), "FirstName");
    assert_eq!(params[1].name.to_string(), "LastName");
    assert_eq!(params[2].name.to_string(), "Age");
}

#[test]
fn test_parse_positional_record_with_base() {
    let code = "record Employee(string Name) : Person;";
    let result = parse_record_decl_test(code.into());
    if result.is_err() {
        println!("Error parsing '{}': {}", code, result.as_ref().unwrap_err());
    }
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.to_string(), "Employee");
    assert_eq!(record.base_types.len(), 1);
    if let Type::Reference(ref id) = record.base_types[0] {
        assert_eq!(id.to_string(), "Person");
    } else {
        panic!("Expected base type to be a reference");
    }
}

#[test]
fn test_parse_record_struct() {
    let code = "record struct Point { }";
    let result = parse_record_decl_test(code.into());
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.to_string(), "Point");
    assert!(record.is_struct);
}

#[test]
fn test_parse_record_with_modifiers() {
    let code = "public record Customer { }";
    let result = parse_record_decl_test(code.into());
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.modifiers.len(), 1);
    assert_eq!(record.modifiers[0], Modifier::Public);
}
