// Auto-generated from Roslyn: NullableParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws_spanned;
use bsharp_syntax::span::Span;
/// Roslyn: NullableParsingTests.PartialAccessibilityAndNullableArray (case 1)
#[test]
fn partial_accessibility_and_nullable_array() {
    let src = r#"class C
{
    privat C[]? F;
}"#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "PartialAccessibilityAndNullableArray",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "PartialAccessibilityAndNullableArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "PartialAccessibilityAndNullableArray",
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

/// Roslyn: NullableParsingTests.NullableArray_Cast_01 (case 2)
#[test]
fn nullable_array_cast_01() {
    let src = r#"(object[]?)null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[]?)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_01",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_01",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Cast_01",
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

/// Roslyn: NullableParsingTests.NullableArray_Cast_02 (case 3)
#[test]
fn nullable_array_cast_02() {
    let src = r#"(object[]??)null"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[]??)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_02",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_02",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Cast_02",
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

/// Roslyn: NullableParsingTests.NullableArray_Cast_03 (case 4)
#[test]
fn nullable_array_cast_03() {
    let src = r#"(object[?])null"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[?])null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_03",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_03",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Cast_03",
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

/// Roslyn: NullableParsingTests.NullableArray_Cast_04 (case 5)
#[test]
fn nullable_array_cast_04() {
    let src = r#"(object?[]?[])null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object?[]?[])null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_04",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_04",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Cast_04",
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

/// Roslyn: NullableParsingTests.NullableArray_Cast_05 (case 6)
#[test]
fn nullable_array_cast_05() {
    let src = r#"(object[][]?[]?)null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[][]?[]?)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_05",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Cast_05",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Cast_05",
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

/// Roslyn: NullableParsingTests.ConditionalOperator_NotNullableType (case 7)
#[test]
fn conditional_operator_not_nullable_type() {
    let src = r#"x is T ? y : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NotNullableType",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NotNullableType",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConditionalOperator_NotNullableType",
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

/// Roslyn: NullableParsingTests.ConditionalOperator_NullableType (case 8)
#[test]
fn conditional_operator_nullable_type() {
    let src = r#"x is T ? ? y : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T ? ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NullableType",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NullableType",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConditionalOperator_NullableType",
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

/// Roslyn: NullableParsingTests.ConditionalOperator_NotNullableArray (case 9)
#[test]
fn conditional_operator_not_nullable_array() {
    let src = r#"x is T[] ? y : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T[] ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NotNullableArray",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NotNullableArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConditionalOperator_NotNullableArray",
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

/// Roslyn: NullableParsingTests.ConditionalOperator_NullableArray (case 10)
#[test]
fn conditional_operator_nullable_array() {
    let src = r#"x is T[] ? ? y : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T[] ? ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NullableArray",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConditionalOperator_NullableArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConditionalOperator_NullableArray",
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

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NotNullableType (case 11)
#[test]
fn null_coalescing_operator_not_nullable_type() {
    let src = r#"x as T?? y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NotNullableType",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NotNullableType",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullCoalescingOperator_NotNullableType",
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

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NullableType (case 12)
#[test]
fn null_coalescing_operator_nullable_type() {
    let src = r#"x as T? ?? y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T? ?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableType",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableType",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullCoalescingOperator_NullableType",
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

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NullableType_Invalid (case 13)
#[test]
fn null_coalescing_operator_nullable_type_invalid() {
    let src = r#"x as T??? y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T??? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableType_Invalid",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableType_Invalid",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullCoalescingOperator_NullableType_Invalid",
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

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NotNullableArray (case 14)
#[test]
fn null_coalescing_operator_not_nullable_array() {
    let src = r#"x as T[] ?? y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T[] ?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NotNullableArray",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NotNullableArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullCoalescingOperator_NotNullableArray",
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

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NullableArray (case 15)
#[test]
fn null_coalescing_operator_nullable_array() {
    let src = r#"x as T[] ? ?? y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T[] ? ?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableArray",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullCoalescingOperator_NullableArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullCoalescingOperator_NullableArray",
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

/// Roslyn: NullableParsingTests.DeclarationPattern_NullableType (case 16)
#[test]
fn declaration_pattern_nullable_type() {
    let src = r#"switch (e) { case T? t: break; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_NullableType",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_NullableType",
                    16,
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DeclarationPattern_NullableType",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.DeclarationPattern_ArrayOfNullableType (case 17)
#[test]
fn declaration_pattern_array_of_nullable_type() {
    let src = r#"switch (e) { case T?[] t: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_ArrayOfNullableType",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_ArrayOfNullableType",
                    17,
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DeclarationPattern_ArrayOfNullableType",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.DeclarationPattern_NullableArrayOfArray (case 18)
#[test]
fn declaration_pattern_nullable_array_of_array() {
    let src = r#"switch (e) { case T[]?[] t: break; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_NullableArrayOfArray",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPattern_NullableArrayOfArray",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DeclarationPattern_NullableArrayOfArray",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.NullableArray_TypeArgument (case 19)
#[test]
fn nullable_array_type_argument() {
    let src = r#"F<A[]?, object[]?>()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F<A[]?, object[]?>(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_TypeArgument",
                    19,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_TypeArgument",
                    19,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_TypeArgument",
            19,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.NullableArray_TupleType (case 20)
#[test]
fn nullable_array_tuple_type() {
    let src = r#"(object[]?, A[]?) t;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_TupleType",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_TupleType",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_TupleType",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.NullableArray_Using (case 21)
#[test]
fn nullable_array_using() {
    let src = r#"using (A[]? a = b) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Using",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Using",
                    21,
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Using",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.NullableArray_Query (case 22)
#[test]
fn nullable_array_query() {
    let src = r#"from A[]? a in b select a"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { from A[]? a in b select a; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Query",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_Query",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_Query",
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

/// Roslyn: NullableParsingTests.NullableArray_ExplicitlyTypedLambda (case 23)
#[test]
fn nullable_array_explicitly_typed_lambda() {
    let src = r#"F((object[]? a) => a)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F((object[]? a) => a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_ExplicitlyTypedLambda",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_ExplicitlyTypedLambda",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_ExplicitlyTypedLambda",
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

/// Roslyn: NullableParsingTests.NullableArray_PartialMember (case 24)
#[test]
fn nullable_array_partial_member() {
    let src = r#"class C
{
    partial A[]? F();
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_PartialMember",
                    24,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "NullableArray_PartialMember",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "NullableArray_PartialMember",
            24,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: NullableParsingTests.CreateNullableArray_01 (case 25)
#[test]
fn create_nullable_array_01() {
    let src = r#"new object[,][]?"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,][]?; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_01",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_01",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_01",
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

/// Roslyn: NullableParsingTests.CreateNullableArray_02 (case 26)
#[test]
fn create_nullable_array_02() {
    let src = r#"new object[,][]? { 1, 2 }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,][]? { 1, 2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_02",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_02",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_02",
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

/// Roslyn: NullableParsingTests.CreateNullableArray_03 (case 27)
#[test]
fn create_nullable_array_03() {
    let src = r#"new object[1,2]?[3]?[4]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[1,2]?[3]?[4]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_03",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_03",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_03",
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

/// Roslyn: NullableParsingTests.CreateNullableArray_04 (case 28)
#[test]
fn create_nullable_array_04() {
    let src = r#"new object[,]?[]?[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,]?[]?[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_04",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_04",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_04",
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

/// Roslyn: NullableParsingTests.CreateNullableArray_05 (case 29)
#[test]
fn create_nullable_array_05() {
    let src = r#"new object[1,2]?[3]?[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[1,2]?[3]?[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_05",
                    29,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_05",
                    29,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_05",
            29,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.CreateNullableArray_06 (case 30)
#[test]
fn create_nullable_array_06() {
    let src = r#"new object[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_06",
                    30,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_06",
                    30,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_06",
            30,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.CreateNullableArray_07 (case 31)
#[test]
fn create_nullable_array_07() {
    let src = r#"new object[1]?[2,3]?[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[1]?[2,3]?[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_07",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "CreateNullableArray_07",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "CreateNullableArray_07",
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

/// Roslyn: NullableParsingTests.IsExpressionOfNullableTypeInStatement (case 32)
#[test]
fn is_expression_of_nullable_type_in_statement() {
    let src = r#"_ = x is Type?;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "IsExpressionOfNullableTypeInStatement",
                    32,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "IsExpressionOfNullableTypeInStatement",
                    32,
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "IsExpressionOfNullableTypeInStatement",
            32,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.DeclarationPatternOfNullableTypeInStatement (case 33)
#[test]
fn declaration_pattern_of_nullable_type_in_statement() {
    let src = r#"_ = x is Type? t;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPatternOfNullableTypeInStatement",
                    33,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DeclarationPatternOfNullableTypeInStatement",
                    33,
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DeclarationPatternOfNullableTypeInStatement",
            33,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: NullableParsingTests.DisjunctivePattern_NullableType1 (case 34)
#[test]
fn disjunctive_pattern_nullable_type_1() {
    let src = r#"x is int? or string?"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int? or string?; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DisjunctivePattern_NullableType1",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DisjunctivePattern_NullableType1",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DisjunctivePattern_NullableType1",
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

/// Roslyn: NullableParsingTests.DisjunctivePattern_NullableType2 (case 35)
#[test]
fn disjunctive_pattern_nullable_type_2() {
    let src = r#"x is int? i or string? s"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int? i or string? s; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DisjunctivePattern_NullableType2",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "DisjunctivePattern_NullableType2",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "DisjunctivePattern_NullableType2",
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

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType1 (case 36)
#[test]
fn conjunctive_pattern_nullable_type_1() {
    let src = r#"x is Type? and { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? and { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType1",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType1",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_NullableType1",
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

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType2 (case 37)
#[test]
fn conjunctive_pattern_nullable_type_2() {
    let src = r#"x is Type? t and { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType2",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType2",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_NullableType2",
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

/// Roslyn: NullableParsingTests.ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3 (case 38)
#[test]
fn conjunctive_pattern_conditional_expression_instead_of_nullable_type_3() {
    let src = r#"x is Type? and (1, 2)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? and (1, 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3",
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

/// Roslyn: NullableParsingTests.ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3_2 (case 39)
#[test]
fn conjunctive_pattern_conditional_expression_instead_of_nullable_type_3_2() {
    let src = r#"x is Type ? f(1, 2) : 0"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type ? f(1, 2) : 0; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3_2",
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3_2",
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
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType3_2",
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

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType4 (case 40)
#[test]
fn conjunctive_pattern_nullable_type_4() {
    let src = r#"x is Type? t and (1, 2)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and (1, 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType4",
                    40,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType4",
                    40,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_NullableType4",
            40,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5 (case 41)
#[test]
fn conjunctive_pattern_conditional_expression_instead_of_nullable_type_5() {
    let src = r#"x is Type? and []"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? and []; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5",
                    41,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5",
                    41,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5",
            41,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5_2 (case 42)
#[test]
fn conjunctive_pattern_conditional_expression_instead_of_nullable_type_5_2() {
    let src = r#"x is Type ? dict[key] : default"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type ? dict[key] : default; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5_2",
                    42,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5_2",
                    42,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_ConditionalExpressionInsteadOfNullableType5_2",
            42,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType6 (case 43)
#[test]
fn conjunctive_pattern_nullable_type_6() {
    let src = r#"x is Type? t and []"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and []; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType6",
                    43,
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
                    "nullable_parsing_tests",
                    "NullableParsingTests",
                    "ConjunctivePattern_NullableType6",
                    43,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "nullable_parsing_tests",
            "NullableParsingTests",
            "ConjunctivePattern_NullableType6",
            43,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}
