// Tests for parsing general expressions (ExpressionSyntax enum)

use bsharp::syntax::nodes::expressions::Expression;

use bsharp::parser::expressions::expression_parser::parse_expression as real_parse_expression;

fn parse_expression(code: &str) -> Result<Expression, String> {
    match real_parse_expression(code) {
        Ok((rest, expr)) if rest.trim().is_empty() => Ok(expr),
        Ok((rest, _)) => Err(format!("Unparsed input: {}", rest)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_binary_operator_precedence() {
    let expr = parse_expression("1 + 2 * 3").unwrap();
    // Should parse as 1 + (2 * 3)
    assert!(matches!(expr, Expression::Binary { .. }));
}

#[test]
fn test_assignment_expression() {
    let expr = parse_expression("x = 42").unwrap();
    use bsharp::syntax::nodes::expressions::Expression;
    match expr {
        Expression::Assignment(assign) => {
            assert!(matches!(*assign.target, Expression::Variable(_)));
            assert!(matches!(*assign.value, Expression::Literal(_)));
        }
        _ => panic!("Expected Expression::Assignment, got {:?}", expr),
    }
}

#[test]
fn test_member_access_and_invocation() {
    let expr = parse_expression("foo.Bar(1, 2)").unwrap();
    assert!(matches!(expr, Expression::Invocation(_)));
}

#[test]
fn test_indexing_and_postfix() {
    let expr = parse_expression("arr[0]++").unwrap();
    assert!(matches!(expr, Expression::PostfixUnary { .. }));
}

#[test]
fn test_object_initializer() {
    let expr = parse_expression("new Foo { X = 1, Y = 2 }").unwrap();
    assert!(matches!(expr, Expression::New(new_expr) if new_expr.object_initializer.is_some()));
}

// #[test]
// fn test_collection_initializer() {
//     // This tests implicitly typed arrays (new[] { ... }) which is not implemented yet
//     let expr = parse_expression("new[] { 1, 2, 3 }").unwrap();
//     assert!(matches!(expr, Expression::New(new_expr) if new_expr.collection_initializer.is_some()));
// }

#[test]
fn test_parse_integer_literal() {
    let input = "123";
    let expr = parse_expression(input).unwrap();
    assert_eq!(expr, Expression::Literal(bsharp::syntax::nodes::expressions::Literal::Integer(123)));
}

#[test]
fn test_parse_identifier() {
    let input = "myVariable";
    let expr = parse_expression(input).unwrap();
    assert_eq!(expr, Expression::Variable(bsharp::syntax::nodes::identifier::Identifier::new("myVariable")));
}

#[test]
fn test_parse_this_keyword() {
    let input = "this";
    let expr = parse_expression(input).unwrap();
    assert_eq!(expr, Expression::This);
}

#[test]
fn test_parse_parenthesized_expression() {
    let input = "(42)";
    let expr = parse_expression(input).unwrap();
    // Parenthesized expressions just resolve to the inner expression
    assert_eq!(expr, Expression::Literal(bsharp::syntax::nodes::expressions::Literal::Integer(42)));
}
