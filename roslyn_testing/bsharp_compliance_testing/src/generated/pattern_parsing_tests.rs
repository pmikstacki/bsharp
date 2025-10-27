// Auto-generated from Roslyn: PatternParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: PatternParsingTests.ThrowExpression (case 1)
#[test]
fn throw_expression() {
    let src = r#"
class C
{
    int x = y ?? throw null;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ThrowExpression",
                    1,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ThrowExpression",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ThrowExpression",
            1,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_1 (case 2)
#[test]
fn is_pattern_precedence_1() {
    let src = r#"A is B < C, D > [ ]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_1",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_1",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_1",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_2 (case 3)
#[test]
fn is_pattern_precedence_2() {
    let src = r#"A < B > C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B > C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_2",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_2",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_2",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 4)
#[test]
fn is_pattern_precedence_3() {
    let src = r#"e is A<B> && e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> && e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 5)
#[test]
fn is_pattern_precedence_3_case_2() {
    let src = r#"e is A<B> || e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> || e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 6)
#[test]
fn is_pattern_precedence_3_case_3() {
    let src = r#"e is A<B> ^ e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> ^ e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 7)
#[test]
fn is_pattern_precedence_3_case_4() {
    let src = r#"e is A<B> | e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> | e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 8)
#[test]
fn is_pattern_precedence_3_case_5() {
    let src = r#"e is A<B> & e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> & e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 9)
#[test]
fn is_pattern_precedence_3_case_6() {
    let src = r#"e is A<B>[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B>[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 10)
#[test]
fn is_pattern_precedence_3_case_7() {
    let src = r#"new { X = e is A<B> }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new { X = e is A<B> }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 11)
#[test]
fn is_pattern_precedence_3_case_8() {
    let src = r#"e is A<B>"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
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
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
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

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 12)
#[test]
fn is_pattern_precedence_3_case_9() {
    let src = r#"(item is Dictionary<string, object>[])"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (item is Dictionary<string, object>[]); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    12,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    12,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
            12,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 13)
#[test]
fn is_pattern_precedence_3_case_10() {
    let src = r#"A is B < C, D > [ ]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    13,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    13,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
            13,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 14)
#[test]
fn is_pattern_precedence_3_case_11() {
    let src = r#"A is B < C, D > [ ] E"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ] E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    14,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    14,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
            14,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 15)
#[test]
fn is_pattern_precedence_3_case_12() {
    let src = r#"A < B > C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B > C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    15,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsPatternPrecedence_3",
                    15,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsPatternPrecedence_3",
            15,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.QueryContextualPatternVariable_01 (case 16)
#[test]
fn query_contextual_pattern_variable_01() {
    let src = r#"from s in a where s is string where s.Length > 1 select s"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { from s in a where s is string where s.Length > 1 select s; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "QueryContextualPatternVariable_01",
                    16,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "QueryContextualPatternVariable_01",
                    16,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "QueryContextualPatternVariable_01",
            16,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.QueryContextualPatternVariable_01 (case 17)
#[test]
fn query_contextual_pattern_variable_01_case_2() {
    let src = r#"M(out int? x)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(out int? x); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "QueryContextualPatternVariable_01",
                    17,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "QueryContextualPatternVariable_01",
                    17,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "QueryContextualPatternVariable_01",
            17,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TypeDisambiguation_01 (case 18)
#[test]
fn type_disambiguation_01() {
    let src = r#"
                var r = from s in a
                        where s is X<T> // should disambiguate as a type here
                        where M(s)
                        select s as X<T>;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_01",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_01",
                    18,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypeDisambiguation_01",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypeDisambiguation_02 (case 19)
#[test]
fn type_disambiguation_02() {
    let src = r#"
                var r = a is X<T> // should disambiguate as a type here
                        is bool;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_02",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_02",
                    19,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypeDisambiguation_02",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypeDisambiguation_03 (case 20)
#[test]
fn type_disambiguation_03() {
    let src = r#"
                var r = a is X<T> // should disambiguate as a type here
                        > Z;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_03",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypeDisambiguation_03",
                    20,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypeDisambiguation_03",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence00 (case 21)
#[test]
fn pattern_expression_precedence_00() {
    let src = r#"A is B << C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B << C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence00",
                    21,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence00",
                    21,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence00",
            21,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence01 (case 22)
#[test]
fn pattern_expression_precedence_01() {
    let src = r#"A is 1 << 2"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is 1 << 2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence01",
                    22,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence01",
                    22,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence01",
            22,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence02 (case 23)
#[test]
fn pattern_expression_precedence_02() {
    let src = r#"A is null < B"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null < B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence02",
                    23,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence02",
                    23,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence02",
            23,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence02b (case 24)
#[test]
fn pattern_expression_precedence_02_b() {
    let src = r#"A is B < C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence02b",
                    24,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence02b",
                    24,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence02b",
            24,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence03 (case 25)
#[test]
fn pattern_expression_precedence_03() {
    let src = r#"A is null == B"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null == B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence03",
                    25,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence03",
                    25,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence03",
            25,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence04 (case 26)
#[test]
fn pattern_expression_precedence_04() {
    let src = r#"A is null & B"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null & B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence04",
                    26,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence04",
                    26,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence04",
            26,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence05 (case 27)
#[test]
fn pattern_expression_precedence_05() {
    let src = r#"A is null && B"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null && B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence05",
                    27,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence05",
                    27,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence05",
            27,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence05b (case 28)
#[test]
fn pattern_expression_precedence_05_b() {
    let src = r#"A is null || B"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null || B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence05b",
                    28,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence05b",
                    28,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence05b",
            28,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence06 (case 29)
#[test]
fn pattern_expression_precedence_06() {
    let src = r#"switch (e) {
case 1 << 2:
case B << C:
case null < B:
case null == B:
case null & B:
case null && B:
    break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence06",
                    29,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence06",
                    29,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence06",
            29,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence07 (case 30)
#[test]
fn pattern_expression_precedence_07() {
    let src = r#"switch (array) {
case KeyValuePair<string, DateTime>[] pairs1:
case KeyValuePair<String, DateTime>[] pairs2:
    break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence07",
                    30,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PatternExpressionPrecedence07",
                    30,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PatternExpressionPrecedence07",
            30,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_01 (case 31)
#[test]
fn array_of_pointer_01() {
    let src = r#"A is B***"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_01",
                    31,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_01",
                    31,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_01",
            31,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_01b (case 32)
#[test]
fn array_of_pointer_01_b() {
    let src = r#"A is B*** C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B*** C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_01b",
                    32,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_01b",
                    32,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_01b",
            32,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_02 (case 33)
#[test]
fn array_of_pointer_02() {
    let src = r#"A is B***[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_02",
                    33,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_02",
                    33,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_02",
            33,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_03 (case 34)
#[test]
fn array_of_pointer_03() {
    let src = r#"A is B***[] C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***[] C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_03",
                    34,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_03",
                    34,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_03",
            34,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_04 (case 35)
#[test]
fn array_of_pointer_04() {
    let src = r#"(B*** C, D)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (B*** C, D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_04",
                    35,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_04",
                    35,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_04",
            35,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_04b (case 36)
#[test]
fn array_of_pointer_04_b() {
    let src = r#"(B*** C)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (B*** C); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_04b",
                    36,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_04b",
                    36,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_04b",
            36,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_05 (case 37)
#[test]
fn array_of_pointer_05() {
    let src = r#"(B***[] C, D)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (B***[] C, D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_05",
                    37,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_05",
                    37,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_05",
            37,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_06 (case 38)
#[test]
fn array_of_pointer_06() {
    let src = r#"(D, B*** C)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (D, B*** C); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_06",
                    38,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_06",
                    38,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_06",
            38,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_07 (case 39)
#[test]
fn array_of_pointer_07() {
    let src = r#"(D, B***[] C)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (D, B***[] C); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_07",
                    39,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_07",
                    39,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_07",
            39,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_08 (case 40)
#[test]
fn array_of_pointer_08() {
    let src = r#"switch (e) { case B*** C: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_08",
                    40,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_08",
                    40,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_08",
            40,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_09 (case 41)
#[test]
fn array_of_pointer_09() {
    let src = r#"switch (e) { case B***[] C: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_09",
                    41,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfPointer_09",
                    41,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfPointer_09",
            41,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NameofInPattern_01 (case 42)
#[test]
fn nameof_in_pattern_01() {
    let src = r#"switch (e) { case nameof n: ; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_01",
                    42,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_01",
                    42,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NameofInPattern_01",
            42,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NameofInPattern_02 (case 43)
#[test]
fn nameof_in_pattern_02() {
    let src = r#"switch (e) { case nameof(n): ; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_02",
                    43,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_02",
                    43,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NameofInPattern_02",
            43,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NameofInPattern_03 (case 44)
#[test]
fn nameof_in_pattern_03() {
    let src = r#"switch (e) { case nameof(n) when true: ; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_03",
                    44,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NameofInPattern_03",
                    44,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NameofInPattern_03",
            44,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpression_01 (case 45)
#[test]
fn parenthesized_expression_01() {
    let src = r#"switch (e) { case (((3))): ; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpression_01",
                    45,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpression_01",
                    45,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpression_01",
            45,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpression_02 (case 46)
#[test]
fn parenthesized_expression_02() {
    let src = r#"switch (e) { case (((3))) when true: ; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpression_02",
                    46,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpression_02",
                    46,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpression_02",
            46,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.BrokenPattern_08 (case 47)
#[test]
fn broken_pattern_08() {
    let src = r#"switch (e) { case"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenPattern_08",
                    47,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenPattern_08",
                    47,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "BrokenPattern_08",
            47,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpression03 (case 48)
#[test]
fn switch_expression_03() {
    let src = r#"1 switch { (a, b, c) => d }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { (a, b, c) => d }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpression03",
                    48,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpression03",
                    48,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpression03",
            48,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.VarIsContextualKeywordForPatterns01 (case 49)
#[test]
fn var_is_contextual_keyword_for_patterns_01() {
    let src = r#"switch (e) { case var: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "VarIsContextualKeywordForPatterns01",
                    49,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "VarIsContextualKeywordForPatterns01",
                    49,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "VarIsContextualKeywordForPatterns01",
            49,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.VarIsContextualKeywordForPatterns02 (case 50)
#[test]
fn var_is_contextual_keyword_for_patterns_02() {
    let src = r#"if (e is var) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "VarIsContextualKeywordForPatterns02",
                    50,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "VarIsContextualKeywordForPatterns02",
                    50,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "VarIsContextualKeywordForPatterns02",
            50,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.WhenAsPatternVariable01 (case 51)
#[test]
fn when_as_pattern_variable_01() {
    let src = r#"switch (e) { case var when: break; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenAsPatternVariable01",
                    51,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenAsPatternVariable01",
                    51,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "WhenAsPatternVariable01",
            51,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.WhenAsPatternVariable02 (case 52)
#[test]
fn when_as_pattern_variable_02() {
    let src = r#"switch (e) { case K when: break; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenAsPatternVariable02",
                    52,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenAsPatternVariable02",
                    52,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "WhenAsPatternVariable02",
            52,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType01 (case 53)
#[test]
fn array_of_tuple_type_01() {
    let src = r#"if (o is (int, int)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType01",
                    53,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType01",
                    53,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType01",
            53,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType02 (case 54)
#[test]
fn array_of_tuple_type_02() {
    let src = r#"if (o is (int a, int b)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType02",
                    54,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType02",
                    54,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType02",
            54,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType03 (case 55)
#[test]
fn array_of_tuple_type_03() {
    let src = r#"if (o is (int, int)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType03",
                    55,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType03",
                    55,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType03",
            55,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType04 (case 56)
#[test]
fn array_of_tuple_type_04() {
    let src = r#"if (o is (int a, int b)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType04",
                    56,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType04",
                    56,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType04",
            56,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType05 (case 57)
#[test]
fn array_of_tuple_type_05() {
    let src = r#"if (o is (Int, Int)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType05",
                    57,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType05",
                    57,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType05",
            57,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType06 (case 58)
#[test]
fn array_of_tuple_type_06() {
    let src = r#"if (o is (Int a, Int b)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType06",
                    58,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType06",
                    58,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType06",
            58,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType07 (case 59)
#[test]
fn array_of_tuple_type_07() {
    let src = r#"if (o is (Int, Int)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType07",
                    59,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType07",
                    59,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType07",
            59,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType08 (case 60)
#[test]
fn array_of_tuple_type_08() {
    let src = r#"if (o is (Int a, Int b)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType08",
                    60,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType08",
                    60,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType08",
            60,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType09 (case 61)
#[test]
fn array_of_tuple_type_09() {
    let src = r#"if (o is (S.Int, S.Int)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType09",
                    61,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType09",
                    61,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType09",
            61,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType10 (case 62)
#[test]
fn array_of_tuple_type_10() {
    let src = r#"if (o is (S.Int a, S.Int b)[]) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType10",
                    62,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType10",
                    62,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType10",
            62,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType11 (case 63)
#[test]
fn array_of_tuple_type_11() {
    let src = r#"if (o is (S.Int, S.Int)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType11",
                    63,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType11",
                    63,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType11",
            63,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType12 (case 64)
#[test]
fn array_of_tuple_type_12() {
    let src = r#"if (o is (S.Int a, S.Int b)[] q) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType12",
                    64,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType12",
                    64,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType12",
            64,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType13 (case 65)
#[test]
fn array_of_tuple_type_13() {
    let src = r#"switch (o) { case (int, int)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType13",
                    65,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType13",
                    65,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType13",
            65,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType14 (case 66)
#[test]
fn array_of_tuple_type_14() {
    let src = r#"switch (o) { case (int a, int b)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType14",
                    66,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType14",
                    66,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType14",
            66,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType15 (case 67)
#[test]
fn array_of_tuple_type_15() {
    let src = r#"switch (o) { case (Int, Int)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType15",
                    67,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType15",
                    67,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType15",
            67,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType16 (case 68)
#[test]
fn array_of_tuple_type_16() {
    let src = r#"switch (o) { case (Int a, Int b)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType16",
                    68,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType16",
                    68,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType16",
            68,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType17 (case 69)
#[test]
fn array_of_tuple_type_17() {
    let src = r#"switch (o) { case (S.Int, S.Int)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType17",
                    69,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType17",
                    69,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType17",
            69,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ArrayOfTupleType18 (case 70)
#[test]
fn array_of_tuple_type_18() {
    let src = r#"switch (o) { case (S.Int a, S.Int b)[] q: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType18",
                    70,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ArrayOfTupleType18",
                    70,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ArrayOfTupleType18",
            70,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_00 (case 71)
#[test]
fn recursive_pattern_00() {
    let src = r#"var x = o is Type (Param: 3, Param2: 4) { Prop : 3 } x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_00",
                    71,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_00",
                    71,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_00",
            71,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_02 (case 72)
#[test]
fn recursive_pattern_02() {
    let src = r#"var x = o is (Param: 3, Param2: 4) { Prop : 3 } x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_02",
                    72,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_02",
                    72,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_02",
            72,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_03 (case 73)
#[test]
fn recursive_pattern_03() {
    let src = r#"var x = o is Type { Prop : 3 } x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_03",
                    73,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_03",
                    73,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_03",
            73,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_04 (case 74)
#[test]
fn recursive_pattern_04() {
    let src = r#"var x = o is { Prop : 3 } x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_04",
                    74,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_04",
                    74,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_04",
            74,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_05 (case 75)
#[test]
fn recursive_pattern_05() {
    let src = r#"var x = o is Type (Param: 3, Param2: 4) x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_05",
                    75,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_05",
                    75,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_05",
            75,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_06 (case 76)
#[test]
fn recursive_pattern_06() {
    let src = r#"var x = o is (Param: 3, Param2: 4) x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_06",
                    76,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_06",
                    76,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_06",
            76,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_07 (case 77)
#[test]
fn recursive_pattern_07() {
    let src = r#"var x = o is Type x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_07",
                    77,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_07",
                    77,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_07",
            77,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_08 (case 78)
#[test]
fn recursive_pattern_08() {
    let src = r#"var x = o is Type (Param: 3, Param2: 4) { Prop : 3 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_08",
                    78,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_08",
                    78,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_08",
            78,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_09 (case 79)
#[test]
fn recursive_pattern_09() {
    let src = r#"var x = o is (Param: 3, Param2: 4) { Prop : 3 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_09",
                    79,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_09",
                    79,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_09",
            79,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_10 (case 80)
#[test]
fn recursive_pattern_10() {
    let src = r#"var x = o is Type { Prop : 3 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_10",
                    80,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_10",
                    80,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_10",
            80,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_11 (case 81)
#[test]
fn recursive_pattern_11() {
    let src = r#"var x = o is { Prop : 3 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_11",
                    81,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_11",
                    81,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_11",
            81,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_12 (case 82)
#[test]
fn recursive_pattern_12() {
    let src = r#"var x = o is Type (Param: 3, Param2: 4);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_12",
                    82,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_12",
                    82,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_12",
            82,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RecursivePattern_13 (case 83)
#[test]
fn recursive_pattern_13() {
    let src = r#"var x = o is (Param: 3, Param2: 4);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_13",
                    83,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RecursivePattern_13",
                    83,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RecursivePattern_13",
            83,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionOfSwitchExpression (case 84)
#[test]
fn parenthesized_expression_of_switch_expression() {
    let src = r#"Console.Write((t) switch {var x => x});"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionOfSwitchExpression",
                    84,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionOfSwitchExpression",
                    84,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionOfSwitchExpression",
            84,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInSwitchExpression (case 85)
#[test]
fn discard_in_switch_expression() {
    let src = r#"e switch { _ => 1 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { _ => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchExpression",
                    85,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchExpression",
                    85,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInSwitchExpression",
            85,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_01a (case 86)
#[test]
fn discard_in_switch_statement_01_a() {
    let src = r#"switch(e) { case _: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_01a",
                    86,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_01a",
                    86,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInSwitchStatement_01a",
            86,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_01b (case 87)
#[test]
fn discard_in_switch_statement_01_b() {
    let src = r#"switch(e) { case _: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_01b",
                    87,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_01b",
                    87,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInSwitchStatement_01b",
            87,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_02 (case 88)
#[test]
fn discard_in_switch_statement_02() {
    let src = r#"switch(e) { case _ when true: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_02",
                    88,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInSwitchStatement_02",
                    88,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInSwitchStatement_02",
            88,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInRecursivePattern_01 (case 89)
#[test]
fn discard_in_recursive_pattern_01() {
    let src = r#"e is (_, _)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is (_, _); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInRecursivePattern_01",
                    89,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInRecursivePattern_01",
                    89,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInRecursivePattern_01",
            89,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.DiscardInRecursivePattern_02 (case 90)
#[test]
fn discard_in_recursive_pattern_02() {
    let src = r#"e is { P: _ }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { P: _ }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInRecursivePattern_02",
                    90,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "DiscardInRecursivePattern_02",
                    90,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "DiscardInRecursivePattern_02",
            90,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.NotDiscardInIsTypeExpression (case 91)
#[test]
fn not_discard_in_is_type_expression() {
    let src = r#"e is _"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is _; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NotDiscardInIsTypeExpression",
                    91,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NotDiscardInIsTypeExpression",
                    91,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NotDiscardInIsTypeExpression",
            91,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ShortTuplePatterns (case 92)
#[test]
fn short_tuple_patterns() {
    let src = r#"e switch {
    var () => 1,
    () => 2,
    var (x) => 3,
    (1) _ => 4,
    (1) x => 5,
    (1) {} => 6,
    (Item1: 1) => 7,
    C(1) => 8
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch {
    var () => 1,
    () => 2,
    var (x) => 3,
    (1) _ => 4,
    (1) x => 5,
    (1) {} => 6,
    (Item1: 1) => 7,
    C(1) => 8
}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ShortTuplePatterns",
                    92,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ShortTuplePatterns",
                    92,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ShortTuplePatterns",
            92,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.NestedShortTuplePatterns (case 93)
#[test]
fn nested_short_tuple_patterns() {
    let src = r#"e switch {
    {X: var ()} => 1,
    {X: ()} => 2,
    {X: var (x)} => 3,
    {X: (1) _} => 4,
    {X: (1) x} => 5,
    {X: (1) {}} => 6,
    {X: (Item1: 1)} => 7,
    {X: C(1)} => 8
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch {
    {X: var ()} => 1,
    {X: ()} => 2,
    {X: var (x)} => 3,
    {X: (1) _} => 4,
    {X: (1) x} => 5,
    {X: (1) {}} => 6,
    {X: (Item1: 1)} => 7,
    {X: C(1)} => 8
}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NestedShortTuplePatterns",
                    93,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NestedShortTuplePatterns",
                    93,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NestedShortTuplePatterns",
            93,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsNullableArray01 (case 94)
#[test]
fn is_nullable_array_01() {
    let src = r#"o is A[] ? b : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A[] ? b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray01",
                    94,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray01",
                    94,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsNullableArray01",
            94,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsNullableArray02 (case 95)
#[test]
fn is_nullable_array_02() {
    let src = r#"o is A[] ? b && c"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A[] ? b && c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray02",
                    95,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray02",
                    95,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsNullableArray02",
            95,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsNullableArray03 (case 96)
#[test]
fn is_nullable_array_03() {
    let src = r#"o is A[][] ? b : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A[][] ? b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray03",
                    96,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableArray03",
                    96,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsNullableArray03",
            96,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsNullableType01 (case 97)
#[test]
fn is_nullable_type_01() {
    let src = r#"o is A ? b : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A ? b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableType01",
                    97,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableType01",
                    97,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsNullableType01",
            97,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.IsNullableType02 (case 98)
#[test]
fn is_nullable_type_02() {
    let src = r#"o is A? ? b : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A? ? b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableType02",
                    98,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "IsNullableType02",
                    98,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "IsNullableType02",
            98,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInSwitchExpression_01 (case 99)
#[test]
fn trailing_comma_in_switch_expression_01() {
    let src = r#"1 switch { 1 => 2, }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { 1 => 2, }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInSwitchExpression_01",
                    99,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInSwitchExpression_01",
                    99,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInSwitchExpression_01",
            99,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInSwitchExpression_02 (case 100)
#[test]
fn trailing_comma_in_switch_expression_02() {
    let src = r#"1 switch { , }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { , }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInSwitchExpression_02",
                    100,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInSwitchExpression_02",
                    100,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInSwitchExpression_02",
            100,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInPropertyPattern_01 (case 101)
#[test]
fn trailing_comma_in_property_pattern_01() {
    let src = r#"e is { X: 3, }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { X: 3, }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPropertyPattern_01",
                    101,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPropertyPattern_01",
                    101,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInPropertyPattern_01",
            101,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInPropertyPattern_02 (case 102)
#[test]
fn trailing_comma_in_property_pattern_02() {
    let src = r#"e is { , }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { , }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPropertyPattern_02",
                    102,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPropertyPattern_02",
                    102,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInPropertyPattern_02",
            102,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInPositionalPattern_01 (case 103)
#[test]
fn trailing_comma_in_positional_pattern_01() {
    let src = r#"e is ( X: 3, )"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is ( X: 3, ); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPositionalPattern_01",
                    103,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPositionalPattern_01",
                    103,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInPositionalPattern_01",
            103,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.TrailingCommaInPositionalPattern_02 (case 104)
#[test]
fn trailing_comma_in_positional_pattern_02() {
    let src = r#"e is ( , )"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is ( , ); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPositionalPattern_02",
                    104,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TrailingCommaInPositionalPattern_02",
                    104,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TrailingCommaInPositionalPattern_02",
            104,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ExtraCommaInSwitchExpression (case 105)
#[test]
fn extra_comma_in_switch_expression() {
    let src = r#"e switch { 1 => 2,, }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { 1 => 2,, }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ExtraCommaInSwitchExpression",
                    105,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ExtraCommaInSwitchExpression",
                    105,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ExtraCommaInSwitchExpression",
            105,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ExtraCommaInPropertyPattern (case 106)
#[test]
fn extra_comma_in_property_pattern() {
    let src = r#"e is { A: 1,, }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { A: 1,, }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ExtraCommaInPropertyPattern",
                    106,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ExtraCommaInPropertyPattern",
                    106,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ExtraCommaInPropertyPattern",
            106,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_01 (case 107)
#[test]
fn parenthesized_expression_in_pattern_01() {
    let src = r#"switch (e) {
    case (('C') << 24) + (('g') << 16) + (('B') << 8) + 'I': break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_01",
                    107,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_01",
                    107,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionInPattern_01",
            107,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_02 (case 108)
#[test]
fn parenthesized_expression_in_pattern_02() {
    let src = r#"switch (e) {
    case ((2) + (2)): break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_02",
                    108,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_02",
                    108,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionInPattern_02",
            108,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_03 (case 109)
#[test]
fn parenthesized_expression_in_pattern_03() {
    let src = r#"switch (e) {
    case ((2 + 2) - 2): break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_03",
                    109,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_03",
                    109,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionInPattern_03",
            109,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_04 (case 110)
#[test]
fn parenthesized_expression_in_pattern_04() {
    let src = r#"switch (e) {
    case (2) | (2): break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_04",
                    110,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_04",
                    110,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionInPattern_04",
            110,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_05 (case 111)
#[test]
fn parenthesized_expression_in_pattern_05() {
    let src = r#"switch (e) {
    case ((2 << 2) | 2): break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_05",
                    111,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedExpressionInPattern_05",
                    111,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedExpressionInPattern_05",
            111,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ChainedSwitchExpression_01 (case 112)
#[test]
fn chained_switch_expression_01() {
    let src = r#"1 switch { 1 => 2 } switch { 2 => 3 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { 1 => 2 } switch { 2 => 3 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ChainedSwitchExpression_01",
                    112,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ChainedSwitchExpression_01",
                    112,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ChainedSwitchExpression_01",
            112,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ChainedSwitchExpression_02 (case 113)
#[test]
fn chained_switch_expression_02() {
    let src = r#"a < b switch { 1 => 2 } < c switch { 2 => 3 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a < b switch { 1 => 2 } < c switch { 2 => 3 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ChainedSwitchExpression_02",
                    113,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ChainedSwitchExpression_02",
                    113,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ChainedSwitchExpression_02",
            113,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_01 (case 114)
#[test]
fn switch_expression_precedence_01() {
    let src = r#"a < b switch { true => 1 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a < b switch { true => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_01",
                    114,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_01",
                    114,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_01",
            114,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_02 (case 115)
#[test]
fn switch_expression_precedence_02() {
    let src = r#"a == b switch { true => 1 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a == b switch { true => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_02",
                    115,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_02",
                    115,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_02",
            115,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_03 (case 116)
#[test]
fn switch_expression_precedence_03() {
    let src = r#"a * b switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a * b switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_03",
                    116,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_03",
                    116,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_03",
            116,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_04 (case 117)
#[test]
fn switch_expression_precedence_04() {
    let src = r#"a + b switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_04",
                    117,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_04",
                    117,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_04",
            117,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_05 (case 118)
#[test]
fn switch_expression_precedence_05() {
    let src = r#"-a switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { -a switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_05",
                    118,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_05",
                    118,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_05",
            118,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_06 (case 119)
#[test]
fn switch_expression_precedence_06() {
    let src = r#"(Type)a switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Type)a switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_06",
                    119,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_06",
                    119,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_06",
            119,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_07 (case 120)
#[test]
fn switch_expression_precedence_07() {
    let src = r#"(Type)a++ switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Type)a++ switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_07",
                    120,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_07",
                    120,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_07",
            120,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_08 (case 121)
#[test]
fn switch_expression_precedence_08() {
    let src = r#"+a switch {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { +a switch {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_08",
                    121,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_08",
                    121,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_08",
            121,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_09 (case 122)
#[test]
fn switch_expression_precedence_09() {
    let src = r#"a switch {}.X"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a switch {}.X; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_09",
                    122,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_09",
                    122,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_09",
            122,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_10 (case 123)
#[test]
fn switch_expression_precedence_10() {
    let src = r#"a switch {}[i]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a switch {}[i]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_10",
                    123,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_10",
                    123,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_10",
            123,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_11 (case 124)
#[test]
fn switch_expression_precedence_11() {
    let src = r#"a switch {}(b)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a switch {}(b); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_11",
                    124,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_11",
                    124,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_11",
            124,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_12 (case 125)
#[test]
fn switch_expression_precedence_12() {
    let src = r#"a switch {}!"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a switch {}!; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_12",
                    125,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchExpressionPrecedence_12",
                    125,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchExpressionPrecedence_12",
            125,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.BrokenSwitchExpression_01 (case 126)
#[test]
fn broken_switch_expression_01() {
    let src = r#"(e switch {)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (e switch {); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_01",
                    126,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_01",
                    126,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "BrokenSwitchExpression_01",
            126,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.BrokenSwitchExpression_02 (case 127)
#[test]
fn broken_switch_expression_02() {
    let src = r#"(e switch {,)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (e switch {,); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_02",
                    127,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_02",
                    127,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "BrokenSwitchExpression_02",
            127,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.BrokenSwitchExpression_03 (case 128)
#[test]
fn broken_switch_expression_03() {
    let src = r#"e switch {,"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch {,; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_03",
                    128,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "BrokenSwitchExpression_03",
                    128,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "BrokenSwitchExpression_03",
            128,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedNamedConstantPatternInSwitchExpression (case 129)
#[test]
fn parenthesized_named_constant_pattern_in_switch_expression() {
    let src = r#"e switch { (X) => 1 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { (X) => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedNamedConstantPatternInSwitchExpression",
                    129,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedNamedConstantPatternInSwitchExpression",
                    129,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedNamedConstantPatternInSwitchExpression",
            129,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchCaseArmErrorRecovery_01 (case 130)
#[test]
fn switch_case_arm_error_recovery_01() {
    let src = r#"e switch { 1 => 1; 2 => 2 }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { 1 => 1; 2 => 2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchCaseArmErrorRecovery_01",
                    130,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchCaseArmErrorRecovery_01",
                    130,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchCaseArmErrorRecovery_01",
            130,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.SwitchCaseArmErrorRecovery_02 (case 131)
#[test]
fn switch_case_arm_error_recovery_02() {
    let src = r#"e switch { 1 => 1, 2 => 2; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { 1 => 1, 2 => 2; }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchCaseArmErrorRecovery_02",
                    131,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "SwitchCaseArmErrorRecovery_02",
                    131,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "SwitchCaseArmErrorRecovery_02",
            131,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.GenericPropertyPattern (case 132)
#[test]
fn generic_property_pattern() {
    let src = r#"e is A<B> {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "GenericPropertyPattern",
                    132,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "GenericPropertyPattern",
                    132,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "GenericPropertyPattern",
            132,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithDeclarationPattern (case 133)
#[test]
fn precedence_inversion_with_declaration_pattern() {
    let src = r#"o is C c + d"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is C c + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithDeclarationPattern",
                    133,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithDeclarationPattern",
                    133,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PrecedenceInversionWithDeclarationPattern",
            133,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithRecursivePattern (case 134)
#[test]
fn precedence_inversion_with_recursive_pattern() {
    let src = r#"o is {} + d"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is {} + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithRecursivePattern",
                    134,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithRecursivePattern",
                    134,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PrecedenceInversionWithRecursivePattern",
            134,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_01 (case 135)
#[test]
fn relational_pattern_precedence_01() {
    let src = r#"_ = e switch {
    < 0 < 0 => 0,
    == 4 < 4 => 4,
    != 5 < 5 => 5,
};"#;
    let expected = Some(ExpectedDiagnostics {
        count: 12,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_01",
                    135,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_01",
                    135,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RelationalPatternPrecedence_01",
            135,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_02 (case 136)
#[test]
fn relational_pattern_precedence_02() {
    let src = r#"_ = e switch {
    < 0 << 0 => 0,
    == 4 << 4 => 4,
    != 5 << 5 => 5,
};"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_02",
                    136,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_02",
                    136,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RelationalPatternPrecedence_02",
            136,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_03 (case 137)
#[test]
fn relational_pattern_precedence_03() {
    let src = r#"_ = e is < 4;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_03",
                    137,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_03",
                    137,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RelationalPatternPrecedence_03",
            137,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_04 (case 138)
#[test]
fn relational_pattern_precedence_04() {
    let src = r#"_ = e is < 4 < 4;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_04",
                    138,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_04",
                    138,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RelationalPatternPrecedence_04",
            138,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_05 (case 139)
#[test]
fn relational_pattern_precedence_05() {
    let src = r#"_ = e is < 4 << 4;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_05",
                    139,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "RelationalPatternPrecedence_05",
                    139,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "RelationalPatternPrecedence_05",
            139,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.WhenIsNotKeywordInIsExpression (case 140)
#[test]
fn when_is_not_keyword_in_is_expression() {
    let src = r#"_ = e is T when;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenIsNotKeywordInIsExpression",
                    140,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenIsNotKeywordInIsExpression",
                    140,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "WhenIsNotKeywordInIsExpression",
            140,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.WhenIsNotKeywordInRecursivePattern (case 141)
#[test]
fn when_is_not_keyword_in_recursive_pattern() {
    let src = r#"_ = e switch { T(X when) => 1, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenIsNotKeywordInRecursivePattern",
                    141,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "WhenIsNotKeywordInRecursivePattern",
                    141,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "WhenIsNotKeywordInRecursivePattern",
            141,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_01 (case 142)
#[test]
fn type_pattern_01() {
    let src = r#"_ = e is int or long;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_01",
                    142,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_01",
                    142,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_01",
            142,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_02 (case 143)
#[test]
fn type_pattern_02() {
    let src = r#"_ = e is int or System.Int64;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_02",
                    143,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_02",
                    143,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_02",
            143,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_03 (case 144)
#[test]
fn type_pattern_03() {
    let src = r#"_ = e switch { int or long => 1, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_03",
                    144,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_03",
                    144,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_03",
            144,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_04 (case 145)
#[test]
fn type_pattern_04() {
    let src = r#"_ = e switch { int or System.Int64 => 1, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_04",
                    145,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_04",
                    145,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_04",
            145,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_05 (case 146)
#[test]
fn type_pattern_05() {
    let src = r#"_ = e switch { T(int) => 1, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_05",
                    146,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_05",
                    146,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_05",
            146,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_06 (case 147)
#[test]
fn type_pattern_06() {
    let src = r#"_ = e switch { int => 1, long => 2, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_06",
                    147,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_06",
                    147,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_06",
            147,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.TypePattern_07 (case 148)
#[test]
fn type_pattern_07() {
    let src = r#"_ = e is (int) or string;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_07",
                    148,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "TypePattern_07",
                    148,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "TypePattern_07",
            148,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CompoundPattern_01 (case 149)
#[test]
fn compound_pattern_01() {
    let src = r#"bool isLetter(char c) => c is >= 'a' and <= 'z' or >= 'A' and <= 'Z';"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CompoundPattern_01",
                    149,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CompoundPattern_01",
                    149,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CompoundPattern_01",
            149,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_01 (case 150)
#[test]
fn combinator_as_designator_01() {
    let src = r#"_ = e is int and;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_01",
                    150,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_01",
                    150,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_01",
            150,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_02 (case 151)
#[test]
fn combinator_as_designator_02() {
    let src = r#"_ = e is int and < Z;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_02",
                    151,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_02",
                    151,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_02",
            151,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_03 (case 152)
#[test]
fn combinator_as_designator_03() {
    let src = r#"_ = e is int and && b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_03",
                    152,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_03",
                    152,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_03",
            152,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_04 (case 153)
#[test]
fn combinator_as_designator_04() {
    let src = r#"_ = e is int and int.MaxValue;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_04",
                    153,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_04",
                    153,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_04",
            153,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_05 (case 154)
#[test]
fn combinator_as_designator_05() {
    let src = r#"_ = e is int and MaxValue;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_05",
                    154,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_05",
                    154,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_05",
            154,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_06 (case 155)
#[test]
fn combinator_as_designator_06() {
    let src = r#"_ = e is int and ?? Z;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_06",
                    155,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_06",
                    155,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_06",
            155,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_07 (case 156)
#[test]
fn combinator_as_designator_07() {
    let src = r#"_ = e is int and ? a : b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_07",
                    156,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CombinatorAsDesignator_07",
                    156,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CombinatorAsDesignator_07",
            156,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithTypeTest (case 157)
#[test]
fn precedence_inversion_with_type_test() {
    let src = r#"o is int + d"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is int + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithTypeTest",
                    157,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithTypeTest",
                    157,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PrecedenceInversionWithTypeTest",
            157,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithBlockLambda (case 158)
#[test]
fn precedence_inversion_with_block_lambda() {
    let src = r#"() => {} + d"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { () => {} + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithBlockLambda",
                    158,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithBlockLambda",
                    158,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PrecedenceInversionWithBlockLambda",
            158,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithAnonymousMethod (case 159)
#[test]
fn precedence_inversion_with_anonymous_method() {
    let src = r#"delegate {} + d"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate {} + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithAnonymousMethod",
                    159,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "PrecedenceInversionWithAnonymousMethod",
                    159,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "PrecedenceInversionWithAnonymousMethod",
            159,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_01 (case 160)
#[test]
fn one_element_positional_01() {
    let src = r#"_ = e is (3);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_01",
                    160,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_01",
                    160,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_01",
            160,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_02 (case 161)
#[test]
fn one_element_positional_02() {
    let src = r#"_ = e is (A);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_02",
                    161,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_02",
                    161,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_02",
            161,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_03 (case 162)
#[test]
fn one_element_positional_03() {
    let src = r#"_ = e is (int);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_03",
                    162,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_03",
                    162,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_03",
            162,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_04 (case 163)
#[test]
fn one_element_positional_04() {
    let src = r#"_ = e is (Item1: int);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_04",
                    163,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_04",
                    163,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_04",
            163,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_05 (case 164)
#[test]
fn one_element_positional_05() {
    let src = r#"_ = e is (A) x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_05",
                    164,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_05",
                    164,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_05",
            164,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OneElementPositional_06 (case 165)
#[test]
fn one_element_positional_06() {
    let src = r#"_ = e is ((A, A)) x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_06",
                    165,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OneElementPositional_06",
                    165,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OneElementPositional_06",
            165,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ZeroElementPositional_01 (case 166)
#[test]
fn zero_element_positional_01() {
    let src = r#"_ = e is ();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_01",
                    166,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_01",
                    166,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ZeroElementPositional_01",
            166,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ZeroElementPositional_02 (case 167)
#[test]
fn zero_element_positional_02() {
    let src = r#"_ = e is () x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_02",
                    167,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_02",
                    167,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ZeroElementPositional_02",
            167,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ZeroElementPositional_03 (case 168)
#[test]
fn zero_element_positional_03() {
    let src = r#"_ = e is () {};"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_03",
                    168,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ZeroElementPositional_03",
                    168,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ZeroElementPositional_03",
            168,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.CastExpressionInPattern_01 (case 169)
#[test]
fn cast_expression_in_pattern_01() {
    let src = r#"_ = e is (int)+1;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CastExpressionInPattern_01",
                    169,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "CastExpressionInPattern_01",
                    169,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "CastExpressionInPattern_01",
            169,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ConjunctiveFollowedByPropertyPattern_01 (case 170)
#[test]
fn conjunctive_followed_by_property_pattern_01() {
    let src = r#"switch (e) { case {} and {}: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConjunctiveFollowedByPropertyPattern_01",
                    170,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConjunctiveFollowedByPropertyPattern_01",
                    170,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ConjunctiveFollowedByPropertyPattern_01",
            170,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ConjunctiveFollowedByTuplePattern_01 (case 171)
#[test]
fn conjunctive_followed_by_tuple_pattern_01() {
    let src = r#"switch (e) { case {} and (): break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConjunctiveFollowedByTuplePattern_01",
                    171,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConjunctiveFollowedByTuplePattern_01",
                    171,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ConjunctiveFollowedByTuplePattern_01",
            171,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_01 (case 172)
#[test]
fn parenthesized_relational_pattern_01() {
    let src = r#"_ = e is (>= 1);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_01",
                    172,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_01",
                    172,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedRelationalPattern_01",
            172,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_02 (case 173)
#[test]
fn parenthesized_relational_pattern_02() {
    let src = r#"_ = e switch { (>= 1) => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_02",
                    173,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_02",
                    173,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedRelationalPattern_02",
            173,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_03 (case 174)
#[test]
fn parenthesized_relational_pattern_03() {
    let src = r#"bool isAsciiLetter(char c) => c is (>= 'A' and <= 'Z') or (>= 'a' and <= 'z');"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_03",
                    174,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_03",
                    174,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedRelationalPattern_03",
            174,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_04 (case 175)
#[test]
fn parenthesized_relational_pattern_04() {
    let src = r#"_ = e is (<= 1, >= 2);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_04",
                    175,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ParenthesizedRelationalPattern_04",
                    175,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ParenthesizedRelationalPattern_04",
            175,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.AndPatternAssociativity_01 (case 176)
#[test]
fn and_pattern_associativity_01() {
    let src = r#"_ = e is A and B and C;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "AndPatternAssociativity_01",
                    176,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "AndPatternAssociativity_01",
                    176,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "AndPatternAssociativity_01",
            176,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.OrPatternAssociativity_01 (case 177)
#[test]
fn or_pattern_associativity_01() {
    let src = r#"_ = e is A or B or C;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OrPatternAssociativity_01",
                    177,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "OrPatternAssociativity_01",
                    177,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "OrPatternAssociativity_01",
            177,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.GenericTypeAsTypePatternInSwitchExpression (case 178)
#[test]
fn generic_type_as_type_pattern_in_switch_expression() {
    let src = r#"_ = e switch { List<X> => 1, List<Y> => 2, };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "GenericTypeAsTypePatternInSwitchExpression",
                    178,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "GenericTypeAsTypePatternInSwitchExpression",
                    178,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "GenericTypeAsTypePatternInSwitchExpression",
            178,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_PredefinedType (case 179)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_predefined_type() {
    let src = r#"_ = e switch { int? => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_PredefinedType",
                    179,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_PredefinedType",
                    179,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchExpression_PredefinedType",
            179,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_PredefinedType (case 180)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_predefined_type() {
    let src = r#"switch(a) { case int?: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_PredefinedType",
                    180,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_PredefinedType",
                    180,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchStatement_PredefinedType",
            180,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized (case 181)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_predefined_type_parenthesized() {
    let src = r#"_ = e switch { (int?) => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized",
                    181,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized",
                    181,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized",
            181,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized (case 182)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_predefined_type_parenthesized() {
    let src = r#"switch(a) { case (int?): break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized",
                    182,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized",
                    182,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized",
            182,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression (case 183)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression() {
    let src = r#"_ = e switch { a? => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression",
                    183,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression",
                    183,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchExpression",
            183,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement (case 184)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement() {
    let src = r#"switch(a) { case a?: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement",
                    184,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement",
                    184,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchStatement",
            184,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_Parenthesized (case 185)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_parenthesized() {
    let src = r#"_ = e switch { (a?) => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_Parenthesized",
                    185,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchExpression_Parenthesized",
                    185,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchExpression_Parenthesized",
            185,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_Parenthesized (case 186)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_parenthesized() {
    let src = r#"switch(a) { case (a?): break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_Parenthesized",
                    186,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "NullableTypeAsTypePatternInSwitchStatement_Parenthesized",
                    186,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "NullableTypeAsTypePatternInSwitchStatement_Parenthesized",
            186,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ConditionalAsConstantPatternInSwitchExpression (case 187)
#[test]
fn conditional_as_constant_pattern_in_switch_expression() {
    let src = r#"_ = e switch { (a?x:y) => 1 };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchExpression",
                    187,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchExpression",
                    187,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ConditionalAsConstantPatternInSwitchExpression",
            187,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ConditionalAsConstantPatternInSwitchStatement (case 188)
#[test]
fn conditional_as_constant_pattern_in_switch_statement() {
    let src = r#"switch(a) { case a?x:y: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchStatement",
                    188,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchStatement",
                    188,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ConditionalAsConstantPatternInSwitchStatement",
            188,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.ConditionalAsConstantPatternInSwitchStatement_Parenthesized (case 189)
#[test]
fn conditional_as_constant_pattern_in_switch_statement_parenthesized() {
    let src = r#"switch(a) { case (a?x:y): break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchStatement_Parenthesized",
                    189,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "ConditionalAsConstantPatternInSwitchStatement_Parenthesized",
                    189,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(
            rest.fragment().trim().is_empty(),
            "Unconsumed input: {}",
            rest.fragment()
        );
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "ConditionalAsConstantPatternInSwitchStatement_Parenthesized",
            189,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: PatternParsingTests.InvalidPropertyPattern (case 190)
#[test]
fn invalid_property_pattern() {
    let src = r#"new object() is { {}: 1 }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object() is { {}: 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "InvalidPropertyPattern",
                    190,
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
                    "pattern_parsing_tests",
                    "PatternParsingTests",
                    "InvalidPropertyPattern",
                    190,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "pattern_parsing_tests",
            "PatternParsingTests",
            "InvalidPropertyPattern",
            190,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}
