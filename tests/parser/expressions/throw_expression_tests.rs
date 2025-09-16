// Tests for parsing throw expressions

use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::parser::expressions::expression_parser::parse_expression;
use bsharp::parser::expressions::throw_expression_parser::parse_throw_expression;

fn parse_throw_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_throw_expression(code) {
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
fn test_parse_simple_throw_expression() {
    let code = "throw new Exception()";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse simple throw expression: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            assert!(matches!(**inner_expr, Expression::New(_)));
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_variable() {
    let code = "throw ex";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw variable: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            if let Expression::Variable(var) = &**inner_expr {
                assert_eq!(var.name, "ex");
            } else {
                panic!("Expected variable expression");
            }
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_literal() {
    let code = "throw \"Error message\"";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw literal: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            if let Expression::Literal(Literal::String(msg)) = &**inner_expr {
                assert_eq!(msg, "Error message");
            } else {
                panic!("Expected string literal expression");
            }
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_without_expression() {
    let code = "throw";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse bare throw: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_none());
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_complex_expression() {
    let code = "throw GetException()";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw with method call: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            assert!(matches!(**inner_expr, Expression::Invocation(_)));
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_member_access() {
    let code = "throw ex.InnerException";
    let result = parse_throw_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw with member access: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            assert!(matches!(**inner_expr, Expression::MemberAccess(_)));
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_in_full_expression_parser() {
    let code = "throw new ArgumentException(\"Invalid argument\")";
    let result = parse_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw in full expression parser: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            assert!(matches!(**inner_expr, Expression::New(_)));
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_throw_with_whitespace() {
    let variations = vec![
        "throw   ex",
        "throw\tex",
        "throw\nex",
        "throw\r\nex",
        "  throw  ex  ",
    ];
    
    for code in variations {
        let result = parse_expr_helper(code);
        assert!(result.is_ok(), "Failed to parse throw with whitespace '{}': {:?}", code, result);
        
        if let Ok(Expression::Throw(throw_expr)) = result {
            assert!(throw_expr.expr.is_some());
        } else {
            panic!("Expected throw expression for input: '{}'", code);
        }
    }
}

#[test]
fn test_throw_expression_precedence() {
    // Throw should have low precedence
    let code = "throw x + y";
    let result = parse_expr_helper(code);
    assert!(result.is_ok(), "Failed to parse throw with addition: {:?}", result);
    
    if let Ok(Expression::Throw(throw_expr)) = result {
        assert!(throw_expr.expr.is_some());
        if let Some(ref inner_expr) = throw_expr.expr {
            // Should parse as throw (x + y), not (throw x) + y
            assert!(matches!(**inner_expr, Expression::Binary { .. }));
        }
    } else {
        panic!("Expected throw expression");
    }
}

#[test]
fn test_parse_invalid_throw_expressions() {
    let invalid_cases = vec![
        "throwexception",  // No space
        "THROW ex",        // Wrong case
        "throw;",          // Semicolon not allowed in expression context
    ];
    
    for code in invalid_cases {
        let result = parse_throw_expr_helper(code);
        assert!(result.is_err(), "Expected parse error for invalid syntax: '{}'", code);
    }
}
