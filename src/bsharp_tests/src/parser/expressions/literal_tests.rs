// Tests for parsing literals

use parser::expressions::literal_parser::parse_literal;
use syntax::expressions::literal::Literal;

#[test]
fn test_integer_literal() {
    let input = "42";
    let result = parse_literal(input.into());
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    let (remaining_input, literal) = result.unwrap();
    assert_eq!(*remaining_input.fragment(), "");
    let expected = Literal::Integer(42);
    assert_eq!(literal, expected);
}

#[test]
fn test_integer_bases_and_underscores() {
    for (src, expected) in [
        ("1_000", 1000i64),
        ("0xFF_FF", 0xFFFFi64),
        ("0b1010_0001", 0b1010_0001i64),
    ] {
        let (rest, lit) = parse_literal(src.into()).expect("parse");
        assert!(rest.fragment().trim().is_empty());
        assert_eq!(lit, Literal::Integer(expected));
    }
}

#[test]
fn test_float_with_exponent_and_underscores() {
    for src in ["1.23e-4", "1_2.3_4e5", ".5", "0.125"] {
        let (rest, lit) = parse_literal(src.into()).expect("parse");
        assert!(rest.fragment().trim().is_empty());
        match lit {
            Literal::Float(_) => {}
            other => panic!("expected float, got {:?}", other),
        }
    }
}

#[test]
fn test_char_escapes() {
    for (src, expected) in [
        ("'\\n'", '\n'),
        ("'\\t'", '\t'),
        ("'\\x41'", 'A'),
        ("'\\u0041'", 'A'),
        ("'\\''", '\''),
    ] {
        let (rest, lit) = parse_literal(src.into()).expect("parse");
        assert!(rest.fragment().trim().is_empty());
        assert_eq!(lit, Literal::Char(expected));
    }
}

#[test]
fn test_verbatim_and_raw_strings() {
    // Verbatim with doubled quotes
    let (rest, lit) = parse_literal("@\"C:\\\\Dir\\\\\"\"Foo\"\"\"\"\"".into()).expect("parse verbatim");
    assert!(rest.fragment().trim().is_empty());
    match lit {
        Literal::VerbatimString(s) => assert!(s.contains("\"Foo\"")),
        _ => panic!("expected verbatim"),
    }

    // Raw strings with N quotes
    let (rest2, lit2) = parse_literal("\"\"\"hello\"\"\"".into()).expect("parse raw");
    assert!(rest2.fragment().trim().is_empty());
    assert_eq!(lit2, Literal::RawString("hello".to_string()));
}

#[test]
fn test_boolean_literal() {
    // Test true
    let input_true = "true".into();
    let result_true = parse_literal(input_true);
    let input_true = "true";
    let result_true = parse_literal(input_true.into());
    assert!(
        result_true.is_ok(),
        "Parsing 'true' failed: {:?}",
        result_true.err()
    );
    let (remaining_true, literal_true) = result_true.unwrap();
    assert_eq!(*remaining_true.fragment(), "");
    assert_eq!(literal_true, Literal::Boolean(true));

    // Test false with surrounding whitespace
    let input_false = "  false  ";
    let result_false = parse_literal(input_false.into());
    assert!(
        result_false.is_ok(),
        "Parsing 'false' failed: {:?}",
        result_false.err()
    );
    let (remaining_false, literal_false) = result_false.unwrap();
    assert_eq!(*remaining_false.fragment(), "");
    assert_eq!(literal_false, Literal::Boolean(false));
}

#[test]
fn test_string_literal() {
    // Simple string
    let input_simple = "\"hello world\"";
    let result_simple = parse_literal(input_simple.into());
    assert!(
        result_simple.is_ok(),
        "Parsing simple string failed: {:?}",
        result_simple.err()
    );
    let (remaining_simple, literal_simple) = result_simple.unwrap();
    assert_eq!(*remaining_simple.fragment(), "");
    assert_eq!(literal_simple, Literal::String("hello world".to_string()));

    // String with escape
    let input_escape = " \"hello \\\"world\\\"\" "; // "hello \"world\""
    let result_escape = parse_literal(input_escape.into());
    assert!(
        result_escape.is_ok(),
        "Parsing string with escape failed: {:?}",
        result_escape.err()
    );
    let (remaining_escape, literal_escape) = result_escape.unwrap();
    assert_eq!(*remaining_escape.fragment(), "");
    assert_eq!(
        literal_escape,
        Literal::String("hello \"world\"".to_string())
    );

    // Empty string
    let input_empty = "\"\"";
    let result_empty = parse_literal(input_empty.into());
    assert!(
        result_empty.is_ok(),
        "Parsing empty string failed: {:?}",
        result_empty.err()
    );
    let (remaining_empty, literal_empty) = result_empty.unwrap();
    assert_eq!(*remaining_empty.fragment(), "");
    assert_eq!(literal_empty, Literal::String("".to_string()));
}
