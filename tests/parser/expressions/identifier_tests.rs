// Tests for parsing identifiers and qualified names
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parsers::identifier_parser::{parse_identifier, parse_qualified_name};

#[test]
fn test_parse_identifier_simple() {
    let input = "foo";
    let (rest, ident) = parse_identifier(input).unwrap();
    assert_eq!(ident, Identifier { name: "foo".to_string() });
    assert_eq!(rest, "");
}

#[test]
fn test_parse_identifier_with_underscore() {
    let input = "_foo123";
    let (rest, ident) = parse_identifier(input).unwrap();
    assert_eq!(ident, Identifier { name: "_foo123".to_string() });
    assert_eq!(rest, "");
}

#[test]
fn test_parse_identifier_keyword() {
    let input = "int";
    // Should fail as 'int' is a keyword
    assert!(parse_identifier(input).is_err());
}

#[test]
fn test_parse_qualified_name() {
    let input = "System.Collections.Generic";
    let (rest, parts) = parse_qualified_name(input).unwrap();
    let expected = vec![
        Identifier { name: "System".to_string() },
        Identifier { name: "Collections".to_string() },
        Identifier { name: "Generic".to_string() },
    ];
    assert_eq!(parts, expected);
    assert_eq!(rest, "");
}
