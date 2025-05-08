// Tests for parsing literals

use bsharp::parser::nodes::expressions::literal::Literal;
use bsharp::parsers::expressions::literal_parser::parse_literal;

#[test]
fn test_integer_literal() {
    let input = "42";
    let result = parse_literal(input);
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    let (remaining_input, literal) = result.unwrap();
    assert_eq!(remaining_input, "");
    let expected = Literal::Integer(42);
    assert_eq!(literal, expected);
}

#[test]
fn test_boolean_literal() {
    // Test true
    let input_true = "true";
    let result_true = parse_literal(input_true);
    assert!(result_true.is_ok(), "Parsing 'true' failed: {:?}", result_true.err());
    let (remaining_true, literal_true) = result_true.unwrap();
    assert_eq!(remaining_true, "");
    assert_eq!(literal_true, Literal::Boolean(true));

    // Test false with surrounding whitespace
    let input_false = "  false  ";
    let result_false = parse_literal(input_false);
    assert!(result_false.is_ok(), "Parsing 'false' failed: {:?}", result_false.err());
    let (remaining_false, literal_false) = result_false.unwrap();
    assert_eq!(remaining_false, "");
    assert_eq!(literal_false, Literal::Boolean(false));
}

#[test]
fn test_string_literal() {
    // Simple string
    let input_simple = "\"hello world\"";
    let result_simple = parse_literal(input_simple);
    assert!(result_simple.is_ok(), "Parsing simple string failed: {:?}", result_simple.err());
    let (remaining_simple, literal_simple) = result_simple.unwrap();
    assert_eq!(remaining_simple, "");
    assert_eq!(literal_simple, Literal::String("hello world".to_string()));

    // String with escape
    let input_escape = " \"hello \\\"world\\\"\" "; // "hello \"world\""
    let result_escape = parse_literal(input_escape);
    assert!(result_escape.is_ok(), "Parsing string with escape failed: {:?}", result_escape.err());
    let (remaining_escape, literal_escape) = result_escape.unwrap();
    assert_eq!(remaining_escape, "");
    assert_eq!(literal_escape, Literal::String("hello \"world\"".to_string()));

    // Empty string
    let input_empty = "\"\"";
    let result_empty = parse_literal(input_empty);
    assert!(result_empty.is_ok(), "Parsing empty string failed: {:?}", result_empty.err());
    let (remaining_empty, literal_empty) = result_empty.unwrap();
    assert_eq!(remaining_empty, "");
    assert_eq!(literal_empty, Literal::String("".to_string()));
}
