// Tests for parsing stackalloc expressions

// use nom::error::{Error, ErrorKind};
// use bsharp::syntax::nodes::types::{PrimitiveType, Type};
use bsharp::parser::expressions::stackalloc_expression_parser::parse_stackalloc_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::types::{PrimitiveType, Type};
// use bsharp::parser::expressions::primary_expression_parser::parse_primary_expression;

fn parse_stackalloc_expr(code: &str) -> Result<Expression, String> {
    println!("Parsing stackalloc: '{}'", code);
    match parse_stackalloc_expression(code) {
        Ok((rest, expr)) => {
            println!("Success! Rest: '{}', Expr: {:?}", rest, expr);
            if rest.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", rest))
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(format!("Parse error: {:?}", e))
        }
    }
}

#[test]
fn test_parse_stackalloc_with_size() {
    let code = "stackalloc int[10]";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with size: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Int)));
        assert!(stackalloc.count.is_some());
        assert!(stackalloc.initializer.is_none());

        if let Some(Expression::Literal(Literal::Integer(10))) = stackalloc.count {
            // Expected
        } else {
            panic!("Expected integer literal 10 as count");
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_with_variable_size() {
    let code = "stackalloc byte[size]";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with variable size: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Byte)));
        assert!(stackalloc.count.is_some());
        assert!(stackalloc.initializer.is_none());

        if let Some(Expression::Variable(var)) = stackalloc.count {
            assert_eq!(var.name, "size");
        } else {
            panic!("Expected variable 'size' as count");
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_with_initializer() {
    let code = "stackalloc int[] { 1, 2, 3, 4 }";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with initializer: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Int)));
        assert!(stackalloc.count.is_none());
        assert!(stackalloc.initializer.is_some());

        let initializer = stackalloc.initializer.unwrap();
        assert_eq!(initializer.len(), 4);

        // Check that we have the expected literal values
        for (i, expected_val) in [1, 2, 3, 4].iter().enumerate() {
            if let Expression::Literal(Literal::Integer(val)) = &initializer[i] {
                assert_eq!(val, expected_val);
            } else {
                panic!("Expected integer literal at index {}", i);
            }
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_implicitly_typed() {
    let code = "stackalloc[] { 1, 2, 3 }";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse implicitly typed stackalloc: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert!(stackalloc.ty.is_none()); // Implicitly typed
        assert!(stackalloc.count.is_none());
        assert!(stackalloc.initializer.is_some());

        let initializer = stackalloc.initializer.unwrap();
        assert_eq!(initializer.len(), 3);
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_empty_initializer() {
    let code = "stackalloc int[] { }";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with empty initializer: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Int)));
        assert!(stackalloc.count.is_none());
        assert!(stackalloc.initializer.is_some());

        let initializer = stackalloc.initializer.unwrap();
        assert_eq!(initializer.len(), 0);
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_with_expression_size() {
    let code = "stackalloc double[count * 2]";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with expression size: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Double)));
        assert!(stackalloc.count.is_some());
        assert!(stackalloc.initializer.is_none());

        // The count should be a binary expression
        if let Some(Expression::Binary { .. }) = stackalloc.count {
            // Expected
        } else {
            panic!("Expected binary expression as count");
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_with_mixed_expressions() {
    let code = "stackalloc string[] { \"hello\", \"world\" }";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with mixed expressions: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::String)));
        assert!(stackalloc.initializer.is_some());

        let initializer = stackalloc.initializer.unwrap();
        assert_eq!(initializer.len(), 2);

        // Check string literals
        if let Expression::Literal(Literal::String(s)) = &initializer[0] {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected string literal 'hello'");
        }

        if let Expression::Literal(Literal::String(s)) = &initializer[1] {
            assert_eq!(s, "world");
        } else {
            panic!("Expected string literal 'world'");
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_pointer_type() {
    let code = "stackalloc char[buffer_size]";
    let result = parse_stackalloc_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse stackalloc with pointer type: {:?}",
        result
    );

    if let Ok(Expression::StackAlloc(stackalloc)) = result {
        assert_eq!(stackalloc.ty, Some(Type::Primitive(PrimitiveType::Char)));
        assert!(stackalloc.count.is_some());

        if let Some(Expression::Variable(var)) = stackalloc.count {
            assert_eq!(var.name, "buffer_size");
        } else {
            panic!("Expected variable 'buffer_size' as count");
        }
    } else {
        panic!("Expected StackAlloc expression");
    }
}

#[test]
fn test_parse_stackalloc_whitespace_variations() {
    let variations = vec![
        "stackalloc int[10]",
        "stackalloc  int  [  10  ]",
        "stackalloc int[] { 1, 2, 3 }",
        "stackalloc  int  [  ]  {  1  ,  2  ,  3  }",
        "stackalloc[] { 1, 2 }",
        "stackalloc  [  ]  {  1  ,  2  }",
    ];

    for code in variations {
        let result = parse_stackalloc_expr(code);
        assert!(
            result.is_ok(),
            "Failed to parse stackalloc with whitespace variation: '{}' -> {:?}",
            code,
            result
        );
    }
}

#[test]
fn test_parse_stackalloc_errors() {
    let invalid_cases = vec![
        "stackalloc",       // Missing type and size/initializer
        "stackalloc int",   // Missing size or initializer
        "stackalloc []",    // Missing type (when not using initializer)
        "stackalloc int[",  // Incomplete bracket
        "stackalloc int[]", // Missing initializer after empty brackets
    ];

    for code in invalid_cases {
        let result = parse_stackalloc_expr(code);
        assert!(result.is_err(), "Expected parsing to fail for: '{}'", code);
    }
}
