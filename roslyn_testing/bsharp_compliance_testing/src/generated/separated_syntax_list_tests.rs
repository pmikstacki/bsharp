// Auto-generated from Roslyn: SeparatedSyntaxListTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 1)
#[test]
fn separated_list_insert() {
    let src = r#"x"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 2)
#[test]
fn separated_list_insert_case_2() {
    let src = r#"y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 3)
#[test]
fn separated_list_insert_case_3() {
    let src = r#"y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 4)
#[test]
fn separated_list_insert_case_4() {
    let src = r#"a"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 5)
#[test]
fn separated_list_insert_case_5() {
    let src = r#"b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 6)
#[test]
fn separated_list_insert_case_6() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 7)
#[test]
fn separated_list_insert_case_7() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 8)
#[test]
fn separated_list_insert_case_8() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestSeparatedListInsert (case 9)
#[test]
fn separated_list_insert_case_9() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestSeparatedListInsert", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 10)
#[test]
fn add_insert_remove() {
    let src = r#"A"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 11)
#[test]
fn add_insert_remove_case_2() {
    let src = r#"B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 12)
#[test]
fn add_insert_remove_case_3() {
    let src = r#"C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 13)
#[test]
fn add_insert_remove_case_4() {
    let src = r#"A"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 14)
#[test]
fn add_insert_remove_case_5() {
    let src = r#"B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 15)
#[test]
fn add_insert_remove_case_6() {
    let src = r#"C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 16)
#[test]
fn add_insert_remove_case_7() {
    let src = r#"D"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.TestAddInsertRemove (case 17)
#[test]
fn add_insert_remove_case_8() {
    let src = r#"E"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "TestAddInsertRemove", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.DoTestAddInsertRemoveOnEmptyList (case 18)
#[test]
fn do_test_add_insert_remove_on_empty_list() {
    let src = r#"D"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "DoTestAddInsertRemoveOnEmptyList", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.DoTestAddInsertRemoveOnEmptyList (case 19)
#[test]
fn do_test_add_insert_remove_on_empty_list_case_2() {
    let src = r#"E"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "DoTestAddInsertRemoveOnEmptyList", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.Extensions (case 20)
#[test]
fn extensions() {
    let src = r#"A+B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A+B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "Extensions", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SeparatedSyntaxListTests.Extensions (case 21)
#[test]
fn extensions_case_2() {
    let src = r#"1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_tests", "SeparatedSyntaxListTests", "Extensions", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

