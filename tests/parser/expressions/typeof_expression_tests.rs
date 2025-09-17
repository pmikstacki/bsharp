// Tests for parsing typeof expressions

use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::parser::expressions::typeof_expression_parser::parse_typeof_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};

fn parse_typeof_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_typeof_expression(code) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_expression(code) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_typeof_primitive_type() {
    let code = "typeof(int)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with primitive type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Int) = typeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected int primitive type, got: {:?}",
                typeof_expr.target_type
            );
        }
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_string_type() {
    let code = "typeof(string)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with string type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::String) = typeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected string primitive type, got: {:?}",
                typeof_expr.target_type
            );
        }
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_reference_type() {
    let code = "typeof(MyClass)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with reference type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        if let Type::Reference(id) = &typeof_expr.target_type {
            assert_eq!(id.name, "MyClass");
        } else {
            panic!(
                "Expected reference type, got: {:?}",
                typeof_expr.target_type
            );
        }
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_generic_type() {
    let code = "typeof(List<int>)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with generic type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        assert!(matches!(typeof_expr.target_type, Type::Generic { .. }));
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_array_type() {
    let code = "typeof(int[])";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with array type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        assert!(matches!(typeof_expr.target_type, Type::Array { .. }));
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_in_full_expression_parser() {
    let code = "typeof(bool)";
    let result = parse_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof in full expression parser: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Bool) = typeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected bool primitive type, got: {:?}",
                typeof_expr.target_type
            );
        }
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_with_whitespace() {
    let variations = vec![
        "typeof(  int  )",
        "typeof(\tint\t)",
        "typeof(\nint\n)",
        "  typeof(int)  ",
    ];

    for code in variations {
        let result = parse_expr_helper(code);
        assert!(
            result.is_ok(),
            "Failed to parse typeof with whitespace '{}': {:?}",
            code,
            result
        );

        if let Ok(Expression::Typeof(typeof_expr)) = result {
            if let Type::Primitive(PrimitiveType::Int) = typeof_expr.target_type {
                // Success
            } else {
                panic!("Expected int primitive type for input: '{}'", code);
            }
        } else {
            panic!("Expected typeof expression for input: '{}'", code);
        }
    }
}

#[test]
fn test_parse_typeof_void() {
    let code = "typeof(void)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with void type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        if let Type::Primitive(PrimitiveType::Void) = typeof_expr.target_type {
            // Success
        } else {
            panic!(
                "Expected void primitive type, got: {:?}",
                typeof_expr.target_type
            );
        }
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_typeof_nullable_type() {
    let code = "typeof(int?)";
    let result = parse_typeof_expr_helper(code);
    assert!(
        result.is_ok(),
        "Failed to parse typeof with nullable type: {:?}",
        result
    );

    if let Ok(Expression::Typeof(typeof_expr)) = result {
        assert!(matches!(typeof_expr.target_type, Type::Nullable { .. }));
    } else {
        panic!("Expected typeof expression");
    }
}

#[test]
fn test_parse_invalid_typeof_expressions() {
    let invalid_cases = vec![
        "typeofint",   // No parentheses
        "typeof()",    // Empty parentheses
        "TYPEOF(int)", // Wrong case
        "typeof int",  // Missing parentheses
        "typeof(123)", // Invalid type
    ];

    for code in invalid_cases {
        let result = parse_typeof_expr_helper(code);
        assert!(
            result.is_err(),
            "Expected parse error for invalid syntax: '{}'",
            code
        );
    }
}
