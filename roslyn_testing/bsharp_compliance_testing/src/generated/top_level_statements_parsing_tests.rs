// Auto-generated from Roslyn: TopLevelStatementsParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: TopLevelStatementsParsingTests.InsertOpenBraceBeforeCodes (case 1)
#[test]
fn insert_open_brace_before_codes() {
    let src = r#"{
        this.I = i;
    };
}"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "InsertOpenBraceBeforeCodes", 1, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "InsertOpenBraceBeforeCodes", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "InsertOpenBraceBeforeCodes", 1, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: TopLevelStatementsParsingTests.IncompleteOperator (case 2)
#[test]
fn incomplete_operator() {
    let src = r#"C operator +(C lhs, C rhs) {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "IncompleteOperator", 2, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "IncompleteOperator", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "IncompleteOperator", 2, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: TopLevelStatementsParsingTests.NewKeyword (case 3)
#[test]
fn new_keyword() {
    let src = r#"new "#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "NewKeyword", 3, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "NewKeyword", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("top_level_statements_parsing_tests", "TopLevelStatementsParsingTests", "NewKeyword", 3, None, CaseData::File { unit: &unit, src, original: None });
    }
}

