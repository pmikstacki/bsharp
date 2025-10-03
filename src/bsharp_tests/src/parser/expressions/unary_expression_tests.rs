// Tests for prefix unary expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::nodes::expressions::UnaryOperator;
use syntax::nodes::expressions::expression::Expression;

fn assert_unary(op_str: &str, expected: UnaryOperator) {
    let (rest, expr) = parse_expression(op_str).expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Unary { op, .. } => assert_eq!(op, expected),
        other => panic!("expected Unary, got {:?}", other),
    }
}

#[test]
fn prefix_ops_basic() {
    assert_unary("+x", UnaryOperator::Plus);
    assert_unary("-x", UnaryOperator::Minus);
    assert_unary("!x", UnaryOperator::LogicalNot);
    assert_unary("~x", UnaryOperator::BitwiseNot);
}

#[test]
fn prefix_inc_dec() {
    let (rest, expr) = parse_expression("++x").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Unary { op, .. } => assert_eq!(op, UnaryOperator::Increment),
        other => panic!("expected prefix ++, got {:?}", other),
    }

    let (rest, expr) = parse_expression("--x").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Unary { op, .. } => assert_eq!(op, UnaryOperator::Decrement),
        other => panic!("expected prefix --, got {:?}", other),
    }
}

#[test]
fn address_of_and_pointer_indirection() {
    // These may require unsafe contexts semantically, but syntactically we parse them
    let (rest, expr) = parse_expression("&x").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Unary { op, .. } => assert_eq!(op, UnaryOperator::AddressOf),
        other => panic!("expected &, got {:?}", other),
    }

    let (rest, expr) = parse_expression("*x").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Unary { op, .. } => assert_eq!(op, UnaryOperator::PointerIndirection),
        other => panic!("expected *, got {:?}", other),
    }
}

#[test]
fn index_from_end_prefix() {
    let (rest, expr) = parse_expression("^1").expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Index(_) => {}
        other => panic!("expected Index (^1), got {:?}", other),
    }
}

#[test]
fn invalid_standalone_assignment_op_is_error() {
    // Ensure '-= 5' isn't misparsed as unary minus followed by '= 5'
    let result = parse_expression("-= 5");
    assert!(result.is_err());
}
