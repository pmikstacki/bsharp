// Tests for parsing await expressions

use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;

fn parse_await_expr(code: &str) -> Result<Expression, String> {
    match parse_expression(code) {
        Ok((remaining, expr)) => {
            if remaining.trim().is_empty() {
                Ok(expr)
            } else {
                Err(format!("Unparsed input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

#[test]
fn test_parse_simple_await_expr() {
    let code = "await task";
    let result = parse_await_expr(code);
    assert!(result.is_ok(), "Failed to parse simple await: {:?}", result);

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a variable
        assert!(matches!(&*await_expr.expr, Expression::Variable(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_method_call() {
    let code = "await SomeMethodAsync()";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await method call: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a method invocation
        assert!(matches!(&*await_expr.expr, Expression::Invocation(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_member_access() {
    let code = "await obj.MethodAsync()";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await member access: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a method invocation
        assert!(matches!(&*await_expr.expr, Expression::Invocation(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_new_expression() {
    let code = "await new Task(() => { })";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await new expression: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a new expression
        assert!(matches!(&*await_expr.expr, Expression::New(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_parenthesized() {
    let code = "await (someTask)";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await parenthesized: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a variable (parentheses removed during parsing)
        assert!(matches!(&*await_expr.expr, Expression::Variable(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_nested_await() {
    let code = "await await GetTaskAsync()";
    let result = parse_await_expr(code);
    assert!(result.is_ok(), "Failed to parse nested await: {:?}", result);

    if let Ok(Expression::Await(outer_await)) = result {
        // The inner expression should be another await
        assert!(matches!(&*outer_await.expr, Expression::Await(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_with_indexing() {
    let code = "await tasks[0]";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await with indexing: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be indexing
        assert!(matches!(&*await_expr.expr, Expression::Indexing(_)));
    } else {
        panic!("Expected Await expression");
    }
}

#[test]
fn test_parse_await_complex_expression() {
    let code = "await obj.GetServiceAsync().ConfigureAwait(false)";
    let result = parse_await_expr(code);
    assert!(
        result.is_ok(),
        "Failed to parse await complex expression: {:?}",
        result
    );

    if let Ok(Expression::Await(await_expr)) = result {
        // The inner expression should be a method invocation (ConfigureAwait call)
        assert!(matches!(&*await_expr.expr, Expression::Invocation(_)));
    } else {
        panic!("Expected Await expression");
    }
}
