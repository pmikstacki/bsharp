use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::literal::Literal;
use bsharp::syntax::nodes::expressions::range_expression::{IndexExpression, RangeExpression};
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::parser::expressions::expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::UnaryOperator;

fn check_expr(input: &str, expected_expr: Expression) {
    let (_, expr) = parse_expression(input).unwrap_or_else(|e| panic!("Failed to parse expression '{}': {:?}", input, e));
    assert_eq!(expr, expected_expr, "Input: {}", input);
}

// Range Expression Tests
#[test]
fn test_range_full() {
    check_expr(
        r#"x..y"#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
            end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_start_only() {
    check_expr(
        r#"x.."#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
            end: None,
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_end_only() {
    check_expr(
        r#"..y"#,
        Expression::Range(Box::new(RangeExpression {
            start: None,
            end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_open() { // Just ".."
    check_expr(
        "..",
        Expression::Range(Box::new(RangeExpression {
            start: None,
            end: None,
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_with_literals() {
    check_expr(
        "1..5",
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Literal(Literal::Integer(1)))),
            end: Some(Box::new(Expression::Literal(Literal::Integer(5)))),
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_with_complex_expressions() {
    check_expr(
        r#"GetStart()..array.Length"#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Invocation(Box::new(
                bsharp::syntax::nodes::expressions::invocation_expression::InvocationExpression {
                    callee: Box::new(Expression::Variable(Identifier::new("GetStart"))),
                    arguments: vec![],
                }
            )))),
            end: Some(Box::new(Expression::MemberAccess(Box::new(
                bsharp::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                    object: Box::new(Expression::Variable(Identifier::new("array"))),
                    member: Identifier::new("Length"),
                }
            )))),
            is_inclusive: false,
        })),
    );
}

#[test]
fn test_range_whitespace_variations() {
    check_expr(
        r#"x .. y"#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
            end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
            is_inclusive: false,
        })),
    );
    check_expr(
        r#"x.. y"#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
            end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
            is_inclusive: false,
        })),
    );
     check_expr(
        r#"x ..y"#,
        Expression::Range(Box::new(RangeExpression {
            start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
            end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
            is_inclusive: false,
        })),
    );
}

// Index Expression Tests
#[test]
fn test_index_from_end_simple() {
    check_expr(
        "^1",
        Expression::Index(Box::new(IndexExpression {
            value: Box::new(Expression::Literal(Literal::Integer(1))),
        })),
    );
}

#[test]
fn test_index_from_end_variable() {
    check_expr(
        r#"^myIndex"#,
        Expression::Index(Box::new(IndexExpression {
            value: Box::new(Expression::Variable(Identifier::new("myIndex"))),
        })),
    );
}

#[test]
fn test_index_from_end_complex_expression() {
    check_expr(
        r#"^(arr.Length - 1)"#,
        Expression::Index(Box::new(IndexExpression {
            value: Box::new(Expression::Binary {
                left: Box::new(Expression::MemberAccess(Box::new(
                    bsharp::syntax::nodes::expressions::member_access_expression::MemberAccessExpression {
                        object: Box::new(Expression::Variable(Identifier::new("arr"))),
                        member: Identifier::new("Length"),
                    }
                ))),
                op: bsharp::syntax::nodes::expressions::BinaryOperator::Subtract,
                right: Box::new(Expression::Literal(Literal::Integer(1))),
            }),
        })),
    );
}

#[test]
fn test_index_from_end_whitespace() {
    check_expr(
        "^ 1",
        Expression::Index(Box::new(IndexExpression {
            value: Box::new(Expression::Literal(Literal::Integer(1))),
        })),
    );
}

// Interaction with other operators / Precedence
#[test]
fn test_range_in_array_indexer() {
    // Example: myArray[x..y]
    check_expr(
        r#"myArray[x..y]"#,
        Expression::Indexing(Box::new(bsharp::syntax::nodes::expressions::indexing_expression::IndexingExpression {
            target: Box::new(Expression::Variable(Identifier::new("myArray"))),
            index: Box::new(Expression::Range(Box::new(RangeExpression {
                start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
                end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
                is_inclusive: false,
            }))),
        })),
    );
}

