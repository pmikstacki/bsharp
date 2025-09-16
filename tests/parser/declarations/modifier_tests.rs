use bsharp::parser::declarations::modifier_parser::{parse_modifiers, parse_modifiers_for_decl_type};
use bsharp::syntax::nodes::declarations::Modifier;

#[test]
fn test_parse_single_modifier() {
    let result = parse_modifiers("public").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Public]);
    
    let result = parse_modifiers("static").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Static]);
    
    let result = parse_modifiers("private").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Private]);
}

#[test]
fn test_parse_multiple_modifiers() {
    let result = parse_modifiers("public static").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Public, Modifier::Static]);

    let result_ws = parse_modifiers("public static ").unwrap();
    assert_eq!(result_ws.0, "");
    assert_eq!(result_ws.1, vec![Modifier::Public, Modifier::Static]);

    let result = parse_modifiers("readonly private").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Private, Modifier::Readonly]);

    let result_ws = parse_modifiers("readonly private ").unwrap();
    assert_eq!(result_ws.0, "");
    assert_eq!(result_ws.1, vec![Modifier::Private, Modifier::Readonly]);

    let result = parse_modifiers("virtual internal protected").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![Modifier::Internal, Modifier::Protected, Modifier::Virtual]);

    let result_ws = parse_modifiers("virtual internal protected ").unwrap();
    assert_eq!(result_ws.0, "");
    assert_eq!(result_ws.1, vec![Modifier::Internal, Modifier::Protected, Modifier::Virtual]);
}

#[test]
fn test_parse_no_modifiers() {
    let result = parse_modifiers("").unwrap();
    assert_eq!(result.0, "");
    assert_eq!(result.1, vec![]);
}

#[test]
fn test_parse_modifiers_for_property() {
    let result = parse_modifiers_for_decl_type("public static", "property").unwrap();
    assert_eq!(result.0, "");
}
