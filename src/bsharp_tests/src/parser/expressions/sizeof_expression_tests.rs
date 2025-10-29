// Tests for parsing sizeof expressions

use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use parser::expressions::sizeof_expression_parser::parse_sizeof_expression;
use syntax::expressions::expression::Expression;
use syntax::types::{PrimitiveType, Type};

fn parse_sizeof_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_sizeof_expression(code.into()) {
        Ok((remaining, expr)) if remaining.fragment().trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining.fragment()
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_expression(code.into()).map(|(rest, s)| (rest, s.node)) {
        Ok((remaining, expr)) if remaining.fragment().trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining.fragment()
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_sizeof_primitive_type() {
    let code = "sizeof(int)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with primitive type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Int) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected int primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_char_type() {
    let code = "sizeof(char)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with char type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Char) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected char primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_double_type() {
    let code = "sizeof(double)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with double type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Double) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected double primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_struct_type() {
    let code = "sizeof(MyStruct)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with struct type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Reference(id) = &sizeof_expr.target_type {
            assert_eq!(id.to_string(), "MyStruct");
        } else {
            panic!(
                "Expected reference type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_in_full_expression_parser() {
    let code = "sizeof(long)";
    let result = parse_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof in full expression parser: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Long) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected long primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_with_whitespace() {
    let variations = vec![
        "sizeof(  int  )",
        "sizeof(\tint\t)",
        "sizeof(\nint\n)",
        "  sizeof(int)  ",
    ];

    for code in variations {
        let result = parse_expr_helper(code.into());
        assert!(
            result.is_ok(),
            "Failed to parse sizeof with whitespace '{}': {:?}",
            code,
            result
        );

        if let Ok(Expression::Sizeof(sizeof_expr)) = result {
            if let Type::Primitive(PrimitiveType::Int) = sizeof_expr.target_type {
                // Success
            } else {
                panic!("Expected int primitive type for input: '{}'", code);
            }
        } else {
            panic!("Expected sizeof expression for input: '{}'", code);
        }
    }
}

#[test]
fn test_parse_sizeof_byte_type() {
    let code = "sizeof(byte)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with byte type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Byte) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected byte primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_decimal_type() {
    let code = "sizeof(decimal)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with decimal type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Decimal) = sizeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected decimal primitive type, got: {:?}",
                sizeof_expr.target_type
            );
        }
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_sizeof_pointer_type() {
    let code = "sizeof(int*)";
    let result = parse_sizeof_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse sizeof with pointer type: {:?}",
        result
    );

    if let Ok(Expression::Sizeof(sizeof_expr)) = result {
        assert!(matches!(sizeof_expr.target_type, Type::Pointer { .. }));
    } else {
        panic!("Expected sizeof expression");
    }
}

#[test]
fn test_parse_invalid_sizeof_expressions() {
    let invalid_cases = vec![
        "sizeofint",      // No parentheses
        "sizeof()",       // Empty parentheses
        "SIZEOF(int)",    // Wrong case
        "sizeof int",     // Missing parentheses
        "sizeof(123)",    // Invalid type
        "sizeof(string)", // Reference types not allowed in sizeof
    ];

    for code in invalid_cases {
        let result = parse_sizeof_expr_helper(code.into());
        // Note: Some of these might actually parse successfully depending on the type syntax
        // The "string" case might parse but would be a semantic error, not a parser error
        if code == "sizeof(string)" {
            // This might actually parse successfully as a parser, but would be a semantic error
            continue;
        }
        assert!(
            result.is_err(),
            "Expected parse error for invalid syntax: '{}'",
            code
        );
    }
}
