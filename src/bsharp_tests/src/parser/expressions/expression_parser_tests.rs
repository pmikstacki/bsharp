use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::types::Type;
// Adjust if parse_expression is not public or in a submodule

#[test]
fn test_parse_simple_new_expression() {
    let input = "new Exception(\"Error\")";
    let result = parse_expression(input.into()).map(|(rest, s)| (rest, s.node));
    assert!(
        result.is_ok(),
        r#"Failed to parse 'new Exception("Error")': {:?}"#,
        result.err()
    );
    let (remaining, expr) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    match expr {
        Expression::New(boxed_new_expr) => {
            match &boxed_new_expr.target_type {
                Some(Type::Reference(ident)) => assert_eq!(ident.to_string(), "Exception"),
                _ => panic!("Expected Some(Type::Reference) for new expression type"),
            }
            assert_eq!(boxed_new_expr.arguments.len(), 1);
            match &boxed_new_expr.arguments[0] {
                Expression::Literal(Literal::String(s)) => assert_eq!(s, "Error"),
                _ => panic!("Expected string literal argument"),
            }
            assert!(boxed_new_expr.object_initializer.is_none());
            assert!(boxed_new_expr.collection_initializer.is_none());
        }
        _ => panic!("Expected Expression::New, got {:?}", expr),
    }
}

#[test]
fn test_parse_new_expression_no_args() {
    let input = "new Object()";
    let result = parse_expression(input.into()).map(|(rest, s)| (rest, s.node));
    assert!(
        result.is_ok(),
        "Failed to parse 'new Object()': {:?}",
        result.err()
    );
    let (remaining, expr) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    match expr {
        Expression::New(boxed_new_expr) => {
            match &boxed_new_expr.target_type {
                Some(Type::Reference(ident)) => assert_eq!(ident.to_string(), "Object"),
                _ => panic!("Expected Some(Type::Reference) for new expression type"),
            }
            assert!(boxed_new_expr.arguments.is_empty());
        }
        _ => panic!("Expected Expression::New, got {:?}", expr),
    }
}

#[test]
fn test_parse_new_expression_multiple_args() {
    let input = "new Data(42, \"test\", true)";
    let result = parse_expression(input.into()).map(|(rest, s)| (rest, s.node));
    assert!(
        result.is_ok(),
        r#"Failed to parse 'new Data(42, "test", true)': {:?}"#,
        result.err()
    );
    let (remaining, expr) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    match expr {
        Expression::New(boxed_new_expr) => {
            match &boxed_new_expr.target_type {
                Some(Type::Reference(ident)) => assert_eq!(ident.to_string(), "Data"),
                _ => panic!("Expected Some(Type::Reference) for new expression type"),
            }
            assert_eq!(boxed_new_expr.arguments.len(), 3);
            match &boxed_new_expr.arguments[0] {
                Expression::Literal(Literal::Integer(i)) => assert_eq!(*i, 42),
                _ => panic!("Expected integer literal for first argument"),
            }
            match &boxed_new_expr.arguments[1] {
                Expression::Literal(Literal::String(s)) => assert_eq!(s, "test"),
                _ => panic!("Expected string literal for second argument"),
            }
            match &boxed_new_expr.arguments[2] {
                Expression::Literal(Literal::Boolean(b)) => assert!(*b),
                _ => panic!("Expected boolean literal for third argument"),
            }
        }
        _ => panic!("Expected Expression::New, got {:?}", expr),
    }
}

#[test]
fn test_basic_identifier() {
    // ... tests ...
    // Assuming this test was incomplete or a placeholder,
    // if it had actual content, it should be moved here.
    // For now, keeping it as is from the original file.
}
