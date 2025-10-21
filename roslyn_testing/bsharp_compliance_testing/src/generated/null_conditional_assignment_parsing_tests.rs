// Auto-generated from Roslyn: NullConditionalAssignmentParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: NullConditionalAssignmentParsingTests.Parentheses_Assignment_LHS_01 (case 1)
#[test]
fn parentheses_assignment_lhs_01() {
    let src = r#"(c?.F) = 1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (c?.F) = 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Parentheses_Assignment_LHS_01", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullConditionalAssignmentParsingTests.Invocation_01 (case 2)
#[test]
fn invocation_01() {
    let src = r#"c?.M() = 1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.M() = 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Invocation_01", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullConditionalAssignmentParsingTests.RefAssignment_01 (case 3)
#[test]
fn ref_assignment_01() {
    let src = r#"c?.F = ref x"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.F = ref x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "RefAssignment_01", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullConditionalAssignmentParsingTests.Suppression_01 (case 4)
#[test]
fn suppression_01() {
    let src = r#"a?.b! = c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?.b! = c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Suppression_01", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

