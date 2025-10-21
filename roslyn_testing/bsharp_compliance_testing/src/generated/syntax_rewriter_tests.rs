// Auto-generated from Roslyn: SyntaxRewriterTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SyntaxRewriterTests.TestSyntaxTreeForParsedSyntaxNode (case 1)
#[test]
fn syntax_tree_for_parsed_syntax_node() {
    let src = r#"class Class1<T> { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestSyntaxTreeForParsedSyntaxNode", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxRewriterTests.TestSyntaxTreeForParsedSyntaxNode (case 2)
#[test]
fn syntax_tree_for_parsed_syntax_node_case_2() {
    let src = r#"2 + 2"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 2 + 2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestSyntaxTreeForParsedSyntaxNode", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxRewriterTests.TestReplaceNodeInListShouldNotLoseParseOptions (case 3)
#[test]
fn replace_node_in_list_should_not_lose_parse_options() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestReplaceNodeInListShouldNotLoseParseOptions", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxRewriterTests.TestReplaceNodeInListShouldNotLoseParseOptions (case 4)
#[test]
fn replace_node_in_list_should_not_lose_parse_options_case_2() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestReplaceNodeInListShouldNotLoseParseOptions", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxRewriterTests.TestInsertNodeShouldNotLoseParseOptions (case 5)
#[test]
fn insert_node_should_not_lose_parse_options() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestInsertNodeShouldNotLoseParseOptions", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxRewriterTests.TestInsertNodeShouldNotLoseParseOptions (case 6)
#[test]
fn insert_node_should_not_lose_parse_options_case_2() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_rewriter_tests", "SyntaxRewriterTests", "TestInsertNodeShouldNotLoseParseOptions", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

