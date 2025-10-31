// Tests for parsing primary expressions (variable references, literals, parenthesized expressions)
use parser::expressions::parse_primary_expression_spanned;
use syntax::expressions::Expression;
use syntax::expressions::Literal;
use syntax::identifier::Identifier;

#[test]
fn test_parse_variable_reference() {
    let input = "foo";
    let (rest, s) = parse_primary_expression_spanned(input.into()).unwrap();
    let expr = s.node;
    match expr {
        Expression::Variable(id) => assert_eq!(id.to_string(), "foo"),
        other => panic!("Expected variable identifier, got {:?}", other),
    }
    assert!(rest.fragment().trim().is_empty());
}

#[test]
fn test_parse_literal_integer() {
    let input = "123";
    let (rest, s) = parse_primary_expression_spanned(input.into()).unwrap();
    let expr = s.node;
    assert_eq!(expr, Expression::Literal(Literal::Integer(123)));
    assert!(rest.fragment().trim().is_empty());
}

#[test]
fn test_parse_literal_string() {
    let input = r#""hello""#;
    let (rest, s) = parse_primary_expression_spanned(input.into()).unwrap();
    let expr = s.node;
    assert_eq!(
        expr,
        Expression::Literal(Literal::String("hello".to_string()))
    );
    assert!(rest.fragment().trim().is_empty());
}

#[test]
fn test_parse_parenthesized_expression() {
    let input = "( 42 )";
    let (rest, s) = parse_primary_expression_spanned(input.into()).unwrap();
    let expr = s.node;
    assert_eq!(expr, Expression::Literal(Literal::Integer(42)));
    assert!(rest.fragment().trim().is_empty());
}
