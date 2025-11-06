use parser::expressions::literal_parser::parse_literal;
use syntax::expressions::literal::Literal;

#[test]
fn test_string_e_escape() {
    let (rest, lit) = parse_literal("\"\\e\"".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    match lit {
        Literal::String(s) => {
            let bytes = s.as_bytes();
            assert_eq!(bytes, &[0x1B]);
        }
        other => panic!("expected string, got {:?}", other),
    }
}

#[test]
fn test_char_e_escape() {
    let (rest, lit) = parse_literal("'\\e'".into()).expect("parse");
    assert!(rest.fragment().trim().is_empty());
    assert_eq!(lit, Literal::Char('\x1B'));
}
