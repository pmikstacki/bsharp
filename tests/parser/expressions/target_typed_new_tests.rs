use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::new_expression::{NewExpression, ObjectInitializerEntry};

fn parse_ok(input: &str) -> Expression {
    let (rest, expr) = parse_expression(input).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn test_target_typed_new_mixed_object_and_collection_initializer_should_error() {
    let res = parse_expression("new() { P = 1, 2 }");
    assert!(res.is_err(), "expected parse error for mixed initializer, got: {:?}", res);
}

#[test]
fn test_target_typed_new_empty_collection_initializer() {
    let expr = parse_ok("new() { }");
    match expr {
        Expression::New(new_expr_box) => {
            let new_expr = *new_expr_box;
            assert!(new_expr.ty.is_none());
            assert!(new_expr.object_initializer.is_none());
            assert!(new_expr.collection_initializer.is_some());
            assert_eq!(new_expr.collection_initializer.unwrap().len(), 0);
        }
        other => panic!("expected New expression, got {:?}", other),
    }
}

#[test]
fn test_target_typed_new_with_collection_initializer() {
    let expr = parse_ok("new() { 1, 2, 3 }");
    match expr {
        Expression::New(new_expr_box) => {
            let new_expr = *new_expr_box;
            assert!(new_expr.ty.is_none());
            assert!(new_expr.object_initializer.is_none());
            assert!(new_expr.collection_initializer.is_some());
            let elems = new_expr.collection_initializer.unwrap();
            assert_eq!(elems.len(), 3);
        }
        other => panic!("expected New expression, got {:?}", other),
    }
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
            match &inits[0] {
                ObjectInitializerEntry::Property { name, .. } => assert_eq!(name, "P"),
                other => panic!("expected Property initializer, got {:?}", other),
            }
        }
        _ => panic!("expected New expression"),
    }
}
