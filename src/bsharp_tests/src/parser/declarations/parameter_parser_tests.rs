// Tests for parsing parameters: attributes and default values

use parser::expressions::declarations::parameter_parser::{
    parse_parameter, parse_parameter_list,
};
use syntax::nodes::expressions::expression::Expression;
use syntax::nodes::expressions::literal::Literal;
use syntax::nodes::identifier::Identifier;
use syntax::nodes::types::{Parameter, ParameterModifier, PrimitiveType, Type};

fn parse_param_ok(code: &str) -> Parameter {
    match parse_parameter(code) {
        Ok((rest, p)) if rest.trim().is_empty() => p,
        Ok((rest, _)) => panic!("Unparsed: '{}'", rest),
        Err(e) => panic!("Parse error: {:?}", e),
    }
}

#[test]
fn parameter_with_single_attribute() {
    let p = parse_param_ok("[A] int x");
    assert_eq!(p.attributes.len(), 1);
    assert_eq!(p.attributes[0].name.name, "A");
    assert_eq!(p.parameter_type, Type::Primitive(PrimitiveType::Int));
    assert_eq!(p.name, Identifier::new("x"));
    assert!(p.default_value.is_none());
}

#[test]
fn parameter_with_multiple_attribute_lists_and_default() {
    let p = parse_param_ok("[A][B] string name = \"John\"");
    assert_eq!(p.attributes.len(), 2);
    assert_eq!(p.attributes[0].name.name, "A");
    assert_eq!(p.attributes[1].name.name, "B");
    assert_eq!(p.parameter_type, Type::Primitive(PrimitiveType::String));
    assert_eq!(p.name, Identifier::new("name"));
    assert!(
        matches!(p.default_value, Some(Expression::Literal(Literal::String(s))) if s == "John")
    );
}

#[test]
fn parameter_with_modifier_and_attribute() {
    let p = parse_param_ok("[A] ref int value");
    assert_eq!(p.attributes.len(), 1);
    assert_eq!(p.modifier, Some(ParameterModifier::Ref));
    assert_eq!(p.parameter_type, Type::Primitive(PrimitiveType::Int));
    assert_eq!(p.name, Identifier::new("value"));
}

#[test]
fn parameter_list_with_attributes_and_defaults() {
    let code = "([A] int x = 1, out string name)";
    let (rest, list) = parse_parameter_list(code).expect("parse list");
    assert!(rest.trim().is_empty());
    assert_eq!(list.len(), 2);
    // First param
    assert_eq!(list[0].attributes.len(), 1);
    assert_eq!(list[0].attributes[0].name.name, "A");
    assert_eq!(list[0].parameter_type, Type::Primitive(PrimitiveType::Int));
    assert!(matches!(
        list[0].default_value,
        Some(Expression::Literal(Literal::Integer(1)))
    ));
    // Second param
    assert_eq!(list[1].modifier, Some(ParameterModifier::Out));
    assert_eq!(
        list[1].parameter_type,
        Type::Primitive(PrimitiveType::String)
    );
    assert_eq!(list[1].name, Identifier::new("name"));
}
