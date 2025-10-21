// Auto-generated from Roslyn: PatternParsingTests_ListPatterns
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_00 (case 1)
#[test]
fn list_pattern_00() {
    let src = r#"c is [[]]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [[]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_00", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_00 (case 2)
#[test]
fn list_pattern_00_case_2() {
    let src = r#"c is [[]]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [[]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_00", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_06 (case 3)
#[test]
fn list_pattern_06() {
    let src = r#"c is [List<int>]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [List<int>]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_06", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_07 (case 4)
#[test]
fn list_pattern_07() {
    let src = r#"c is [string[]]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [string[]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_07", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_08 (case 5)
#[test]
fn list_pattern_08() {
    let src = r#"c is [var(x,y)]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [var(x,y)]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_08", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.ListPattern_09 (case 6)
#[test]
fn list_pattern_09() {
    let src = r#"c is [>0]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [>0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "ListPattern_09", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.NoRegressionOnArrayTypePattern_01 (case 7)
#[test]
fn no_regression_on_array_type_pattern_01() {
    let src = r#"c is string[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is string[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "NoRegressionOnArrayTypePattern_01", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_01 (case 8)
#[test]
fn slice_pattern_01() {
    let src = r#"c is [..]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_01", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_01 (case 9)
#[test]
fn slice_pattern_01_case_2() {
    let src = r#"c is [..]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_01", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_02 (case 10)
#[test]
fn slice_pattern_02() {
    let src = r#"c is [.. var x]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [.. var x]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_02", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_05 (case 11)
#[test]
fn slice_pattern_05() {
    let src = r#"c is [..[]]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [..[]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_05", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_06 (case 12)
#[test]
fn slice_pattern_06() {
    let src = r#"c is [.. not p]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [.. not p]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_06", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests_ListPatterns.SlicePattern_19 (case 13)
#[test]
fn slice_pattern_19() {
    let src = r#"c is [..>5]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c is [..>5]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_list_patterns", "PatternParsingTests_ListPatterns", "SlicePattern_19", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

