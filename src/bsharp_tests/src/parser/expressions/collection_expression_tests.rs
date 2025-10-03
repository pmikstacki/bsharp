use parser::expressions::primary_expression_parser::parse_expression;
use syntax::nodes::expressions::expression::{CollectionElement, Expression};
use syntax::nodes::expressions::literal::Literal;
use syntax::nodes::identifier::Identifier;

fn parse_ok(input: &str) -> Expression {
    let (rest, expr) = parse_expression(input).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    expr
}

#[test]
fn test_basic_collection_expression() {
    let expr = parse_ok("[1, 2, 3]");
    match expr {
        Expression::Collection(elems) => {
            assert_eq!(elems.len(), 3);
            assert!(matches!(
                elems[0],
                CollectionElement::Expr(Expression::Literal(Literal::Integer(1)))
            ));
        }
        other => panic!("expected collection expression, got {:?}", other),
    }
}

#[test]
fn test_collection_with_spread_and_binary() {
    let expr = parse_ok("[1, 2, ..other, x + 3]");
    match expr {
        Expression::Collection(elems) => {
            assert_eq!(elems.len(), 4);
            // third is spread
            match &elems[2] {
                CollectionElement::Spread(Expression::Variable(Identifier { name })) => {
                    assert_eq!(name, "other")
                }
                _ => panic!("expected spread element"),
            }
        }
        _ => panic!("expected collection expression"),
    }
}

#[test]
fn test_empty_collection_expression() {
    let expr = parse_ok("[]");
    match expr {
        Expression::Collection(elems) => {
            assert_eq!(elems.len(), 0);
        }
        _ => panic!("expected collection expression"),
    }
}
