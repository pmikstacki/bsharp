// Tests for switch expressions

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::expression::{Expression, SwitchExpression};

#[test]
fn basic_switch_expression() {
    let code = "x switch { 1 => 10, _ => 20 }";
    let (rest, expr) = parse_expression(code).expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::SwitchExpression(se) => {
            let SwitchExpression { expression, arms } = *se;
            // scrutinee can be a variable expression `x`
            assert!(matches!(expression, Expression::Variable(_)));
            assert_eq!(arms.len(), 2);
        }
        other => panic!("expected SwitchExpression, got {:?}", other),
    }
}

#[test]
fn switch_when_clause() {
    let code = "x switch { > 0 when (1 + 1) == 2 => 1, _ => 0 }";
    let (rest, expr) = parse_expression(code).expect("parse ok");
    assert!(rest.trim().is_empty());
    match expr {
        Expression::SwitchExpression(se) => {
            let se = *se;
            assert_eq!(se.arms.len(), 2);
        }
        other => panic!("expected SwitchExpression, got {:?}", other),
    }
}
