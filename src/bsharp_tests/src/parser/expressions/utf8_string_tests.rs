use parser::expressions::literal_parser::parse_literal;
use syntax::expressions::literal::Literal;

#[test]
fn parses_utf8_string_suffix() {
    let (rest, lit) = parse_literal(r#""abc"u8"#.into()).expect("parse u8");
    assert!(rest.fragment().trim().is_empty());
    match lit {
        Literal::Utf8String(bytes) => assert_eq!(bytes, b"abc".to_vec()),
        other => panic!("expected Utf8String, got {:?}", other),
    }
}

#[test]
fn parses_empty_utf8_string_suffix() {
    let (rest, lit) = parse_literal(r#"""u8"#.into()).expect("parse empty u8");
    assert!(rest.fragment().trim().is_empty());
    match lit {
        Literal::Utf8String(bytes) => assert_eq!(bytes, b"".to_vec()),
        other => panic!("expected Utf8String, got {:?}", other),
    }
}
