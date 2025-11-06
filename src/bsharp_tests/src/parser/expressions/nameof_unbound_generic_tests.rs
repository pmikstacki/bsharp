use parser::expressions::nameof_expression_parser::parse_nameof_expression;
use parser::expressions::primary_expression_parser::parse_expression_spanned as parse_expression;
use syntax::expressions::expression::Expression;

#[test]
fn nameof_list_unbound_generic() {
    let (rest, expr) = parse_nameof_expression("nameof(List<>)".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Nameof(_) => {}
        other => panic!("expected nameof, got {:?}", other),
    }
}

#[test]
fn nameof_dictionary_unbound_generic() {
    let (rest, expr) = parse_nameof_expression("nameof(Dictionary<,>)".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    match expr {
        Expression::Nameof(_) => {}
        other => panic!("expected nameof, got {:?}", other),
    }
}

#[test]
fn nameof_qualified_unbound_generic() {
    let (rest, expr) = parse_nameof_expression("nameof(System.Collections.Generic.List<>)".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    match expr { Expression::Nameof(_) => {} other => panic!("expected nameof, got {:?}", other), }
}

#[test]
fn nameof_unbound_ignored_in_full_expr_parser() {
    let (rest, s) = parse_expression("nameof(List<>)".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    match s.node { Expression::Nameof(_) => {} other => panic!("expected nameof, got {:?}", other), }
}
