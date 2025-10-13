// Tests for parsing tuple expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;
use syntax::expressions::tuple_expression::{TupleElement, TupleExpression};
use syntax::identifier::Identifier;

fn check_tuple_expr(input: &str, expected_elements: Vec<TupleElement>) {
    let (_, expr) = parse_expression(input)
        .unwrap_or_else(|e| panic!("Failed to parse tuple expression '{}': {:?}", input, e));
    let expected_expr = Expression::Tuple(TupleExpression {
        elements: expected_elements,
    });
    assert_eq!(expr, expected_expr, "Input: {}", input);
}

#[test]
fn test_simple_tuple_expression() {
    check_tuple_expr(
        "(1, 2)",
        vec![
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(2)),
            },
        ],
    );
}

#[test]
fn test_tuple_with_more_elements() {
    check_tuple_expr(
        r#"("a", true, 3.0)"#,
        vec![
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::String("a".to_string())),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Boolean(true)),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Float(3.0)),
            },
        ],
    );
}

#[test]
fn test_tuple_with_named_elements() {
    check_tuple_expr(
        r#"(x: 1, y: "hello")"#,
        vec![
            TupleElement {
                name: Some(Identifier::new("x")),
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: Some(Identifier::new("y")),
                value: Expression::Literal(Literal::String("hello".to_string())),
            },
        ],
    );
}

#[test]
fn test_tuple_with_mixed_named_and_unnamed_elements() {
    check_tuple_expr(
        r#"(1, name: "test", 3)"#,
        vec![
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: Some(Identifier::new("name")),
                value: Expression::Literal(Literal::String("test".to_string())),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(3)),
            },
        ],
    );
}

#[test]
fn test_tuple_with_trailing_comma() {
    check_tuple_expr(
        "(1, 2, )",
        vec![
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(2)),
            },
        ],
    );
}

#[test]
fn test_tuple_with_named_elements_and_trailing_comma() {
    check_tuple_expr(
        "(a: 1, b: 2, )",
        vec![
            TupleElement {
                name: Some(Identifier::new("a")),
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: Some(Identifier::new("b")),
                value: Expression::Literal(Literal::Integer(2)),
            },
        ],
    );
}

#[test]
fn test_tuple_with_expressions_as_elements() {
    check_tuple_expr(
        "(1 + 2, MyVar)",
        vec![
            TupleElement {
                name: None,
                value: Expression::Binary {
                    left: Box::new(Expression::Literal(Literal::Integer(1))),
                    op: syntax::expressions::BinaryOperator::Add,
                    right: Box::new(Expression::Literal(Literal::Integer(2))),
                },
            },
            TupleElement {
                name: None,
                value: Expression::Variable(Identifier::new("MyVar")),
            },
        ],
    );
}

#[test]
fn test_nested_tuple_expression() {
    check_tuple_expr(
        "((1, 2), b: (x: 3, 4))",
        vec![
            TupleElement {
                name: None,
                value: Expression::Tuple(TupleExpression {
                    elements: vec![
                        TupleElement {
                            name: None,
                            value: Expression::Literal(Literal::Integer(1)),
                        },
                        TupleElement {
                            name: None,
                            value: Expression::Literal(Literal::Integer(2)),
                        },
                    ],
                }),
            },
            TupleElement {
                name: Some(Identifier::new("b")),
                value: Expression::Tuple(TupleExpression {
                    elements: vec![
                        TupleElement {
                            name: Some(Identifier::new("x")),
                            value: Expression::Literal(Literal::Integer(3)),
                        },
                        TupleElement {
                            name: None,
                            value: Expression::Literal(Literal::Integer(4)),
                        },
                    ],
                }),
            },
        ],
    );
}

#[test]
fn test_tuple_whitespace_variations() {
    check_tuple_expr(
        "(   1   ,   2   )",
        vec![
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: None,
                value: Expression::Literal(Literal::Integer(2)),
            },
        ],
    );
    check_tuple_expr(
        "(a:1,b:2)",
        vec![
            TupleElement {
                name: Some(Identifier::new("a")),
                value: Expression::Literal(Literal::Integer(1)),
            },
            TupleElement {
                name: Some(Identifier::new("b")),
                value: Expression::Literal(Literal::Integer(2)),
            },
        ],
    );
}

// Error case: Single element is not a tuple, should be parsed as parenthesized expression
#[test]
fn test_single_element_not_a_tuple() {
    let result = parse_expression("(1)");
    assert!(
        result.is_ok(),
        "Expected parenthesized expression, not tuple error. Input: (1)"
    );
    let expr = result.unwrap().1;
    assert_eq!(
        expr,
        Expression::Literal(Literal::Integer(1)),
        "Expected parenthesized literal 1. Input: (1)"
    );

    let result_named = parse_expression("(a: 1)");
    assert!(
        result_named.is_err(),
        "Expected error for named single element tuple. Input: (a:1)"
    );
}

#[test]
fn test_empty_tuple_is_error() {
    // C# does not support empty tuples like ()
    // It would be a method call with zero arguments if `()` was a variable of delegate type.
    // Otherwise, it's a parser error. Our syntax should error.
    let result = parse_expression("()");
    assert!(result.is_err(), "Expected error for empty tuple. Input: ()");
}

#[test]
fn test_tuple_with_just_comma_is_error() {
    let result = parse_expression("(,)");
    assert!(
        result.is_err(),
        "Expected error for tuple with just comma. Input: (,)"
    );
}

#[test]
fn test_tuple_missing_closing_paren_is_error() {
    let result = parse_expression("(1, 2");
    assert!(
        result.is_err(),
        "Expected error for missing closing paren. Input: (1, 2"
    );
}

#[test]
fn test_tuple_missing_comma_is_error() {
    let result = parse_expression("(1 2)");
    assert!(
        result.is_err(),
        "Expected error for missing comma. Input: (1 2)"
    );
}

#[test]
fn test_tuple_invalid_element_name() {
    // e.g. (1: x, 2) - number cannot be a name
    let result = parse_expression("(1: x, 2)");
    assert!(
        result.is_err(),
        "Expected error for invalid element name. Input: (1: x, 2)"
    );
}
