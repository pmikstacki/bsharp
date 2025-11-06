use parser::expressions::lambda_expression_parser::parse_lambda_expression;
use syntax::expressions::expression::Expression;
use syntax::expressions::lambda_expression::LambdaParameterModifier;

fn parse(code: &str) -> Expression {
    let (rest, expr) = parse_lambda_expression(code.into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    expr
}

#[test]
fn lambda_simple_ref_modifier() {
    let expr = parse("ref x => x");
    if let Expression::Lambda(l) = expr {
        assert_eq!(l.parameters.len(), 1);
        assert!(matches!(l.parameters[0].modifier, Some(LambdaParameterModifier::Ref)));
        assert_eq!(l.parameters[0].name.to_string(), "x");
    } else { panic!("expected lambda") }
}

#[test]
fn lambda_simple_in_modifier() {
    let expr = parse("in x => x");
    if let Expression::Lambda(l) = expr {
        assert_eq!(l.parameters.len(), 1);
        assert!(matches!(l.parameters[0].modifier, Some(LambdaParameterModifier::In)));
        assert_eq!(l.parameters[0].name.to_string(), "x");
    } else { panic!("expected lambda") }
}

#[test]
fn lambda_simple_out_modifier() {
    let expr = parse("out x => 0");
    if let Expression::Lambda(l) = expr {
        assert_eq!(l.parameters.len(), 1);
        assert!(matches!(l.parameters[0].modifier, Some(LambdaParameterModifier::Out)));
        assert_eq!(l.parameters[0].name.to_string(), "x");
    } else { panic!("expected lambda") }
}
