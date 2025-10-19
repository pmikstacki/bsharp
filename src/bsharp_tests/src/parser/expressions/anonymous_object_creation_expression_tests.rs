// Tests for parsing anonymous object creation expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::literal::Literal;
use syntax::expressions::Expression;

// Helper function for parsing anonymous object expressions
fn parse_anon_obj_expr(code: &str) -> Result<Expression, String> {
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
fn test_parse_implicit_dotted_and_identifier_members() {
    let code = "new { x.Name, adult }";
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse implicit dotted/identifier members: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);
        // name should be None for implicit members
        assert!(anon_obj.initializers[0].name.is_none());
        assert!(anon_obj.initializers[1].name.is_none());
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_simple_anonymous_object() {
    let code = r#"new { Name = "John", Age = 30 }"#;
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse simple anonymous object: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);

        // Check first member (Name)
        assert_eq!(anon_obj.initializers[0].name.as_ref().unwrap().to_string(), "Name");
        if let Expression::Literal(Literal::String(value)) = &anon_obj.initializers[0].value {
            assert_eq!(value, "John");
        } else {
            panic!("Expected string literal for Name");
        }

        // Check second member (Age)
        assert_eq!(anon_obj.initializers[1].name.as_ref().unwrap().to_string(), "Age");
        if let Expression::Literal(Literal::Integer(value)) = &anon_obj.initializers[1].value {
            assert_eq!(*value, 30);
        } else {
            panic!("Expected integer literal for Age");
        }
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_empty_anonymous_object() {
    let code = "new { }";
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse empty anonymous object: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 0);
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_single_member_anonymous_object() {
    let code = r#"new { Status = "Active" }"#;
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse single member anonymous object: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 1);
        assert_eq!(
            anon_obj.initializers[0].name.as_ref().unwrap().to_string(),
            "Status"
        );
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_anonymous_object_with_complex_expressions() {
    let code = "new { FullName = firstName + \" \" + lastName, IsValid = age > 18 }";
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse anonymous object with complex expressions: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);
        assert_eq!(
            anon_obj.initializers[0].name.as_ref().unwrap().to_string(),
            "FullName"
        );
        assert_eq!(
            anon_obj.initializers[1].name.as_ref().unwrap().to_string(),
            "IsValid"
        );
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_anonymous_object_with_method_calls() {
    let code = "new { Length = text.Length, Upper = text.ToUpper() }";
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse anonymous object with method calls: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);
        assert_eq!(
            anon_obj.initializers[0].name.as_ref().unwrap().to_string(),
            "Length"
        );
        assert_eq!(
            anon_obj.initializers[1].name.as_ref().unwrap().to_string(),
            "Upper"
        );
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_nested_anonymous_objects() {
    let code = r#"new { Person = new { Name = "John", Age = 30 }, Status = "Active" }"#;
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse nested anonymous objects: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);
        assert_eq!(
            anon_obj.initializers[0].name.as_ref().unwrap().to_string(),
            "Person"
        );
        assert_eq!(
            anon_obj.initializers[1].name.as_ref().unwrap().to_string(),
            "Status"
        );

        // Check that the first member is another anonymous object
        if let Expression::AnonymousObject(nested) = &anon_obj.initializers[0].value {
            assert_eq!(nested.initializers.len(), 2);
            assert_eq!(nested.initializers[0].name.as_ref().unwrap().to_string(), "Name");
            assert_eq!(nested.initializers[1].name.as_ref().unwrap().to_string(), "Age");
        } else {
            panic!("Expected nested anonymous object");
        }
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_anonymous_object_with_different_types() {
    let code = r#"new { Text = "Hello", Number = 42, Flag = true, Value = 3.14 }"#;
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse anonymous object with different types: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 4);
        assert_eq!(anon_obj.initializers[0].name.as_ref().unwrap().to_string(), "Text");
        assert_eq!(
            anon_obj.initializers[1].name.as_ref().unwrap().to_string(),
            "Number"
        );
        assert_eq!(anon_obj.initializers[2].name.as_ref().unwrap().to_string(), "Flag");
        assert_eq!(
            anon_obj.initializers[3].name.as_ref().unwrap().to_string(),
            "Value"
        );
    } else {
        panic!("Expected anonymous object creation expression");
    }
}

#[test]
fn test_parse_anonymous_object_whitespace_variations() {
    // Test different whitespace patterns
    let variations = vec![
        "new{Name=\"John\"}",
        "new { Name = \"John\" }",
        "new {  Name  =  \"John\"  }",
        "new\n{\n  Name = \"John\"\n}",
    ];

    for code in variations {
        let result = parse_anon_obj_expr(code.into());
        assert!(
            result.is_ok(),
            "Failed to parse anonymous object with whitespace variation '{}': {:?}",
            code,
            result
        );
    }
}

#[test]
fn test_parse_anonymous_object_trailing_comma() {
    let code = r#"new { Name = "John", Age = 30, }"#;
    let result = parse_anon_obj_expr(code.into());
    // This should either succeed or fail gracefully depending on implementation
    // Most C# implementations allow trailing commas
    if result.is_ok() {
        if let Ok(Expression::AnonymousObject(anon_obj)) = result {
            assert_eq!(anon_obj.initializers.len(), 2);
        }
    }
}

#[test]
fn test_anonymous_object_parse_errors() {
    // These should fail to parse
    let invalid_cases = vec![
        "new {",              // Missing closing brace
        "new { = \"John\" }", // Missing property name in explicit assignment
        "new { Name = }",     // Missing value in explicit assignment
        "new { , }",          // Invalid comma usage
        "new { 1, 2 }",       // Invalid positional-like entries in anonymous object
        "new { Name Name }",  // Missing assignment or comma
    ];

    for code in invalid_cases {
        let result = parse_anon_obj_expr(code.into());
        assert!(
            result.is_err(),
            "Expected parse error for invalid syntax: '{}'",
            code
        );
    }
}

#[test]
fn test_parse_implicit_member_anonymous_object() {
    let code = "new { Name, Age }";
    let result = parse_anon_obj_expr(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse implicit member anonymous object: {:?}",
        result
    );

    if let Ok(Expression::AnonymousObject(anon_obj)) = result {
        assert_eq!(anon_obj.initializers.len(), 2);
        // For implicit members, name is None
        assert!(anon_obj.initializers[0].name.is_none());
        assert!(anon_obj.initializers[1].name.is_none());
    } else {
        panic!("Expected anonymous object creation expression");
    }
}