#[test]
fn test_index_in_array_indexer() {
    // Example: myArray[^1]
    check_expr(
        r#"myArray[^1]"#,
        Expression::Indexing(Box::new(bsharp::syntax::nodes::expressions::indexing_expression::IndexingExpression {
            target: Box::new(Expression::Variable(Identifier::new("myArray"))),
            index: Box::new(Expression::Index(Box::new(IndexExpression {
                value: Box::new(Expression::Literal(Literal::Integer(1))),
            }))),
        })),
    );
}

#[test]
fn test_range_as_argument() {
    // Example: MyMethod(x..y)
    check_expr(
        r#"MyMethod(x..y)"#,
        Expression::Invocation(Box::new(bsharp::syntax::nodes::expressions::invocation_expression::InvocationExpression{
            callee: Box::new(Expression::Variable(Identifier::new("MyMethod"))),
            arguments: vec![
                Expression::Range(Box::new(RangeExpression {
                    start: Some(Box::new(Expression::Variable(Identifier::new("x")))),
                    end: Some(Box::new(Expression::Variable(Identifier::new("y")))),
                    is_inclusive: false,
                }))
            ]
        }))
    );
}

#[test]
fn test_index_as_argument() {
    // Example: MyMethod(^idx)
     check_expr(
        r#"MyMethod(^idx)"#,
        Expression::Invocation(Box::new(bsharp::syntax::nodes::expressions::invocation_expression::InvocationExpression{
            callee: Box::new(Expression::Variable(Identifier::new("MyMethod"))),
            arguments: vec![
                Expression::Index(Box::new(IndexExpression {
                    value: Box::new(Expression::Variable(Identifier::new("idx"))),
                }))
            ]
        }))
    );
}

// Error cases
#[test]
fn test_range_missing_operand_error() {
    // E.g. x.. or .. (if parsing requires an operand where it's optional by grammar but needed by context)
    // Current syntax logic makes operands optional for `..` and `x..` and `..y`
    // `..` alone is valid.
    // `x..` alone is valid.
    // `..y` alone is valid.
    // `x..y` is valid.
    // An error would be if we expected a range but got something else.
    // Test cases like `a[..]` which is valid, `a[1..]` valid, `a[..2]` valid.
    // This might be more about semantic analysis than pure parsing for some cases.
    // For now, let's test something clearly syntactically wrong.
    
    // Note: "x. .y" actually parses as member access "x." followed by ".y" which is invalid
    // So we'll test for the three dots case which should definitely be an error
    let result2 = parse_expression(r#"x...y"#); // Three dots
    match result2 {
        Ok((remaining, _expr)) => {
            // Should have remaining input ".y" which indicates incomplete parsing
            assert!(!remaining.trim().is_empty(), "Expected unparsed input for x...y but all input was consumed");
        }
        Err(_e) => {
            // This is also acceptable - if the syntax fails completely
        }
    }
    
    // Test for incomplete range that would cause issues in a larger context
    let result3 = parse_expression(r#"[.x]"#); // Invalid start to range in array indexer
    assert!(result3.is_err(), "Expected error for malformed range expression. Input: [.x]");
}

#[test]
fn test_index_missing_operand_error() {
    let result = parse_expression("^");
    assert!(result.is_err(), "Expected error for index operator without operand. Input: ^");
}

#[test]
fn test_index_operator_not_prefix() {
    // Example: x ^ y (this should be XOR, not index from end)
    check_expr(
        r#"x ^ y"#,
        Expression::Binary {
            left: Box::new(Expression::Variable(Identifier::new("x"))),
            op: bsharp::syntax::nodes::expressions::BinaryOperator::BitwiseXor,
            right: Box::new(Expression::Variable(Identifier::new("y"))),
        },
    );
}

#[test]
fn test_index_from_end_with_unary() {
    // Example: ^-1 - this should parse as ^(-1)
    // The UnaryOperator::Minus should apply to 1, then IndexFromEnd to that result.
    // parse_unary_expression_or_higher is recursive, so ^ will call it again for the operand.
    check_expr(
        "^-1",
        Expression::Index(Box::new(IndexExpression {
            value: Box::new(Expression::Unary {
                op: UnaryOperator::Minus,
                expr: Box::new(Expression::Literal(Literal::Integer(1))),
            }),
        })),
    );
} 