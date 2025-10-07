use parser::expressions::declarations::field_declaration_parser::parse_field_declaration;
use parser::expressions::declarations::property_declaration_parser::parse_property_declaration;
use syntax::nodes::declarations::{Modifier, PropertyAccessor};
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::{PrimitiveType, Type};

#[test]
fn required_property_modifer_is_parsed() {
    let code = "public required string Name { get; init; }";
    let (rest, prop) = parse_property_declaration(code).expect("parse");
    assert!(rest.trim().is_empty());
    assert!(prop.modifiers.contains(&Modifier::Public));
    assert!(prop.modifiers.contains(&Modifier::Required));
    assert_eq!(prop.ty, Type::Primitive(PrimitiveType::String));
    assert_eq!(prop.name, Identifier::new("Name"));
    // Make sure we saw get/init
    assert!(prop.accessors.iter().any(|a| matches!(a, PropertyAccessor::Get { .. })));
    assert!(prop.accessors.iter().any(|a| matches!(a, PropertyAccessor::Init { .. })));
}

#[test]
fn required_field_modifier_is_parsed() {
    let code = "public required int Id;";
    let (rest, field) = parse_field_declaration(code).expect("parse");
    assert!(rest.trim().is_empty());
    assert!(field.modifiers.contains(&Modifier::Public));
    assert!(field.modifiers.contains(&Modifier::Required));
    assert_eq!(field.ty, Type::Primitive(PrimitiveType::Int));
    assert_eq!(field.name, Identifier::new("Id"));
}
