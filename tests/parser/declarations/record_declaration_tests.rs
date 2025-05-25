// Tests for parsing record declarations

use bsharp::parser::nodes::declarations::{RecordDeclaration, Modifier};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::{Parameter, Type, PrimitiveType};
use bsharp::parsers::declarations::type_declaration_parser::parse_record_declaration;

fn parse_record_decl_test(code: &str) -> Result<RecordDeclaration, String> {
    match parse_record_declaration(code) {
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
        name: Identifier { name: "Person".to_string() },
        is_struct: false,
        parameters: Some(vec![
            Parameter {
                parameter_type: Type::Primitive(PrimitiveType::String),
                name: Identifier { name: "Name".to_string() },
            },
        ]),
        base_types: vec![],
        body_declarations: vec![],
    };
    assert_eq!(parse_record_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_positional_record_with_multiple_parameters() {
    let code = "record Person(string FirstName, string LastName, int Age);";
    let result = parse_record_decl_test(code);
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.name, "Person");
    assert!(record.parameters.is_some());
    let params = record.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 3);
    assert_eq!(params[0].name.name, "FirstName");
    assert_eq!(params[1].name.name, "LastName");
    assert_eq!(params[2].name.name, "Age");
}

#[test]
fn test_parse_positional_record_with_base() {
    let code = "record Employee(string Name) : Person;";
    let result = parse_record_decl_test(code);
    if result.is_err() {
        println!("Error parsing '{}': {}", code, result.as_ref().unwrap_err());
    }
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.name, "Employee");
    assert_eq!(record.base_types.len(), 1);
    if let Type::Reference(ref id) = record.base_types[0] {
        assert_eq!(id.name, "Person");
    } else {
        panic!("Expected base type to be a reference");
    }
}

#[test]
fn test_parse_record_struct() {
    let code = "record struct Point { }";
    let result = parse_record_decl_test(code);
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.name.name, "Point");
    assert!(record.is_struct);
}

#[test]
fn test_parse_record_with_modifiers() {
    let code = "public record Customer { }";
    let result = parse_record_decl_test(code);
    assert!(result.is_ok());
    let record = result.unwrap();
    assert_eq!(record.modifiers.len(), 1);
    assert_eq!(record.modifiers[0], Modifier::Public);
}
