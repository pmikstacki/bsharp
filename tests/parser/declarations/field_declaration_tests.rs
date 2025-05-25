// Tests for parsing field declarations

use bsharp::parser::nodes::declarations::FieldDeclaration;
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::types::{Type, PrimitiveType};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::declarations::field_declaration_parser::parse_field_declaration;

fn parse_field_decl_test(code: &str) -> Result<FieldDeclaration, String> {
    match parse_field_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_field() {
    let code = "int count;";
    let expected = FieldDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "count".to_string() },
        initializer: None,
    };
    assert_eq!(parse_field_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_field_with_initializer() {
    let code = "string message = \"Hello\";";
    let expected = FieldDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::String),
        name: Identifier { name: "message".to_string() },
        initializer: Some(Expression::Literal(Literal::String("Hello".to_string()))),
    };
    assert_eq!(parse_field_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_field_bool_initializer() {
    let code = "bool enabled = true;";
    let expected = FieldDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Bool),
        name: Identifier { name: "enabled".to_string() },
        initializer: Some(Expression::Literal(Literal::Boolean(true))),
    };
    assert_eq!(parse_field_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_field_missing_semicolon() {
    let code = "int value = 5"; // Missing semicolon
    assert!(parse_field_decl_test(code).is_err());
}
