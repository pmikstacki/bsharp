// Tests for cast expressions and parens disambiguation

use parser::expressions::primary_expression_parser::parse_expression;
use syntax::expressions::expression::Expression;
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type};

#[test]
fn explicit_cast_basic() {
    let code = "(int)x";
    let (rest, expr) = parse_expression(code.into()).expect("parse ok");
    assert!(
        rest.fragment().trim().is_empty(),
        "unparsed: {}",
        rest.fragment()
    );
    match expr {
        Expression::Cast {
            expression,
            target_type,
        } => {
            assert!(matches!(target_type, Type::Primitive(PrimitiveType::Int)));
            assert!(matches!(*expression, Expression::Variable(_)));
        }
        other => panic!("expected Cast expression, got {:?}", other),
    }
}

#[test]
fn parens_not_cast_without_trailer() {
    let code = "(x)";
    let (rest, expr) = parse_expression(code.into()).expect("parse ok");
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Variable(id) => assert_eq!(id.to_string(), "x"),
        other => panic!("expected Variable, got {:?}", other),
    }
}
