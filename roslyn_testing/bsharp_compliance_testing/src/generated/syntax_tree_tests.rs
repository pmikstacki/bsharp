// Auto-generated from Roslyn: SyntaxTreeTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SyntaxTreeTests.Create (case 1)
#[test]
fn create() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_tree_tests", "SyntaxTreeTests", "Create", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxTreeTests.Create_WithDiagnosticOptions (case 2)
#[test]
fn create_with_diagnostic_options() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_tree_tests", "SyntaxTreeTests", "Create_WithDiagnosticOptions", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_ParsedTree (case 3)
#[test]
fn with_root_and_options_parsed_tree() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_tree_tests", "SyntaxTreeTests", "WithRootAndOptions_ParsedTree", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_ParsedTreeWithText (case 4)
#[test]
fn with_root_and_options_parsed_tree_with_text() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_tree_tests", "SyntaxTreeTests", "WithRootAndOptions_ParsedTreeWithText", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_DummyTree (case 5)
#[test]
fn with_root_and_options_dummy_tree() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_tree_tests", "SyntaxTreeTests", "WithRootAndOptions_DummyTree", 5, CaseData::File { unit: &unit, src, original: None });
}

