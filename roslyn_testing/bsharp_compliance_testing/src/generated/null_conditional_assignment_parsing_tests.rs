// Auto-generated from Roslyn: NullConditionalAssignmentParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
/// Roslyn: NullConditionalAssignmentParsingTests.Parentheses_Assignment_LHS_01 (case 1)
#[test]
fn parentheses_assignment_lhs_01() {
    let src = r#"(c?.F) = 1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (c?.F) = 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Parentheses_Assignment_LHS_01", 1, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: NullConditionalAssignmentParsingTests.Invocation_01 (case 2)
#[test]
fn invocation_01() {
    let src = r#"c?.M() = 1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.M() = 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Invocation_01", 2, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: NullConditionalAssignmentParsingTests.RefAssignment_01 (case 3)
#[test]
fn ref_assignment_01() {
    let src = r#"c?.F = ref x"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.F = ref x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "RefAssignment_01", 3, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: NullConditionalAssignmentParsingTests.RefReturningLambda_01 (case 4)
#[test]
fn ref_returning_lambda_01() {
    let src = r#"c?.F = ref int () => ref x"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.F = ref int () => ref x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "RefReturningLambda_01", 4, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: NullConditionalAssignmentParsingTests.Suppression_01 (case 5)
#[test]
fn suppression_01() {
    let src = r#"a?.b! = c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?.b! = c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("null_conditional_assignment_parsing_tests", "NullConditionalAssignmentParsingTests", "Suppression_01", 5, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

