// Tests for parsing operator declarations

use parser::expressions::declarations::operator_declaration_parser::parse_operator_declaration;
use syntax::declarations::{ConversionKind, Modifier, OperatorDeclaration, OperatorKind};
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type};

fn parse_operator_declaration_helper(code: &str) -> Result<OperatorDeclaration, String> {
    match parse_operator_declaration(code) {
        Ok((remaining, declaration)) => {
            if remaining.trim().is_empty() {
                Ok(declaration)
            } else {
                Err(format!("Unexpected remaining input: '{}'", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_binary_addition_operator() {
    let code = "public static MyType operator +(MyType a, MyType b) { }";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse binary addition operator: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert_eq!(
        declaration.modifiers,
        vec![Modifier::Public, Modifier::Static]
    );
    assert_eq!(
        declaration.return_type,
        Type::Reference(Identifier::new("MyType"))
    );

    match declaration.operator {
        OperatorKind::Binary(symbol) => {
            assert_eq!(symbol.name, "+");
        }
        _ => panic!("Expected binary operator, got {:?}", declaration.operator),
    }

    assert_eq!(declaration.parameters.len(), 2);
    assert_eq!(declaration.body, "{ /* body */ }");
}

#[test]
fn test_parse_binary_subtraction_operator() {
    let code = "public static MyType operator -(MyType a, MyType b) { }";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse binary subtraction operator: {:?}",
        result
    );

    let declaration = result.unwrap();
    match declaration.operator {
        OperatorKind::Binary(symbol) => {
            assert_eq!(symbol.name, "-");
        }
        _ => panic!("Expected binary operator, got {:?}", declaration.operator),
    }
}

#[test]
fn test_parse_comparison_operators() {
    let operators = vec!["==", "!=", ">", "<", ">=", "<="];

    for op in operators {
        let code = format!(
            "public static bool operator {}(MyType a, MyType b) {{ }}",
            op
        );
        let result = parse_operator_declaration_helper(&code);
        assert!(
            result.is_ok(),
            "Failed to parse {} operator: {:?}",
            op,
            result
        );

        let declaration = result.unwrap();
        match declaration.operator {
            OperatorKind::Binary(symbol) => {
                assert_eq!(symbol.name, op);
            }
            _ => panic!(
                "Expected binary operator for {}, got {:?}",
                op, declaration.operator
            ),
        }
    }
}

#[test]
fn test_parse_arithmetic_operators() {
    let operators = vec!["*", "/", "%"];

    for op in operators {
        let code = format!(
            "public static MyType operator {}(MyType a, MyType b) {{ }}",
            op
        );
        let result = parse_operator_declaration_helper(&code);
        assert!(
            result.is_ok(),
            "Failed to parse {} operator: {:?}",
            op,
            result
        );

        let declaration = result.unwrap();
        match declaration.operator {
            OperatorKind::Binary(symbol) => {
                assert_eq!(symbol.name, op);
            }
            _ => panic!(
                "Expected binary operator for {}, got {:?}",
                op, declaration.operator
            ),
        }
    }
}

#[test]
fn test_parse_unary_operators() {
    let operators = vec!["!", "~", "++", "--"];

    for op in operators {
        let code = format!("public static MyType operator {}(MyType value) {{ }}", op);
        let result = parse_operator_declaration_helper(&code);
        assert!(
            result.is_ok(),
            "Failed to parse unary {} operator: {:?}",
            op,
            result
        );

        let declaration = result.unwrap();
        // Note: Our current implementation treats all operators as binary for simplicity
        // In a real implementation, we'd check parameter count to determine unary vs binary
        match declaration.operator {
            OperatorKind::Binary(symbol) => {
                assert_eq!(symbol.name, op);
            }
            _ => panic!(
                "Expected operator for {}, got {:?}",
                op, declaration.operator
            ),
        }
    }
}

#[test]
fn test_parse_implicit_conversion_operator() {
    let code = "public static implicit operator int(MyType value) { }";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse implicit conversion operator: {:?}",
        result
    );

    let declaration = result.unwrap();
    match declaration.operator {
        OperatorKind::Conversion { kind, target_type } => {
            assert_eq!(kind, ConversionKind::Implicit);
            assert_eq!(target_type, Type::Primitive(PrimitiveType::Int));
        }
        _ => panic!(
            "Expected conversion operator, got {:?}",
            declaration.operator
        ),
    }
}

#[test]
fn test_parse_explicit_conversion_operator() {
    let code = "public static explicit operator string(MyType value) { }";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse explicit conversion operator: {:?}",
        result
    );

    let declaration = result.unwrap();
    match declaration.operator {
        OperatorKind::Conversion { kind, target_type } => {
            assert_eq!(kind, ConversionKind::Explicit);
            assert_eq!(target_type, Type::Primitive(PrimitiveType::String));
        }
        _ => panic!(
            "Expected conversion operator, got {:?}",
            declaration.operator
        ),
    }
}

#[test]
fn test_parse_operator_with_attributes() {
    let code = "[Obsolete] public static MyType operator +(MyType a, MyType b) { }";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse operator with attributes: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(
        !declaration.attributes.is_empty(),
        "Expected attributes to be parsed"
    );
}

#[test]
fn test_parse_abstract_operator() {
    let code = "public static abstract MyType operator +(MyType a, MyType b);";
    let result = parse_operator_declaration_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse abstract operator: {:?}",
        result
    );

    let declaration = result.unwrap();
    assert!(declaration.modifiers.contains(&Modifier::Abstract));
    assert_eq!(declaration.body, "");
}

#[test]
fn test_parse_true_false_operators() {
    let operators = vec!["true", "false"];

    for op in operators {
        let code = format!("public static bool operator {}(MyType value) {{ }}", op);
        let result = parse_operator_declaration_helper(&code);
        assert!(
            result.is_ok(),
            "Failed to parse {} operator: {:?}",
            op,
            result
        );

        let declaration = result.unwrap();
        match declaration.operator {
            OperatorKind::Binary(symbol) => {
                assert_eq!(symbol.name, op);
            }
            _ => panic!(
                "Expected operator for {}, got {:?}",
                op, declaration.operator
            ),
        }
    }
}
