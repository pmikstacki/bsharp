use bsharp::parser::expressions::literal_parser::parse_literal;
use bsharp::syntax::nodes::expressions::literal::{IntegerSuffix, Literal};

fn parse_ok(code: &str) -> Literal {
    let (rest, lit) = parse_literal(code).expect("parse");
    assert!(rest.trim().is_empty(), "unparsed tail: {:?}", rest);
    lit
}

#[test]
fn raw_multiline_trimming_mixed_tabs_spaces() {
    // Closing indent is "\t  " (tab + two spaces)
    let src = "\"\"\"\n\t  A\n\t  B\n\t  \"\"\"";
    match parse_ok(src) {
        Literal::RawString(s) => {
            assert_eq!(s, "A\nB");
        }
        other => panic!("expected RawString, got {:?}", other),
    }
}

#[test]
fn raw_multiline_trimming_partial_indent_lines() {
    // Closing indent is four spaces; the second line has only two leading spaces
    // Expect: first line trims 4 spaces, second trims what matches (2 spaces)
    let src = "\"\"\"\n    A\n  B\n    \"\"\"";
    match parse_ok(src) {
        Literal::RawString(s) => {
            assert_eq!(s, "A\nB");
        }
        other => panic!("expected RawString, got {:?}", other),
    }
}

#[test]
fn raw_interpolated_multiline_trimming() {
    let src = "$\"\"\"\n    Hello {name}\n    World\n    \"\"\"";
    match parse_ok(src) {
        Literal::InterpolatedString(is) => {
            // After trimming, the text should be "Hello " then interpolation, then "\nWorld"
            assert!(is.is_verbatim); // raw treated as verbatim true in this implementation
            assert!(is.parts.len() >= 2);
            // First part starts with "Hello "
            if let bsharp::syntax::nodes::expressions::literal::InterpolatedStringPart::Text(t0) = &is.parts[0] {
                assert!(t0.starts_with("Hello "));
            } else {
                panic!("expected first part text");
            }
        }
        other => panic!("expected InterpolatedString, got {:?}", other),
    }
}

#[test]
fn integer_suffixes_decimal() {
    let cases = vec![
        ("42u", 42, IntegerSuffix::U),
        ("42U", 42, IntegerSuffix::U),
        ("42l", 42, IntegerSuffix::L),
        ("42L", 42, IntegerSuffix::L),
        ("42ul", 42, IntegerSuffix::UL),
        ("42Lu", 42, IntegerSuffix::UL),
    ];
    for (src, val, suf) in cases {
        match parse_ok(src) {
            Literal::IntegerWithSuffix(v, s) => {
                assert_eq!(v, val);
                assert_eq!(s, suf);
            }
            other => panic!("expected IntegerWithSuffix, got {:?}", other),
        }
    }
}

#[test]
fn integer_suffixes_hex_bin() {
    let cases = vec![
        ("0xFFu", 255, IntegerSuffix::U),
        ("0b1010L", 10, IntegerSuffix::L),
        ("0x10ul", 16, IntegerSuffix::UL),
    ];
    for (src, val, suf) in cases {
        match parse_ok(src) {
            Literal::IntegerWithSuffix(v, s) => {
                assert_eq!(v, val);
                assert_eq!(s, suf);
            }
            other => panic!("expected IntegerWithSuffix, got {:?}", other),
        }
    }
}

#[test]
fn float_and_decimal_suffixes() {
    match parse_ok("1.0f") { Literal::Float(v) => assert!((v - 1.0).abs() < 1e-9), other => panic!("expected Float, got {:?}", other) }
    match parse_ok("2.5D") { Literal::Float(v) => assert!((v - 2.5).abs() < 1e-9), other => panic!("expected Float, got {:?}", other) }
    match parse_ok("1m") { Literal::Decimal(s) => assert_eq!(s, "1"), other => panic!("expected Decimal, got {:?}", other) }
    match parse_ok("1_000.25M") { Literal::Decimal(s) => assert_eq!(s, "1000.25"), other => panic!("expected Decimal, got {:?}", other) }
}

#[test]
fn raw_multiline_trimming_basic() {
    let src = "\"\"\"\n    line1\n    line2\n    \"\"\"";
    match parse_ok(src) {
        Literal::RawString(s) => {
            assert_eq!(s, "line1\nline2");
        }
        other => panic!("expected RawString, got {:?}", other),
    }
}
