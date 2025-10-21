// Auto-generated from Roslyn: RangeExpressionParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenTrue (case 1)
#[test]
fn conditional_expression_with_empty_range_for_when_true() {
    let src = r#"a ? .. : b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? .. : b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyRangeForWhenTrue", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenFalse (case 2)
#[test]
fn conditional_expression_with_empty_range_for_when_false() {
    let src = r#"a ? b : .."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b : ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyRangeForWhenFalse", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse (case 3)
#[test]
fn conditional_expression_with_empty_range_for_when_true_and_when_false() {
    let src = r#"a ? .. : .."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? .. : ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenTrue (case 4)
#[test]
fn conditional_expression_with_empty_start_range_for_when_true() {
    let src = r#"a ? ..b : c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? ..b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyStartRangeForWhenTrue", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenFalse (case 5)
#[test]
fn conditional_expression_with_empty_start_range_for_when_false() {
    let src = r#"a ? b : ..c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b : ..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyStartRangeForWhenFalse", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse (case 6)
#[test]
fn conditional_expression_with_empty_start_range_for_when_true_and_false() {
    let src = r#"a ? ..b : ..c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? ..b : ..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("range_expression_parsing_tests", "RangeExpressionParsingTests", "ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

