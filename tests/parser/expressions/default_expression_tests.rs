// Tests for parsing default expressions

use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::expressions::default_expression::DefaultExpression;
use bsharp::parser::nodes::types::{Type, PrimitiveType};
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::expressions::expression_parser::parse_expression;
use bsharp::parsers::expressions::default_expression_parser::parse_default_expression;

fn parse_default_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_default_expression(code) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!("Didn't consume all input. Remaining: '{}'", remaining)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

fn parse_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_expression(code) {
        Ok((remaining, expr)) if remaining.trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!("Didn't consume all input. Remaining: '{}'", remaining)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_default_with_type() {
    let code = "default(int)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        if let Some(Type::Primitive(PrimitiveType::Int)) = &default_expr.target_type {
            // Success
        } else {
            panic!("Expected int primitive type, got: {:?}", default_expr.target_type);
        }
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_literal() {
    let code = "default";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default literal: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_none());
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_string_type() {
    let code = "default(string)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with string type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        if let Some(Type::Primitive(PrimitiveType::String)) = &default_expr.target_type {
            // Success
        } else {
            panic!("Expected string primitive type, got: {:?}", default_expr.target_type);
        }
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_reference_type() {
    let code = "default(MyClass)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with reference type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        if let Some(Type::Reference(id)) = &default_expr.target_type {
            assert_eq!(id.name, "MyClass");
        } else {
            panic!("Expected reference type, got: {:?}", default_expr.target_type);
        }
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_nullable_type() {
    let code = "default(int?)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with nullable type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        assert!(matches!(default_expr.target_type, Some(Type::Nullable { .. })));
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_generic_type() {
    let code = "default(List<int>)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with generic type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        assert!(matches!(default_expr.target_type, Some(Type::Generic { .. })));
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_in_full_expression_parser() {
    let code = "default(bool)";
    let result = parse_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default in full expression parser: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        if let Some(Type::Primitive(PrimitiveType::Bool)) = &default_expr.target_type {
            // Success
        } else {
            panic!("Expected bool primitive type, got: {:?}", default_expr.target_type);
        }
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_literal_in_full_expression_parser() {
    let code = "default";
    let result = parse_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default literal in full expression parser: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_none());
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_with_whitespace() {
    let variations = vec![
        "default(  int  )",
        "default(\tint\t)",
        "default(\nint\n)",
        "  default(int)  ",
        "  default  ",
    ];
    
    for code in variations {
        let result = parse_expr_helper(code);
        assert!(result.is_ok(), "Failed to parse default with whitespace '{}': {:?}", code, result);
        
        if let Ok(Expression::Default(default_expr)) = result {
            if code.contains("(") {
                // Should have a type
                assert!(default_expr.target_type.is_some());
                if let Some(Type::Primitive(PrimitiveType::Int)) = &default_expr.target_type {
                    // Success
                } else {
                    panic!("Expected int primitive type for input: '{}'", code);
                }
            } else {
                // Should be a literal
                assert!(default_expr.target_type.is_none());
            }
        } else {
            panic!("Expected default expression for input: '{}'", code);
        }
    }
}

#[test]
fn test_parse_default_array_type() {
    let code = "default(int[])";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with array type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        assert!(matches!(default_expr.target_type, Some(Type::Array { .. })));
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_default_void_type() {
    let code = "default(void)";
    let result = parse_default_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse default with void type: {:?}", result);
    
    if let Ok(Expression::Default(default_expr)) = result {
        assert!(default_expr.target_type.is_some());
        if let Some(Type::Primitive(PrimitiveType::Void)) = &default_expr.target_type {
            // Success
        } else {
            panic!("Expected void primitive type, got: {:?}", default_expr.target_type);
        }
    } else {
        panic!("Expected default expression");
    }
}

#[test]
fn test_parse_invalid_default_expressions() {
    let invalid_cases = vec![
        "defaultint",         // No parentheses or space
        "default()",          // Empty parentheses
        "DEFAULT",            // Wrong case
        "default(",           // Incomplete
        "default(123)",       // Invalid type
    ];
    
    for code in invalid_cases {
        let result = parse_default_expr_helper(code);
        assert!(result.is_err(), "Expected parse error for invalid syntax: '{}'", code);
    }
}
