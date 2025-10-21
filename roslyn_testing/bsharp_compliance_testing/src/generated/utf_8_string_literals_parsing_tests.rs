// Auto-generated from Roslyn: Utf8StringLiteralsParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_01 (case 1)
#[test]
fn raw_string_literal_01() {
    let src = r#"""""""hello"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "RawStringLiteral_01", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_02 (case 2)
#[test]
fn raw_string_literal_02() {
    let src = r#"""""""
hello
"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "RawStringLiteral_02", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_13 (case 3)
#[test]
fn utf_8_string_literal_13() {
    let src = r#"""""""hello"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Utf8StringLiteral_13", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_17 (case 4)
#[test]
fn errors_17() {
    let src = r#"""""""hello"""""" "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello"""""" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_17", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_18 (case 5)
#[test]
fn errors_18() {
    let src = r#"""""""hello"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_18", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_20 (case 6)
#[test]
fn errors_20() {
    let src = r#"""""""hello"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_20", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_05 (case 7)
#[test]
fn interpolation_05() {
    let src = r#"$""""""hello"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Interpolation_05", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_15 (case 8)
#[test]
fn utf_8_string_literal_15() {
    let src = r#"""""""
hello
"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Utf8StringLiteral_15", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_21 (case 9)
#[test]
fn errors_21() {
    let src = r#"""""""
hello
"""""" "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
"""""" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_21", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_22 (case 10)
#[test]
fn errors_22() {
    let src = r#"""""""
hello
"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_22", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_24 (case 11)
#[test]
fn errors_24() {
    let src = r#"""""""
hello
"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Errors_24", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_07 (case 12)
#[test]
fn interpolation_07() {
    let src = r#"$""""""
hello
"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("utf_8_string_literals_parsing_tests", "Utf8StringLiteralsParsingTests", "Interpolation_07", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

