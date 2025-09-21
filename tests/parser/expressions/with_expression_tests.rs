use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::identifier::Identifier;

fn parse_expr_ok(src: &str) -> Expression {
    let (rest, expr) = parse_expression(src).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn test_simple_with_expression() {
    let expr = parse_expr_ok("x with { P = 1, Q = x.Q + 2 }");
    match expr {
        Expression::With { target, initializers } => {
            // target should be variable x
            assert!(matches!(*target, Expression::Variable(Identifier { ref name }) if name == "x"));
            // two initializers
            assert_eq!(initializers.len(), 2);
            assert_eq!(initializers[0].0, "P");
            assert_eq!(initializers[1].0, "Q");
        }
        other => panic!("expected with expression, got {:?}", other),
    }
}
