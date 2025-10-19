// Tests for type declaration helpers

use parser::expressions::declarations::type_declaration_helpers::{
    at_end_of_body, parse_type_declaration_header_span,
};
use syntax::declarations::Modifier;

#[test]
fn test_base_type_declaration() {
    let input = "public class MyClass<T> : IComparable<T> {";
    let (input, result) = parse_type_declaration_header_span(input.into(), "class", "class").unwrap();

    assert_eq!(result.modifiers, vec![Modifier::Public]);
    assert_eq!(result.name.to_string(), "MyClass");
    assert!(result.type_parameters.is_some());
    assert_eq!(result.base_types.len(), 1);

    // Check that we're left with the opening brace
    assert_eq!(input.fragment().trim(), "{");
}

#[test]
fn test_at_end_of_body() {
    assert!(at_end_of_body(" }".into()));
    assert!(at_end_of_body("\n\t}".into()));
    assert!(!at_end_of_body(" int x;".into()));
}
