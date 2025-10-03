use parser::expressions::primary_expression_parser::parse_expression;
use syntax::nodes::expressions::expression::{Expression, WithInitializerEntry};
use syntax::nodes::identifier::Identifier;

fn parse_expr_ok(src: &str) -> Expression {
    let (rest, expr) = parse_expression(src).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn test_simple_with_expression() {
    let expr = parse_expr_ok("x with { P = 1, Q = x.Q + 2 }");
    match expr {
        Expression::With {
            target,
            initializers,
        } => {
            // target should be variable x
            assert!(
                matches!(*target, Expression::Variable(Identifier { ref name }) if name == "x")
            );
            // two initializers
            assert_eq!(initializers.len(), 2);
            match &initializers[0] {
                WithInitializerEntry::Property { name, .. } => assert_eq!(name, "P"),
                other => panic!("expected Property init, got {:?}", other),
            }
            match &initializers[1] {
                WithInitializerEntry::Property { name, .. } => assert_eq!(name, "Q"),
                other => panic!("expected Property init, got {:?}", other),
            }
        }
        other => panic!("expected with expression, got {:?}", other),
    }
}
