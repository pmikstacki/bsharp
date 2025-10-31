// Tests for postfix unary expressions (x++, x--, null-forgiving)

use parser::expressions::parse_expression_spanned;
use syntax::expressions::{Expression, UnaryOperator};

#[test]
fn postfix_increment_basic() {
    let (rest, s) = parse_expression_spanned("x++".into()).expect("parse ok");
    let expr = s.node;
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::PostfixUnary { op, .. } => assert_eq!(op, UnaryOperator::Increment),
        other => panic!("expected PostfixUnary ++, got {:?}", other),
    }
}

#[test]
fn postfix_decrement_chained_with_member() {
    let (rest, s) = parse_expression_spanned("x--.ToString()".into()).expect("parse ok");
    let expr = s.node;
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Invocation(_) => {}
        other => panic!("expected Invocation after chaining, got {:?}", other),
    }
}

#[test]
fn null_forgiving_postfix() {
    let (rest, s) = parse_expression_spanned("x!".into()).expect("parse ok");
    let expr = s.node;
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::PostfixUnary { op, .. } => assert_eq!(op, UnaryOperator::NullForgiving),
        other => panic!("expected PostfixUnary ! (null-forgiving), got {:?}", other),
    }
}
