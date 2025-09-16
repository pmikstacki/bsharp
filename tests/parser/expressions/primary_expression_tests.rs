// Tests for parsing primary expressions (variable references, literals, parenthesized expressions)
use bsharp::syntax::nodes::expressions::Expression;
use bsharp::syntax::nodes::identifier::Identifier;
use bsharp::syntax::nodes::expressions::Literal;
use bsharp::parser::expressions::primary_parser::parse_primary_expression;

#[test]
fn test_parse_variable_reference() {
    let input = "foo";
    let (rest, expr) = parse_primary_expression(input).unwrap();
    assert_eq!(expr, Expression::Variable(Identifier { name: "foo".to_string() }));
    assert_eq!(rest, "");
}

#[test]
fn test_parse_literal_integer() {
    let input = "123";
    let (rest, expr) = parse_primary_expression(input).unwrap();
    assert_eq!(expr, Expression::Literal(Literal::Integer(123)));
    assert_eq!(rest, "");
}

#[test]
fn test_parse_literal_string() {
    let input = r#""hello""#;
    let (rest, expr) = parse_primary_expression(input).unwrap();
    assert_eq!(expr, Expression::Literal(Literal::String("hello".to_string())));
    assert_eq!(rest, "");
}

#[test]
fn test_parse_parenthesized_expression() {
    let input = "( 42 )";
    let (rest, expr) = parse_primary_expression(input).unwrap();
    assert_eq!(expr, Expression::Literal(Literal::Integer(42)));
    assert_eq!(rest, "");
}
