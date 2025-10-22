// Auto-generated from Roslyn: LambdaUtilitiesTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 1)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 2)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_2() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 3)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_3() {
    let src = r#"F(1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                3,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 4)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_4() {
    let src = r#"F(2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                4,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 5)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_5() {
    let src = r#"F(a => 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(a => 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                5,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 6)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_6() {
    let src = r#"F(a => 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(a => 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                6,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 7)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_7() {
    let src = r#"F(() => 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                7,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 8)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_8() {
    let src = r#"F(() => 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                8,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 9)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_9() {
    let src = r#"F(delegate { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                9,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 10)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_10() {
    let src = r#"F(delegate { return 2; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate { return 2; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                10,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 11)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_11() {
    let src = r#"F(delegate (int a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                11,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 12)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_12() {
    let src = r#"F(delegate (bool a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (bool a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                12,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 13)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_13() {
    let src = r#"F(delegate (int a) { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                13,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 14)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_14() {
    let src = r#"F(delegate (int a) { return 2; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(delegate (int a) { return 2; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                14,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 15)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_15() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                15,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 16)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_16() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                16,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 17)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_17() {
    let src = r#"F(() => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(() => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                17,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 18)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_18() {
    let src = r#"F((a) => { return 1; })"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F((a) => { return 1; }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                18,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 19)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_19() {
    let src = r#"F(from a in new[] { 1, 2 } select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                19,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 20)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_20() {
    let src = r#"F(from a in new[] { 1, 2 } select a + 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } select a + 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                20,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 21)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_21() {
    let src = r#"F(from a in new[] { 1, 2 } where a > 0 select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } where a > 0 select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                21,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 22)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_22() {
    let src = r#"F(from a in new[] { 1, 2 } where a > 0 select a + 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } where a > 0 select a + 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                22,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 23)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_23() {
    let src = r#"F(from a in new[] { 1, 2 } orderby a select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                23,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 24)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_24() {
    let src = r#"F(from a in new[] { 1, 2 } orderby a select a + 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a select a + 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                24,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 25)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_25() {
    let src = r#"F(from a in new[] { 1, 2 } let b = 1 select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } let b = 1 select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                25,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 26)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_26() {
    let src = r#"F(from a in new[] { 1, 2 } let b = 1 select a + 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } let b = 1 select a + 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                26,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 27)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_27() {
    let src = r#"F(from a in new[] { 1, 2 } select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                27,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 28)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_28() {
    let src = r#"F(from a in new[] { 1, 2 } where b > 0 select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } where b > 0 select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                28,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 29)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_29() {
    let src = r#"F(from a in new[] { 1, 2 } from b in new[] { 3, 4 } where b > 0 select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } from b in new[] { 3, 4 } where b > 0 select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                29,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 30)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_30() {
    let src = r#"F(from a in new[] { 1, 2 } from b in new[] { 3, 4, 5 } where b > 1 select a + 1)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } from b in new[] { 3, 4, 5 } where b > 1 select a + 1); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                30,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 31)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_31() {
    let src = r#"F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                31,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 32)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_32() {
    let src = r#"F(from a in new[] { 1, 2 } join b in new[] { 3, 4, 5 } on a equals b select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } join b in new[] { 3, 4, 5 } on a equals b select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                32,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 33)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_33() {
    let src = r#"F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                33,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 34)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_34() {
    let src =
        r#"F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a + 1 equals b + 1 select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a + 1 equals b + 1 select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                34,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 35)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_35() {
    let src = r#"F(from a in new[] { 1, 2 } select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                35,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 36)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_36() {
    let src = r#"F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } join b in new[] { 3, 4 } on a equals b select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                36,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 37)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_37() {
    let src = r#"F(from a in new[] { 1, 2 } group a by a into g select g)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a by a into g select g); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                37,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 38)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_38() {
    let src = r#"F(from a in new[] { 1, 2 } group a + 1 by a into g select g)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a + 1 by a into g select g); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                38,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 39)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_39() {
    let src = r#"F(from a in new[] { 1, 2 } group a by a into g select g)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a by a into g select g); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                39,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 40)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_40() {
    let src = r#"F(from a in new[] { 1, 2 } group a by a + 1 into g select g)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a by a + 1 into g select g); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                40,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 41)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_41() {
    let src = r#"F(from a in new[] { 1, 2 } group a by a into g select g)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a by a into g select g); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                41,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 42)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_42() {
    let src = r#"F(from a in new[] { 1, 2 } group a by a into q select q)"#;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { F(from a in new[] { 1, 2 } group a by a into q select q); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                42,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 43)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_43() {
    let src = r#"F(from a in new[] { 1, 2 } orderby a, a descending, a ascending select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a, a descending, a ascending select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                43,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 44)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_44() {
    let src =
        r#"F(from a in new[] { 1, 2 } orderby a + 1, a - 1 descending, a + 1 ascending select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a + 1, a - 1 descending, a + 1 ascending select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                44,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 45)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_45() {
    let src = r#"F(from a in new[] { 1, 2 } orderby a, a descending, a ascending select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a, a descending, a ascending select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                45,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: LambdaUtilitiesTests.AreEquivalentIgnoringLambdaBodies1 (case 46)
#[test]
fn are_equivalent_ignoring_lambda_bodies_1_case_46() {
    let src = r#"F(from a in new[] { 1, 2 } orderby a, a descending, a descending select a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F(from a in new[] { 1, 2 } orderby a, a descending, a descending select a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "lambda_utilities_tests",
                "LambdaUtilitiesTests",
                "AreEquivalentIgnoringLambdaBodies1",
                46,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
