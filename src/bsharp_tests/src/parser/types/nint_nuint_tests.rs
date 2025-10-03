use parser::types::type_parser::parse_type_expression;
use syntax::nodes::types::{PrimitiveType, Type};

fn parse_type_ok(src: &str) -> Type {
    let (rest, ty) = parse_type_expression(src).expect("parse ok");
    assert!(rest.trim().is_empty(), "unparsed: {}", rest);
    ty
}

#[test]
fn test_nint_type() {
    let ty = parse_type_ok("nint");
    assert_eq!(ty, Type::Primitive(PrimitiveType::NInt));
}

#[test]
fn test_nuint_type() {
    let ty = parse_type_ok("nuint");
    assert_eq!(ty, Type::Primitive(PrimitiveType::NUInt));
}
