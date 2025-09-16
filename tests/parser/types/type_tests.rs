// Tests for parsing types

use bsharp::syntax::nodes::types::{Type, PrimitiveType};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::parser::types::type_parser::parse_type_expression;

fn parse_type(code: &str) -> Result<Type, String> {
    match parse_type_expression(code) {
        Ok((rest, ty)) if rest.trim().is_empty() => Ok(ty),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_primitive_type() {
    assert_eq!(parse_type("int"), Ok(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_type("bool"), Ok(Type::Primitive(PrimitiveType::Bool)));
    assert_eq!(parse_type("string"), Ok(Type::Primitive(PrimitiveType::String)));
    assert_eq!(parse_type("void"), Ok(Type::Primitive(PrimitiveType::Void)));
    assert_eq!(parse_type("dynamic"), Ok(Type::Dynamic));
}

#[test]
fn test_parse_reference_type() {
    assert_eq!(parse_type("MyClass"), Ok(Type::Reference(Identifier { name: "MyClass".to_string() })));
}

#[test]
fn test_parse_generic_type() {
    let expected = Type::Generic {
        base: Identifier { name: "List".to_string() },
        args: vec![Type::Primitive(PrimitiveType::Int)],
    };
    assert_eq!(parse_type("List<int>"), Ok(expected));
}

#[test]
fn test_parse_array_type() {
    let expected = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 1,
    };
    assert_eq!(parse_type("int[]"), Ok(expected));
    let expected2 = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::String)),
        rank: 2,
    };
    assert_eq!(parse_type("string[,]"), Ok(expected2));
}

#[test]
fn test_parse_nullable_type() {
    let expected = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_type("int?"), Ok(expected));
}

