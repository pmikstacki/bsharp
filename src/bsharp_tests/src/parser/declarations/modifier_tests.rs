use parser::expressions::declarations::modifier_parser::{
    parse_modifiers, parse_modifiers_for_decl_type,
};
use syntax::declarations::Modifier;

fn expect_modifiers(input: &str, expected_modifiers: Vec<Modifier>) {
    let result = parse_modifiers(input.into());
    assert!(result.is_ok());
    let (remaining, modifiers) = result.unwrap();
    assert_eq!(remaining.fragment().to_string(), "");
    assert_eq!(modifiers, expected_modifiers);
}

fn expect_modifiers_decl(modifiers: &str, name: &str, expected_modifiers: Vec<Modifier>) {
    let result = parse_modifiers_for_decl_type(modifiers.into(), name);
    assert!(result.is_ok());
    let (remaining, modifiers) = result.unwrap();
    assert_eq!(remaining.fragment().to_string(), "");
    assert_eq!(modifiers, expected_modifiers);
}
#[test]
fn test_parse_single_modifier() {
    expect_modifiers("public", vec![Modifier::Public]);

    expect_modifiers("static", vec![Modifier::Static]);

    expect_modifiers("extern", vec![Modifier::Extern]);

    expect_modifiers("unsafe", vec![Modifier::Unsafe]);

    expect_modifiers("private", vec![Modifier::Private]);
}

#[test]
fn test_parse_multiple_modifiers() {
    expect_modifiers("public static", vec![Modifier::Public, Modifier::Static]);
    expect_modifiers("public static ", vec![Modifier::Public, Modifier::Static]);
    expect_modifiers(
        "readonly private",
        vec![Modifier::Readonly, Modifier::Private],
    );
    expect_modifiers(
        "readonly private ",
        vec![Modifier::Readonly, Modifier::Private],
    );

    expect_modifiers(
        "virtual internal protected",
        vec![Modifier::Virtual, Modifier::Internal, Modifier::Protected],
    );
    expect_modifiers(
        "virtual internal protected ",
        vec![Modifier::Virtual, Modifier::Internal, Modifier::Protected],
    );
}

#[test]
fn test_parse_no_modifiers() {
    expect_modifiers("", vec![]);
}

#[test]
fn test_parse_modifiers_for_property() {
    expect_modifiers_decl(
        "public static",
        "property",
        vec![Modifier::Public, Modifier::Static],
    );
    expect_modifiers_decl(
        "public static ",
        "property",
        vec![Modifier::Public, Modifier::Static],
    );
}
