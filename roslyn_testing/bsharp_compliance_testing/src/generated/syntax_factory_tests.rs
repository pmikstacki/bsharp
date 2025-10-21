// Auto-generated from Roslyn: SyntaxFactoryTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SyntaxFactoryTests.GetTokenDiagnosticsWithSyntaxTree_WithDiagnostics (case 1)
#[test]
fn get_token_diagnostics_with_syntax_tree_with_diagnostics() {
    let src = r#"1l"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1l; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "GetTokenDiagnosticsWithSyntaxTree_WithDiagnostics", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.GetTokenDiagnosticsWithSyntaxTree_WithoutDiagnostics (case 2)
#[test]
fn get_token_diagnostics_with_syntax_tree_without_diagnostics() {
    let src = r#"1L"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1L; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "GetTokenDiagnosticsWithSyntaxTree_WithoutDiagnostics", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 3)
#[test]
fn spacing_on_coalescing() {
    let src = r#"x is int??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestSpacingOnCoalescing", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 4)
#[test]
fn spacing_on_coalescing_case_2() {
    let src = r#"x is DateTime??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is DateTime??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestSpacingOnCoalescing", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 5)
#[test]
fn spacing_on_coalescing_case_3() {
    let src = r#"x is object??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is object??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestSpacingOnCoalescing", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestUnnecessarySemicolon (case 6)
#[test]
fn unnecessary_semicolon() {
    let src = r#"{}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestUnnecessarySemicolon", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxFactoryTests.TestCreateRecordWithMembers (case 7)
#[test]
fn create_record_with_members() {
    let src = r#"private int i;"#;
    let span = Span::new(src);
    let src2 = r#"class C { private int i; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestCreateRecordWithMembers", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestCreateRecordWithMembers (case 8)
#[test]
fn create_record_with_members_case_2() {
    let src = r#"private int i;"#;
    let span = Span::new(src);
    let src2 = r#"class C { private int i; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestCreateRecordWithMembers", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 9)
#[test]
fn parse_methods_keep_parse_options_in_the_tree() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestParseMethodsKeepParseOptionsInTheTree", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 10)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_2() {
    let src = r#""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestParseMethodsKeepParseOptionsInTheTree", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 11)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_3() {
    let src = r#"public"#;
    let span = Span::new(src);
    let src2 = r#"class C { public }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_factory_tests", "SyntaxFactoryTests", "TestParseMethodsKeepParseOptionsInTheTree", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

