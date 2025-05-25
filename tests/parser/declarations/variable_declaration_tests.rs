use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parser::nodes::expressions::expression::Expression;
use bsharp::parser::nodes::types::Type;
use bsharp::parsers::declarations::variable_declaration_parser::{
    parse_variable_declarator, parse_local_variable_declaration
};
use bsharp::parser::test_helpers::parse_input_unwrap;

#[test]
fn test_parse_single_variable_declarator_no_initializer() {
    let input = "myVar";
    let (remaining_input, declarator) = parse_input_unwrap(parse_variable_declarator(input));
    assert!(remaining_input.is_empty());
    assert_eq!(declarator.name, Identifier { name: "myVar".to_string() });
    assert!(declarator.initializer.is_none());
}

#[test]
fn test_parse_single_variable_declarator_with_initializer() {
    let input = "count = 42";
    let (remaining_input, declarator) = parse_input_unwrap(parse_variable_declarator(input));
    assert!(remaining_input.is_empty());
    assert_eq!(declarator.name, Identifier { name: "count".to_string() });
    assert!(matches!(
        declarator.initializer,
        Some(Expression::Literal(Literal::Integer(42)))
    ));
}

#[test]
fn test_parse_local_variable_declaration_single_declarator() {
    let input = "int x;";
    let (remaining_input, decl) = parse_input_unwrap(parse_local_variable_declaration(input));
    assert!(remaining_input.is_empty());
    assert_eq!(decl.declaration_type, Type::Primitive(bsharp::parser::nodes::types::PrimitiveType::Int));
    assert_eq!(decl.declarators.len(), 1);
    assert_eq!(decl.declarators[0].name, Identifier { name: "x".to_string() });
    assert!(decl.declarators[0].initializer.is_none());
}

#[test]
fn test_parse_local_variable_declaration_multiple_declarators() {
    let input = "string name = \"Test\", value;";
    let (remaining_input, decl) = parse_input_unwrap(parse_local_variable_declaration(input));
    assert!(remaining_input.is_empty());
    assert_eq!(decl.declaration_type, Type::Primitive(bsharp::parser::nodes::types::PrimitiveType::String));
    assert_eq!(decl.declarators.len(), 2);
    assert_eq!(decl.declarators[0].name, Identifier { name: "name".to_string() });
    assert!(matches!(
        decl.declarators[0].initializer,
        Some(Expression::Literal(Literal::String(ref s))) if s == "Test"
    ));
    assert_eq!(decl.declarators[1].name, Identifier { name: "value".to_string() });
    assert!(decl.declarators[1].initializer.is_none());
}

#[test]
fn test_parse_local_variable_declaration_with_qualified_type() {
    let input = "System.Collections.Generic.List<string> myList = new System.Collections.Generic.List<string>();";
    let (remaining_input, decl) = parse_input_unwrap(parse_local_variable_declaration(input));
    assert!(remaining_input.is_empty());
    // Further assertions would depend on the exact structure of Type::Reference/Generic and Expression::New
    assert_eq!(decl.declarators.len(), 1);
    assert_eq!(decl.declarators[0].name, Identifier { name: "myList".to_string() });
    assert!(decl.declarators[0].initializer.is_some()); 
}
