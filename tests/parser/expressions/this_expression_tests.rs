// Tests for `this` expression

use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;

#[test]
fn this_basic() {
    let (rest, expr) = parse_expression("this").expect("parse ok");
    assert!(rest.trim().is_empty());
    assert!(matches!(expr, Expression::This));
}

#[test]
fn this_member_access() {
    let (rest, expr) = parse_expression("this.ToString()").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Invocation(_) => {}
        other => panic!("expected Invocation on this, got {:?}", other),
    }
}
