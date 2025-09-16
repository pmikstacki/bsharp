// Tests for parsing nameof expressions

use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::parser::expressions::expression_parser::parse_expression;
use bsharp::parser::expressions::nameof_expression_parser::parse_nameof_expression;

fn parse_nameof_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_nameof_expression(code) {
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
fn test_parse_simple_nameof_expression() {
    let code = "nameof(variable)";
    let result = parse_nameof_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse simple nameof expression: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        if let Expression::Variable(var) = &*nameof_expr.expr {
            assert_eq!(var.name, "variable");
        } else {
            panic!("Expected variable expression inside nameof");
        }
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_nameof_member_access() {
    let code = "nameof(MyClass.MyProperty)";
    let result = parse_nameof_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse nameof with member access: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        assert!(matches!(*nameof_expr.expr, Expression::MemberAccess(_)));
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_nameof_nested_member_access() {
    let code = "nameof(System.Console.WriteLine)";
    let result = parse_nameof_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse nameof with nested member access: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        assert!(matches!(*nameof_expr.expr, Expression::MemberAccess(_)));
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_nameof_in_full_expression_parser() {
    let code = "nameof(value)";
    let result = parse_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse nameof in full expression parser: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        if let Expression::Variable(var) = &*nameof_expr.expr {
            assert_eq!(var.name, "value");
        } else {
            panic!("Expected variable expression inside nameof");
        }
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_nameof_with_whitespace() {
    let variations = vec![
        "nameof(  variable  )",
        "nameof(\tvariable\t)",
        "nameof(\nvariable\n)",
        "  nameof(variable)  ",
    ];
    
    for code in variations {
        let result = parse_expr_helper(code);
        assert!(result.is_ok(), "Failed to parse nameof with whitespace '{}': {:?}", code, result);
        
        if let Ok(Expression::Nameof(nameof_expr)) = result {
            if let Expression::Variable(var) = &*nameof_expr.expr {
                assert_eq!(var.name, "variable");
            } else {
                panic!("Expected variable expression inside nameof for input: '{}'", code);
            }
        } else {
            panic!("Expected nameof expression for input: '{}'", code);
        }
    }
}

#[test]
fn test_parse_nameof_parameter() {
    let code = "nameof(parameter)";
    let result = parse_nameof_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse nameof with parameter: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        if let Expression::Variable(var) = &*nameof_expr.expr {
            assert_eq!(var.name, "parameter");
        } else {
            panic!("Expected variable expression inside nameof");
        }
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_nameof_property() {
    let code = "nameof(obj.Property)";
    let result = parse_nameof_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse nameof with property: {:?}", result);
    
    if let Ok(Expression::Nameof(nameof_expr)) = result {
        assert!(matches!(*nameof_expr.expr, Expression::MemberAccess(_)));
    } else {
        panic!("Expected nameof expression");
    }
}

#[test]
fn test_parse_invalid_nameof_expressions() {
    let invalid_cases = vec![
        "nameofvariable",     // No parentheses
        "nameof()",           // Empty parentheses
        "NAMEOF(variable)",   // Wrong case
        "nameof variable",    // Missing parentheses
        "nameof(123)",        // Invalid identifier
    ];
    
    for code in invalid_cases {
        let result = parse_nameof_expr_helper(code);
        assert!(result.is_err(), "Expected parse error for invalid syntax: '{}'", code);
    }
}
