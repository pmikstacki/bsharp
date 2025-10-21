// Auto-generated from Roslyn: SyntaxListTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 1)
#[test]
fn add_insert_remove_replace() {
    let src = r#"A "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 2)
#[test]
fn add_insert_remove_replace_case_2() {
    let src = r#"B "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 3)
#[test]
fn add_insert_remove_replace_case_3() {
    let src = r#"C "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 4)
#[test]
fn add_insert_remove_replace_case_4() {
    let src = r#"A "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 5)
#[test]
fn add_insert_remove_replace_case_5() {
    let src = r#"B "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 6)
#[test]
fn add_insert_remove_replace_case_6() {
    let src = r#"C "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 7)
#[test]
fn add_insert_remove_replace_case_7() {
    let src = r#"D "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 8)
#[test]
fn add_insert_remove_replace_case_8() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "TestAddInsertRemoveReplace", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.DoTestAddInsertRemoveReplaceOnEmptyList (case 9)
#[test]
fn do_test_add_insert_remove_replace_on_empty_list() {
    let src = r#"D "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "DoTestAddInsertRemoveReplaceOnEmptyList", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.DoTestAddInsertRemoveReplaceOnEmptyList (case 10)
#[test]
fn do_test_add_insert_remove_replace_on_empty_list_case_2() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "DoTestAddInsertRemoveReplaceOnEmptyList", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.Extensions (case 11)
#[test]
fn extensions() {
    let src = r#"A+B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A+B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "Extensions", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxListTests.Extensions (case 12)
#[test]
fn extensions_case_2() {
    let src = r#"1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_list_tests", "SyntaxListTests", "Extensions", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

