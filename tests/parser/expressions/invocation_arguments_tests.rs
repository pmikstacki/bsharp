use bsharp::parser::expressions::primary_expression_parser::parse_expression;
use bsharp::syntax::nodes::expressions::expression::Expression;
use bsharp::syntax::nodes::expressions::invocation_expression::{ArgumentModifier};
use bsharp::syntax::nodes::identifier::Identifier;

fn parse_ok(input: &str) -> Expression {
    let (rest, expr) = parse_expression(input).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn invocation_with_argument_modifiers() {
    let expr = parse_ok("Foo(ref x, out y, in z)");
    match expr {
        Expression::Invocation(inv) => {
            let args = &inv.arguments;
            assert_eq!(args.len(), 3);
            assert_eq!(args[0].modifier, Some(ArgumentModifier::Ref));
            assert_eq!(args[1].modifier, Some(ArgumentModifier::Out));
            assert_eq!(args[2].modifier, Some(ArgumentModifier::In));
            // Ensure names are None and exprs are variables
            assert!(args.iter().all(|a| a.name.is_none()));
            assert!(matches!(args[0].expr, Expression::Variable(Identifier{..})));
        }
        other => panic!("expected invocation, got {:?}", other),
    }
}

#[test]
fn invocation_with_named_arguments_and_calls() {
    let expr = parse_ok("Foo(p: 1, q: Get())");
    match expr {
        Expression::Invocation(inv) => {
            let args = &inv.arguments;
            assert_eq!(args.len(), 2);
            assert_eq!(args[0].name.as_ref().map(|n| n.name.clone()).as_deref(), Some("p"));
            assert!(matches!(args[0].expr, Expression::Literal(_)));
            assert_eq!(args[1].name.as_ref().map(|n| n.name.clone()).as_deref(), Some("q"));
            assert!(matches!(args[1].expr, Expression::Invocation(_)));
        }
        _ => panic!("expected invocation"),
    }
}
