// Tests for parsing identifiers and qualified names
use parser::identifier_parser::{parse_identifier, parse_qualified_name};
use syntax::identifier::Identifier;

#[test]
fn test_parse_identifier_simple() {
    let input = "foo";
    let (rest, ident) = parse_identifier(input.into()).unwrap();
    assert_eq!(ident.to_string(), "foo");
    assert_eq!(*rest.fragment(), "");
}

#[test]
fn test_parse_identifier_with_underscore() {
    let input = "_foo123";
    let (rest, ident) = parse_identifier(input.into()).unwrap();
    assert_eq!(ident.to_string(), "_foo123");
    assert_eq!(*rest.fragment(), "");
}

#[test]
fn test_parse_identifier_keyword() {
    let input = "int";
    // Should fail as 'int' is a keyword
    assert!(parse_identifier(input.into()).is_err());
}

#[test]
fn test_parse_qualified_name() {
    let input = "System.Collections.Generic";
    let (rest, parts) = parse_qualified_name(input.into()).unwrap();
    let actual: Vec<String> = parts.iter().map(|p| p.to_string()).collect();
    let expected = vec![
        "System".to_string(),
        "Collections".to_string(),
        "Generic".to_string(),
    ];
    assert_eq!(actual, expected);
    assert_eq!(*rest.fragment(), "");
}
