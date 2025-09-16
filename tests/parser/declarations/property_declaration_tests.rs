// Tests for parsing property declarations

use bsharp::syntax::nodes::declarations::{PropertyDeclaration, PropertyAccessor, Modifier};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::parser::declarations::property_declaration_parser::parse_property_declaration;

fn parse_property_decl_test(code: &str) -> Result<PropertyDeclaration, String> {
    match parse_property_declaration(code) {
        Ok((rest, decl)) if rest.trim().is_empty() => Ok(decl),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_auto_property() {
    let code = "int Count { get; set; }";
    let expected = PropertyDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get(None),
            PropertyAccessor::Set(None),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_readonly_auto_property() {
    let code = "string Name { get; }";
    let expected = PropertyDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::String),
        name: Identifier { name: "Name".to_string() },
        accessors: vec![
            PropertyAccessor::Get(None),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_getter_with_body() {
    let code = "int Value { get { return _value; } }";
    let expected = PropertyDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Value".to_string() },
        accessors: vec![
            PropertyAccessor::Get(Some("return _value;".to_string())),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_property_with_bodies() {
    let code = "int Total { get { return _total; } set { _total = value; } }";
    let expected = PropertyDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Total".to_string() },
        accessors: vec![
            PropertyAccessor::Get(Some("return _total;".to_string())),
            PropertyAccessor::Set(Some("_total = value;".to_string())),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_init_only_property() {
    let code = "string Id { get; init; }";
    let expected = PropertyDeclaration {
        modifiers: vec![],
        ty: Type::Primitive(PrimitiveType::String),
        name: Identifier { name: "Id".to_string() },
        accessors: vec![
            PropertyAccessor::Get(None),
            PropertyAccessor::Init(None),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_property_with_modifier() {
    let code = "public int Count { get; set; }";
    let expected = PropertyDeclaration {
        modifiers: vec![Modifier::Public],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get(None),
            PropertyAccessor::Set(None),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}

#[test]
fn test_parse_property_with_multiple_modifiers() {
    let code = "public static int Count { get; set; }";
    let expected = PropertyDeclaration {
        modifiers: vec![Modifier::Public, Modifier::Static],
        ty: Type::Primitive(PrimitiveType::Int),
        name: Identifier { name: "Count".to_string() },
        accessors: vec![
            PropertyAccessor::Get(None),
            PropertyAccessor::Set(None),
        ],
        initializer: None,
    };
    assert_eq!(parse_property_decl_test(code), Ok(expected));
}
