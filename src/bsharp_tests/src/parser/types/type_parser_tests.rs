// use nom::IResult; // Removing unused import
use parser::types::type_parser::parse_type_expression;
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type};

// Helper function for unwrapping syntax results
fn parse_test(code: &str) -> Result<Type, String> {
    match parse_type_expression(code.into()) {
        Ok((remaining, ty)) if remaining.trim().is_empty() => Ok(ty),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_primitive_types() {
    assert_eq!(
        parse_test("int").unwrap(),
        Type::Primitive(PrimitiveType::Int)
    );
    assert_eq!(
        parse_test("bool").unwrap(),
        Type::Primitive(PrimitiveType::Bool)
    );
    assert_eq!(
        parse_test("string").unwrap(),
        Type::Primitive(PrimitiveType::String)
    );
    assert_eq!(
        parse_test("void").unwrap(),
        Type::Primitive(PrimitiveType::Void)
    );
}

#[test]
fn test_identifier_type() {
    assert_eq!(
        parse_test("MyClass").unwrap(),
        Type::Reference(Identifier::new("MyClass"))
    );
}

#[test]
fn test_array_type() {
    let expected = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 1,
    };
    assert_eq!(parse_test("int[]").unwrap(), expected);

    let qualified = Type::Array {
        element_type: Box::new(Type::Reference(Identifier::QualifiedIdentifier(vec![
            "System".to_string(),
            "String".to_string(),
        ]))),
        rank: 1,
    };
    assert_eq!(parse_test("System.String[]").unwrap(), qualified);
}

#[test]
fn test_nullable_type() {
    let expected = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_test("int?").unwrap(), expected);

    let nullable_class = Type::Nullable(Box::new(Type::Reference(Identifier::new("DateTime"))));
    assert_eq!(parse_test("DateTime?").unwrap(), nullable_class);
}

#[test]
fn test_nullable_primitive() {
    let expected = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Int)));
    assert_eq!(parse_test("int?").unwrap(), expected);
}

#[test]
fn test_nullable_identifier() {
    let expected = Type::Nullable(Box::new(Type::Reference(Identifier::new("MyClass"))));
    assert_eq!(parse_test("MyClass?").unwrap(), expected);
}

#[test]
fn test_array_of_nullable() {
    let inner = Type::Nullable(Box::new(Type::Primitive(PrimitiveType::Int)));
    let expected = Type::Array {
        element_type: Box::new(inner),
        rank: 1,
    };
    assert_eq!(parse_test("int?[]").unwrap(), expected);
}

#[test]
fn test_nullable_array() {
    let inner = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 1,
    };
    let expected = Type::Nullable(Box::new(inner));
    assert_eq!(parse_test("int[]?").unwrap(), expected);
}

#[test]
fn test_multi_dimensional_array() {
    let expected = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 2,
    };
    assert_eq!(parse_test("int[,]").unwrap(), expected);

    let expected3d = Type::Array {
        element_type: Box::new(Type::Primitive(PrimitiveType::Int)),
        rank: 3,
    };
    assert_eq!(parse_test("int[,,]").unwrap(), expected3d);
}
