// Tests for type declaration helpers

use bsharp::parser::nodes::declarations::Modifier;
use bsharp::parsers::declarations::type_declaration_helpers::{parse_type_declaration_header, at_end_of_body};

#[test]
fn test_base_type_declaration() {
    let input = "public class MyClass<T> : IComparable<T> {";
    let (input, result) = parse_type_declaration_header(input, "class", "class").unwrap();
    
    assert_eq!(result.modifiers, vec![Modifier::Public]);
    assert_eq!(result.name.name, "MyClass");
    assert!(result.type_parameters.is_some());
    assert_eq!(result.base_types.len(), 1);
    
    // Check that we're left with the opening brace
    assert_eq!(input.trim(), "{");
}

#[test]
fn test_at_end_of_body() {
    assert!(at_end_of_body(" }"));
    assert!(at_end_of_body("\n\t}"));
    assert!(!at_end_of_body(" int x;"));
} 