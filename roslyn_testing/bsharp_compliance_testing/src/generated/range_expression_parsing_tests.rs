// Auto-generated from Roslyn: RangeExpressionParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: RangeExpressionParsingTests.CastingRangeExpressionWithoutStartOrEnd (case 1)
#[test]
fn casting_range_expression_without_start_or_end() {
    let src = r#"(int).."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int)..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionWithoutStartOrEnd",
                    1,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionWithoutStartOrEnd",
                    1,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "CastingRangeExpressionWithoutStartOrEnd",
            1,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.CastingRangeExpressionWithoutStart (case 2)
#[test]
fn casting_range_expression_without_start() {
    let src = r#"(int)..0"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int)..0; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionWithoutStart",
                    2,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionWithoutStart",
                    2,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "CastingRangeExpressionWithoutStart",
            2,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenTrue (case 3)
#[test]
fn conditional_expression_with_empty_range_for_when_true() {
    let src = r#"a ? .. : b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? .. : b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenTrue",
                    3,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenTrue",
                    3,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyRangeForWhenTrue",
            3,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenFalse (case 4)
#[test]
fn conditional_expression_with_empty_range_for_when_false() {
    let src = r#"a ? b : .."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b : ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenFalse",
                    4,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenFalse",
                    4,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyRangeForWhenFalse",
            4,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse (case 5)
#[test]
fn conditional_expression_with_empty_range_for_when_true_and_when_false() {
    let src = r#"a ? .. : .."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? .. : ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse",
                    5,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse",
                    5,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyRangeForWhenTrueAndWhenFalse",
            5,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenTrue (case 6)
#[test]
fn conditional_expression_with_empty_start_range_for_when_true() {
    let src = r#"a ? ..b : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? ..b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenTrue",
                    6,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenTrue",
                    6,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyStartRangeForWhenTrue",
            6,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenFalse (case 7)
#[test]
fn conditional_expression_with_empty_start_range_for_when_false() {
    let src = r#"a ? b : ..c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b : ..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenFalse",
                    7,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenFalse",
                    7,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyStartRangeForWhenFalse",
            7,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse (case 8)
#[test]
fn conditional_expression_with_empty_start_range_for_when_true_and_false() {
    let src = r#"a ? ..b : ..c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? ..b : ..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse",
                    8,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse",
                    8,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "ConditionalExpressionWithEmptyStartRangeForWhenTrueAndFalse",
            8,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.CastingRangeExpressionInPattern1 (case 9)
#[test]
fn casting_range_expression_in_pattern_1() {
    let src = r#"x is (int).."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is (int)..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern1",
                    9,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern1",
                    9,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "CastingRangeExpressionInPattern1",
            9,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.CastingRangeExpressionInPattern2 (case 10)
#[test]
fn casting_range_expression_in_pattern_2() {
    let src = r#"x is (int)."#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is (int).; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern2",
                    10,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern2",
                    10,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "CastingRangeExpressionInPattern2",
            10,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: RangeExpressionParsingTests.CastingRangeExpressionInPattern3 (case 11)
#[test]
fn casting_range_expression_in_pattern_3() {
    let src = r#"x is (int).Length"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is (int).Length; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern3",
                    11,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "range_expression_parsing_tests",
                    "RangeExpressionParsingTests",
                    "CastingRangeExpressionInPattern3",
                    11,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "range_expression_parsing_tests",
            "RangeExpressionParsingTests",
            "CastingRangeExpressionInPattern3",
            11,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}
