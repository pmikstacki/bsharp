// Tests for anonymous method expressions

use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::lambda_expression::LambdaBody;

fn parse(code: &str) -> Expression {
    let (rest, expr) = parse_expression(code).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn anonymous_method_basic() {
    let expr = parse("delegate(int x) { return x; }");
    match expr {
        Expression::AnonymousMethod(m) => {
            let m = *m;
            assert_eq!(m.parameters.len(), 1);
            assert!(matches!(m.body, LambdaBody::Block(_)));
            assert!(!m.is_async);
        }
        other => panic!("expected AnonymousMethod, got {:?}", other),
    }
}

#[test]
fn anonymous_method_no_params_expression_body() {
    let expr = parse("delegate { return 42; }");
    match expr {
        Expression::AnonymousMethod(m) => {
            let m = *m;
            assert_eq!(m.parameters.len(), 0);
            // Anonymous method with braces is a block body
            assert!(matches!(m.body, LambdaBody::Block(_)));
        }
        other => panic!("expected AnonymousMethod, got {:?}", other),
    }
}
