// Auto-generated from Roslyn: LambdaUtilitiesTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 1)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 2)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_2() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 3)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_3() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 4)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_4() {
    let src = r#"F(2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 5)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_5() {
    let src = r#"F(a => 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(a => 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 6)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_6() {
    let src = r#"F(a => 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(a => 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 7)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_7() {
    let src = r#"F(() => 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 8)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_8() {
    let src = r#"F(() => 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 9)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_9() {
    let src = r#"F(delegate { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 10)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_10() {
    let src = r#"F(delegate { return 2; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate { return 2; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 11)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_11() {
    let src = r#"F(delegate (int a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 12)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_12() {
    let src = r#"F(delegate (bool a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (bool a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 13)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_13() {
    let src = r#"F(delegate (int a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 14)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_14() {
    let src = r#"F(delegate (int a) { return 2; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 2; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 15)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_15() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 16)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_16() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 17)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_17() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_utilities_tests", "LambdaUtilitiesTests", "AreEquivalentIgnoringLambdaBodies1", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

