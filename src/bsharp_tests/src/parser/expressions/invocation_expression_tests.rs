// Tests for parsing invocation expressions (modifiers and named args)

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::invocation_expression::ArgumentModifier;

fn parse_invocation_ok(
    code: &str,
) -> syntax::expressions::invocation_expression::InvocationExpression {
    let (rest, expr) = parse_expression(code.into()).expect("parse expression");
    assert!(
        rest.fragment().trim().is_empty(),
        "Unparsed rest: '{}'",
        rest.fragment()
    );
    match expr {
        Expression::Invocation(inv) => *inv,
        other => panic!("Expected Invocation, got: {:?}", other),
    }
}

#[test]
fn invocation_with_modifiers() {
    let inv = parse_invocation_ok("foo(ref x, out y, in z)");
    assert_eq!(inv.arguments.len(), 3);
    assert!(matches!(
        inv.arguments[0].modifier,
        Some(ArgumentModifier::Ref)
    ));
    assert!(matches!(
        inv.arguments[1].modifier,
        Some(ArgumentModifier::Out)
    ));
    assert!(matches!(
        inv.arguments[2].modifier,
        Some(ArgumentModifier::In)
    ));
}

#[test]
fn invocation_with_named_arguments_and_calls() {
    let inv = parse_invocation_ok("foo(p: 1, q: bar())");
    assert_eq!(inv.arguments.len(), 2);
    assert_eq!(
        inv.arguments[0]
            .name
            .as_ref()
            .map(|n| n.to_string())
            .as_deref(),
        Some("p")
    );
    assert_eq!(
        inv.arguments[1]
            .name
            .as_ref()
            .map(|n| n.to_string())
            .as_deref(),
        Some("q")
    );
    // Ensure second argument is an invocation expression
    if let Expression::Invocation(_) = inv.arguments[1].expr { /* ok */
    } else {
        panic!("expected invocation in q:");
    }
}
