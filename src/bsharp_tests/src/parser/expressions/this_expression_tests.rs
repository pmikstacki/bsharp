// Tests for `this` expression

use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use syntax::expressions::expression::Expression;

#[test]
fn this_basic() {
    let (rest, expr) = parse_expression("this".into()).map(|(rest, s)| (rest, s.node)).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    assert!(matches!(expr, Expression::This));
}

#[test]
fn this_member_access() {
    let (rest, expr) = parse_expression("this.ToString()".into()).map(|(rest, s)| (rest, s.node)).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Invocation(_) => {}
        other => panic!("expected Invocation on this, got {:?}", other),
    }
}
