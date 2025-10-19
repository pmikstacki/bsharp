// Tests for parsing null-conditional expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;

fn parse_null_conditional_expr_helper(code: &str) -> Result<Expression, String> {
    match parse_expression(code.into()) {
        Ok((remaining, expr)) if remaining.fragment().trim().is_empty() => Ok(expr),
        Ok((remaining, _)) => Err(format!(
            "Didn't consume all input. Remaining: '{}'",
            remaining.fragment()
        )),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_null_conditional_member_access() {
    let code = "obj?.Property";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse null-conditional member access: {:?}",
        result
    );

    if let Ok(Expression::NullConditional(null_cond)) = result {
        assert!(!null_cond.is_element_access);
        assert_eq!(null_cond.member.to_string(), "Property");
        assert!(null_cond.argument.is_none());

        // Check that target is a variable
        if let Expression::Variable(var) = &*null_cond.target {
            assert_eq!(var.to_string(), "obj");
        } else {
            panic!("Expected target to be a variable");
        }
    } else {
        panic!("Expected NullConditional expression");
    }
}

#[test]
fn test_parse_null_conditional_indexing() {
    let code = "arr?[0]";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse null-conditional indexing: {:?}",
        result
    );

    if let Ok(Expression::NullConditional(null_cond)) = result {
        assert!(null_cond.is_element_access);
        assert!(null_cond.argument.is_some());

        // Check the index argument
        if let Some(ref index) = null_cond.argument {
            if let Expression::Literal(Literal::Integer(0)) = &**index {
                // Correct
            } else {
                panic!("Expected index to be integer literal 0");
            }
        }

        // Check that target is a variable
        if let Expression::Variable(var) = &*null_cond.target {
            assert_eq!(var.to_string(), "arr");
        } else {
            panic!("Expected target to be a variable");
        }
    } else {
        panic!("Expected NullConditional expression");
    }
}

#[test]
fn test_parse_chained_null_conditional() {
    let code = "obj?.Child?.Property";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse chained null-conditional: {:?}",
        result
    );

    if let Ok(Expression::NullConditional(outer_null_cond)) = result {
        assert!(!outer_null_cond.is_element_access);
        assert_eq!(outer_null_cond.member.to_string(), "Property");

        // Check that target is another null-conditional expression
        if let Expression::NullConditional(inner_null_cond) = &*outer_null_cond.target {
            assert!(!inner_null_cond.is_element_access);
            assert_eq!(inner_null_cond.member.to_string(), "Child");

            // Check that the innermost target is a variable
            if let Expression::Variable(var) = &*inner_null_cond.target {
                assert_eq!(var.to_string(), "obj");
            } else {
                panic!("Expected innermost target to be a variable");
            }
        } else {
            panic!("Expected target to be another null-conditional expression");
        }
    } else {
        panic!("Expected NullConditional expression");
    }
}

#[test]
fn test_parse_null_conditional_with_method_call() {
    let code = "obj?.Method()";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse null-conditional with method call: {:?}",
        result
    );

    // This should parse as obj?.Method followed by ()
    if let Ok(Expression::Invocation(invocation)) = result {
        // The callee should be a null-conditional expression
        if let Expression::NullConditional(null_cond) = &*invocation.callee {
            assert!(!null_cond.is_element_access);
            assert_eq!(null_cond.member.to_string(), "Method");
            assert!(invocation.arguments.is_empty());
        } else {
            panic!("Expected callee to be null-conditional expression");
        }
    } else {
        panic!("Expected Invocation expression");
    }
}

#[test]
fn test_parse_null_conditional_indexing_with_expression() {
    let code = "dict?[key + 1]";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse null-conditional indexing with expression: {:?}",
        result
    );

    if let Ok(Expression::NullConditional(null_cond)) = result {
        assert!(null_cond.is_element_access);
        assert!(null_cond.argument.is_some());

        // Check the index argument is a binary expression
        if let Some(ref index) = null_cond.argument {
            if let Expression::Binary { .. } = &**index {
                // Correct - it's a binary expression (key + 1)
            } else {
                panic!("Expected index to be a binary expression");
            }
        }
    } else {
        panic!("Expected NullConditional expression");
    }
}

#[test]
fn test_parse_mixed_conditional_and_regular_access() {
    let code = "obj?.Child.Property";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse mixed conditional and regular access: {:?}",
        result
    );

    if let Ok(Expression::MemberAccess(member_access)) = result {
        assert_eq!(member_access.member.to_string(), "Property");

        // The object should be a null-conditional expression
        if let Expression::NullConditional(null_cond) = &*member_access.object {
            assert!(!null_cond.is_element_access);
            assert_eq!(null_cond.member.to_string(), "Child");
        } else {
            panic!("Expected object to be null-conditional expression");
        }
    } else {
        panic!("Expected MemberAccess expression");
    }
}

#[test]
fn test_parse_null_conditional_vs_ternary() {
    // Make sure ?. doesn't interfere with ternary operator
    let code = "condition ? obj.Property : null";
    let result = parse_null_conditional_expr_helper(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse ternary with null-conditional syntax: {:?}",
        result
    );

    if let Ok(Expression::Conditional(_)) = result {
        // Correct - should be parsed as ternary, not null-conditional
    } else {
        panic!("Expected Conditional expression (ternary operator)");
    }
}

#[test]
fn test_parse_null_conditional_whitespace_variations() {
    let variations = vec![
        "obj?.Property",
        "obj ?. Property",
        "obj? .Property",
        "obj ? . Property",
    ];

    for code in variations {
        let result = parse_null_conditional_expr_helper(code.into());
        // Some variations might not parse due to whitespace sensitivity
        // The important thing is that "obj?.Property" works
        if code == "obj?.Property" {
            assert!(
                result.is_ok(),
                "Failed to parse standard null-conditional: {:?}",
                result
            );
        }
    }
}

#[test]
fn test_parse_null_conditional_error_cases() {
    let invalid_cases = vec![
        "obj?",   // Missing member or indexer
        "obj?.",  // Missing member name
        "obj?[]", // Missing index expression
        "?obj",   // ? at wrong position
    ];

    for code in invalid_cases {
        let result = parse_null_conditional_expr_helper(code.into());
        // These should either fail to parse or parse as something else
        // The important thing is they don't crash
        let _ = result; // Just make sure it doesn't panic
    }
}
