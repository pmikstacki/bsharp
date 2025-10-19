use parser::types::type_parser::parse_type_expression;
use syntax::types::Type;

#[test]
fn parses_ref_readonly_return_type() {
    let (rest, ty) = parse_type_expression("ref readonly int".into()).expect("parse");
    assert!(rest.trim().is_empty());
    match ty {
        Type::RefReadOnlyReturn(inner) => match *inner {
            Type::Primitive(p) => assert_eq!(format!("{:?}", p), "Int"),
            other => panic!("expected primitive int, got {:?}", other),
        },
        other => panic!("expected RefReadOnlyReturn, got {:?}", other),
    }
}
