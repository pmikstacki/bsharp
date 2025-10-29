// Dedicated tests for parenthesized expressions

use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use parser::syntax::test_helpers::expect_ok;
use syntax::expressions::expression::Expression;
use syntax::expressions::literal::Literal;

#[test]
fn parenthesized_literal() {
    let (rest, expr) = parse_expression("(42)".into()).map(|(rest, s)| (rest, s.node)).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    assert_eq!(expr, Expression::Literal(Literal::Integer(42)));
}

#[test]
fn nested_parentheses() {
    let (rest, expr) = parse_expression("(((true)))".into()).map(|(rest, s)| (rest, s.node)).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    assert_eq!(expr, Expression::Literal(Literal::Boolean(true)));
}

#[test]
fn parenthesized_with_binary_inside() {
    let (rest, expr) = parse_expression("(1 + 2)".into()).map(|(rest, s)| (rest, s.node)).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Binary { .. } => {}
        other => panic!("expected Binary, got {:?}", other),
    }
}

#[test]
fn parenthesized_then_postfix_call() {
    let input = "(x).ToString()";
    let (rest, expr) = expect_ok(input, parse_expression(input.into()).map(|(rest, s)| (rest, s.node)));
    assert!(rest.trim().is_empty());
    match expr {
        Expression::Invocation(inv) => {
            let inv = *inv;
            assert!(inv.arguments.is_empty(), "expected no arguments");
            match *inv.callee {
                Expression::MemberAccess(ma) => {
                    let ma = *ma;
                    assert_eq!(ma.member.to_string(), "ToString");
                    match *ma.object {
                        Expression::Variable(id) => assert_eq!(id.to_string(), "x"),
                        other => panic!("expected Variable 'x' as callee object, got {:?}", other),
                    }
                }
                other => panic!("expected MemberAccess callee, got {:?}", other),
            }
        }
        other => panic!("expected Invocation, got {:?}", other),
    }
}
