use parser::expressions::declarations::parameter_parser::parse_parameter;
use syntax::identifier::Identifier;
use syntax::types::{Parameter, ParameterModifier, PrimitiveType, Type};

fn parse_ok(src: &str) -> Parameter {
    let (rest, p) = parse_parameter(src).expect("parse");
    assert!(rest.trim().is_empty());
    p
}

#[test]
fn parses_scoped_ref_parameter_and_ignores_flag_in_ast() {
    let p = parse_ok("scoped ref int x");
    assert_eq!(p.modifier, Some(ParameterModifier::ScopedRef));
    assert_eq!(p.parameter_type, Type::Primitive(PrimitiveType::Int));
    assert_eq!(p.name, Identifier::new("x"));
}

#[test]
fn parses_scoped_in_and_out_parameters() {
    let p_in = parse_ok("scoped in T t");
    assert_eq!(p_in.modifier, Some(ParameterModifier::ScopedIn));
    assert_eq!(p_in.name, Identifier::new("t"));

    let p_out = parse_ok("scoped out string s");
    assert_eq!(p_out.modifier, Some(ParameterModifier::ScopedOut));
    assert_eq!(p_out.name, Identifier::new("s"));
}
