use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::new_expression::NewExpression;

fn parse_ok(input: &str) -> Expression {
    let (rest, expr) = parse_expression(input).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn test_target_typed_new_simple() {
    let expr = parse_ok("new()");
    match expr {
        Expression::New(new_expr_box) => {
            let NewExpression { ty, arguments, object_initializer, collection_initializer } = *new_expr_box;
            assert!(ty.is_none());
            assert!(arguments.is_empty());
            assert!(object_initializer.is_none());
            assert!(collection_initializer.is_none());
        }
        other => panic!("expected New expression, got {:?}", other),
    }
}

#[test]
fn test_target_typed_new_with_object_initializer() {
    let expr = parse_ok("new() { P = 1 }");
    match expr {
        Expression::New(new_expr_box) => {
            let NewExpression { ty, object_initializer, .. } = *new_expr_box;
            assert!(ty.is_none());
            assert!(object_initializer.is_some());
            let inits = object_initializer.unwrap();
            assert_eq!(inits.len(), 1);
            assert_eq!(inits[0].0, "P");
        }
        _ => panic!("expected New expression"),
    }
}
