// Auto-generated from Roslyn: CollectionExpressionParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: CollectionExpressionParsingTests.CollectionExpressionParsingDoesNotProduceLangVersionError (case 1)
#[test]
fn collection_expression_parsing_does_not_produce_lang_version_error() {
    let src = r#"[A, B]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpressionParsingDoesNotProduceLangVersionError",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpressionParsingDoesNotProduceLangVersionError",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpressionParsingDoesNotProduceLangVersionError",
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

/// Roslyn: CollectionExpressionParsingTests.ExpressionDotAccess (case 2)
#[test]
fn expression_dot_access() {
    let src = r#"_ = [A, B].C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionDotAccess",
                    2,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionDotAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionDotAccess",
            2,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess (case 3)
#[test]
fn top_level_dot_access() {
    let src = r#"[A, B].C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess",
                    3,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess",
            3,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_GlobalAttributeAmbiguity1 (case 4)
#[test]
fn top_level_dot_access_global_attribute_ambiguity_1() {
    let src = r#"[assembly: A, B].C();"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_GlobalAttributeAmbiguity1",
                    4,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_GlobalAttributeAmbiguity1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_GlobalAttributeAmbiguity1",
            4,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity2A (case 5)
#[test]
fn top_level_dot_access_attribute_ambiguity_2_a() {
    let src = r#"[return: A, B].C();"#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity2A",
                    5,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity2A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity2A",
            5,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity2B (case 6)
#[test]
fn top_level_dot_access_attribute_ambiguity_2_b() {
    let src = r#"[return: A, B] void F() { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity2B",
                    6,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity2B",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity2B",
            6,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity3A (case 7)
#[test]
fn top_level_dot_access_attribute_ambiguity_3_a() {
    let src = r#"[method: A, B].C();"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity3A",
                    7,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity3A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity3A",
            7,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity3B (case 8)
#[test]
fn top_level_dot_access_attribute_ambiguity_3_b() {
    let src = r#"[method: A, B] void F() { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity3B",
                    8,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity3B",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity3B",
            8,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity4A (case 9)
#[test]
fn top_level_dot_access_attribute_ambiguity_4_a() {
    let src = r#"[return: A].C();"#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity4A",
                    9,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity4A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity4A",
            9,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_AttributeAmbiguity4B (case 10)
#[test]
fn top_level_dot_access_attribute_ambiguity_4_b() {
    let src = r#"[return: A] void F() { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity4B",
                    10,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_AttributeAmbiguity4B",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_AttributeAmbiguity4B",
            10,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess_GlobalAttributeAmbiguity2 (case 11)
#[test]
fn top_level_dot_access_global_attribute_ambiguity_2() {
    let src = r#"[module: A, B].C();"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_GlobalAttributeAmbiguity2",
                    11,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelDotAccess_GlobalAttributeAmbiguity2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelDotAccess_GlobalAttributeAmbiguity2",
            11,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionNullSafeAccess (case 12)
#[test]
fn expression_null_safe_access() {
    let src = r#"_ = [A, B]?.C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionNullSafeAccess",
                    12,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionNullSafeAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionNullSafeAccess",
            12,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelNullSafeAccess (case 13)
#[test]
fn top_level_null_safe_access() {
    let src = r#"[A, B]?.C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelNullSafeAccess",
                    13,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelNullSafeAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelNullSafeAccess",
            13,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionPointerAccess (case 14)
#[test]
fn expression_pointer_access() {
    let src = r#"_ = [A, B]->C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionPointerAccess",
                    14,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionPointerAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionPointerAccess",
            14,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelPointerAccess (case 15)
#[test]
fn top_level_pointer_access() {
    let src = r#"[A, B]->C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelPointerAccess",
                    15,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelPointerAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelPointerAccess",
            15,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelDotAccessStatement (case 16)
#[test]
fn attribute_on_top_level_dot_access_statement() {
    let src = r#"[A] [B].C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelDotAccessStatement",
                    16,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelDotAccessStatement",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelDotAccessStatement",
            16,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttemptToImmediatelyIndexInTopLevelStatement (case 17)
#[test]
fn attempt_to_immediately_index_in_top_level_statement() {
    let src = r#"["A", "B"][0].C();"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttemptToImmediatelyIndexInTopLevelStatement",
                    17,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttemptToImmediatelyIndexInTopLevelStatement",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttemptToImmediatelyIndexInTopLevelStatement",
            17,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AlwaysParsedAsAttributeInsideNamespace (case 18)
#[test]
fn always_parsed_as_attribute_inside_namespace() {
    let src = r#"
                namespace A;
                [B].C();
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AlwaysParsedAsAttributeInsideNamespace",
                    18,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AlwaysParsedAsAttributeInsideNamespace",
                    18,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AlwaysParsedAsAttributeInsideNamespace",
            18,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionIs (case 19)
#[test]
fn expression_is() {
    let src = r#"_ = [A, B] is [A, B];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionIs",
                    19,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionIs",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionIs",
            19,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionWith (case 20)
#[test]
fn expression_with() {
    let src = r#"_ = [A, B] with { };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionWith",
                    20,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionWith",
                    20,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionWith",
            20,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionSwitch (case 21)
#[test]
fn expression_switch() {
    let src = r#"_ = [A, B] switch { _ => M() };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionSwitch",
                    21,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ExpressionSwitch",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ExpressionSwitch",
            21,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelSwitch (case 22)
#[test]
fn top_level_switch() {
    let src = r#"[A, B] switch { _ => M() };"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelSwitch",
                    22,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TopLevelSwitch",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TopLevelSwitch",
            22,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.StatementLevelSwitch (case 23)
#[test]
fn statement_level_switch() {
    let src = r#"
            class C
            {
                void M()
                {
                    [A, B] switch { _ => M() };
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "StatementLevelSwitch",
                    23,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "StatementLevelSwitch",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "StatementLevelSwitch",
            23,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.BinaryOperator (case 24)
#[test]
fn binary_operator() {
    let src = r#"_ = [A, B] + [C, D];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "BinaryOperator",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "BinaryOperator",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "BinaryOperator",
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

/// Roslyn: CollectionExpressionParsingTests.EmptyCollection (case 25)
#[test]
fn empty_collection() {
    let src = r#"_ = [];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "EmptyCollection",
                    25,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "EmptyCollection",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "EmptyCollection",
            25,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionOfEmptyCollection (case 26)
#[test]
fn collection_of_empty_collection() {
    let src = r#"_ = [[]];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionOfEmptyCollection",
                    26,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionOfEmptyCollection",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionOfEmptyCollection",
            26,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionaryOfEmptyCollections (case 27)
#[test]
fn dictionary_of_empty_collections() {
    let src = r#"_ = [[]: []];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryOfEmptyCollections",
                    27,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryOfEmptyCollections",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryOfEmptyCollections",
            27,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionarySyntaxMissingKey (case 28)
#[test]
fn dictionary_syntax_missing_key() {
    let src = r#"_ = [:B];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingKey",
                    28,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingKey",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionarySyntaxMissingKey",
            28,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionarySyntaxMissingValue (case 29)
#[test]
fn dictionary_syntax_missing_value() {
    let src = r#"_ = [A:];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingValue",
                    29,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingValue",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionarySyntaxMissingValue",
            29,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionarySyntaxMissingKeyAndValue (case 30)
#[test]
fn dictionary_syntax_missing_key_and_value() {
    let src = r#"_ = [:];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingKeyAndValue",
                    30,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionarySyntaxMissingKeyAndValue",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionarySyntaxMissingKeyAndValue",
            30,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithTypeExpressions (case 31)
#[test]
fn dictionary_with_type_expressions() {
    let src = r#"_ = [A::B: C::D];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithTypeExpressions",
                    31,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithTypeExpressions",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithTypeExpressions",
            31,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithConditional1 (case 32)
#[test]
fn dictionary_with_conditional_1() {
    let src = r#"[a ? b : c : d]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ? b : c : d]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithConditional1",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithConditional2 (case 33)
#[test]
fn dictionary_with_conditional_2() {
    let src = r#"[a : b ? c : d]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a : b ? c : d]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithConditional2",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithConditional3 (case 34)
#[test]
fn dictionary_with_conditional_3() {
    let src = r#"[a ? b : c : d ? e : f]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ? b : c : d ? e : f]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithConditional3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithConditional3",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithNullCoalesce1 (case 35)
#[test]
fn dictionary_with_null_coalesce_1() {
    let src = r#"[a ?? b : c]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ?? b : c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithNullCoalesce1",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithNullCoalesce2 (case 36)
#[test]
fn dictionary_with_null_coalesce_2() {
    let src = r#"[a : b ?? c]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a : b ?? c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithNullCoalesce2",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithNullCoalesce3 (case 37)
#[test]
fn dictionary_with_null_coalesce_3() {
    let src = r#"[a ?? b : c ?? d]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ?? b : c ?? d]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithNullCoalesce3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithNullCoalesce3",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithQuery1 (case 38)
#[test]
fn dictionary_with_query_1() {
    let src = r#"[from x in y select x : c]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [from x in y select x : c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithQuery1",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithQuery2 (case 39)
#[test]
fn dictionary_with_query_2() {
    let src = r#"[a : from x in y select x]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a : from x in y select x]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithQuery2",
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

/// Roslyn: CollectionExpressionParsingTests.DictionaryWithQuery3 (case 40)
#[test]
fn dictionary_with_query_3() {
    let src = r#"[from a in b select a : from x in y select x]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [from a in b select a : from x in y select x]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "DictionaryWithQuery3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "DictionaryWithQuery3",
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

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity1 (case 41)
#[test]
fn conditional_ambiguity_1() {
    let src = r#"[a ? [b] : c]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ? [b] : c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity1",
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

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity1A (case 42)
#[test]
fn conditional_ambiguity_1_a() {
    let src = r#"[A] ? [B] : C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] ? [B] : C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1A",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity1A",
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

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity1B (case 43)
#[test]
fn conditional_ambiguity_1_b() {
    let src = r#"[A] ? [B] . C"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] ? [B] . C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1B",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity1B",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity1B",
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

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity2 (case 44)
#[test]
fn conditional_ambiguity_2() {
    let src = r#"[(a ? [b]) : c]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [(a ? [b]) : c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity2",
                    44,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity2",
                    44,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity2",
            44,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity3 (case 45)
#[test]
fn conditional_ambiguity_3() {
    let src = r#"a ? [b] : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? [b] : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity3",
                    45,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity3",
                    45,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity3",
            45,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity3A (case 46)
#[test]
fn conditional_ambiguity_3_a() {
    let src = r#"a ? [b].M() : c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? [b].M() : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity3A",
                    46,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity3A",
                    46,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity3A",
            46,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity4 (case 47)
#[test]
fn conditional_ambiguity_4() {
    let src = r#"a ? b?[c] : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity4",
                    47,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity4",
                    47,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity4",
            47,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity4A (case 48)
#[test]
fn conditional_ambiguity_4_a() {
    let src = r#"a ? b?[c].M() : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c].M() : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity4A",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity4A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity4A",
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

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity5 (case 49)
#[test]
fn conditional_ambiguity_5() {
    let src = r#"a ? b ? [c] : d : e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b ? [c] : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity5",
                    49,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity5",
                    49,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity5",
            49,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity5A (case 50)
#[test]
fn conditional_ambiguity_5_a() {
    let src = r#"a ? b ? [c].M() : d : e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b ? [c].M() : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity5A",
                    50,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity5A",
                    50,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity5A",
            50,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity6 (case 51)
#[test]
fn conditional_ambiguity_6() {
    let src = r#"a?[c] ? b : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c] ? b : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity6",
                    51,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity6",
                    51,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity6",
            51,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity6A (case 52)
#[test]
fn conditional_ambiguity_6_a() {
    let src = r#"a?[c].M() ? b : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c].M() ? b : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity6A",
                    52,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity6A",
                    52,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity6A",
            52,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity7 (case 53)
#[test]
fn conditional_ambiguity_7() {
    let src = r#"a?[c] ? b : d : e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c] ? b : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity7",
                    53,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity7",
                    53,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity7",
            53,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity7A (case 54)
#[test]
fn conditional_ambiguity_7_a() {
    let src = r#"a?[c].M() ? b : d : e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c].M() ? b : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity7A",
                    54,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity7A",
                    54,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity7A",
            54,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity8 (case 55)
#[test]
fn conditional_ambiguity_8() {
    let src = r#"a ? b?[() => { var v = x ? [y] : z; }] : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[() => { var v = x ? [y] : z; }] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity8",
                    55,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity8",
                    55,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity8",
            55,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity9 (case 56)
#[test]
fn conditional_ambiguity_9() {
    let src = r#"a ? b?[delegate { var v = x ? [y] : z; }] : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[delegate { var v = x ? [y] : z; }] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity9",
                    56,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity9",
                    56,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity9",
            56,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity10 (case 57)
#[test]
fn conditional_ambiguity_10() {
    let src = r#"a ? b?[() => x ? [y] : z] : d"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[() => x ? [y] : z] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity10",
                    57,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity10",
                    57,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity10",
            57,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity11 (case 58)
#[test]
fn conditional_ambiguity_11() {
    let src = r#"a ? b?[c] : d ? e?[f] : g"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e?[f] : g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity11",
                    58,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity11",
                    58,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity11",
            58,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity12 (case 59)
#[test]
fn conditional_ambiguity_12() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity12",
                    59,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity12",
                    59,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity12",
            59,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity12A (case 60)
#[test]
fn conditional_ambiguity_12_a() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h : i"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h : i; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity12A",
                    60,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity12A",
                    60,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity12A",
            60,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity13 (case 61)
#[test]
fn conditional_ambiguity_13() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h : i : j"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h : i : j; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity13",
                    61,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity13",
                    61,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity13",
            61,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity14 (case 62)
#[test]
fn conditional_ambiguity_14() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h : i : j : k"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h : i : j : k; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity14",
                    62,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity14",
                    62,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity14",
            62,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity15 (case 63)
#[test]
fn conditional_ambiguity_15() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h : i : j : k : m"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h : i : j : k : m; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity15",
                    63,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ConditionalAmbiguity15",
                    63,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ConditionalAmbiguity15",
            63,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity1 (case 64)
#[test]
fn cast_versus_index_ambiguity_1() {
    let src = r#"(type)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (type)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity1",
                    64,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity1",
                    64,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity1",
            64,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity2 (case 65)
#[test]
fn cast_versus_index_ambiguity_2() {
    let src = r#"(ImmutableArray<int>)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (ImmutableArray<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity2",
                    65,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity2",
                    65,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity2",
            65,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity3 (case 66)
#[test]
fn cast_versus_index_ambiguity_3() {
    let src = r#"(Dotted.ImmutableArray<int>)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Dotted.ImmutableArray<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity3",
                    66,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity3",
                    66,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity3",
            66,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity4 (case 67)
#[test]
fn cast_versus_index_ambiguity_4() {
    let src = r#"(ColonColon::ImmutableArray<int>)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (ColonColon::ImmutableArray<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity4",
                    67,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity4",
                    67,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity4",
            67,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity5 (case 68)
#[test]
fn cast_versus_index_ambiguity_5() {
    let src = r#"(NotCast())[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (NotCast())[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity5",
                    68,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity5",
                    68,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity5",
            68,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity6 (case 69)
#[test]
fn cast_versus_index_ambiguity_6() {
    let src = r#"(Not + Cast)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Not + Cast)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity6",
                    69,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity6",
                    69,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity6",
            69,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity7 (case 70)
#[test]
fn cast_versus_index_ambiguity_7() {
    let src = r#"(List<int>?)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (List<int>?)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity7",
                    70,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity7",
                    70,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity7",
            70,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity8 (case 71)
#[test]
fn cast_versus_index_ambiguity_8() {
    let src = r#"(int[])[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int[])[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity8",
                    71,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity8",
                    71,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity8",
            71,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity9 (case 72)
#[test]
fn cast_versus_index_ambiguity_9() {
    let src = r#"((int,int)[])[(1,2), (2,3), (3,4)]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ((int,int)[])[(1,2), (2,3), (3,4)]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity9",
                    72,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity9",
                    72,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity9",
            72,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity10 (case 73)
#[test]
fn cast_versus_index_ambiguity_10() {
    let src = r#"((A, B))[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ((A, B))[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity10",
                    73,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity10",
                    73,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity10",
            73,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity11 (case 74)
#[test]
fn cast_versus_index_ambiguity_11() {
    let src = r#"((A))[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ((A))[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity11",
                    74,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity11",
                    74,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity11",
            74,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity12 (case 75)
#[test]
fn cast_versus_index_ambiguity_12() {
    let src = r#"(int[]?)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int[]?)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity12",
                    75,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity12",
                    75,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity12",
            75,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity13 (case 76)
#[test]
fn cast_versus_index_ambiguity_13() {
    let src = r#"(int?[])[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int?[])[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity13",
                    76,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity13",
                    76,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity13",
            76,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity14 (case 77)
#[test]
fn cast_versus_index_ambiguity_14() {
    let src = r#"(type)([1, 2, 3])"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (type)([1, 2, 3]); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity14",
                    77,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity14",
                    77,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity14",
            77,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity15 (case 78)
#[test]
fn cast_versus_index_ambiguity_15() {
    let src = r#"(alias::type)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (alias::type)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity15",
                    78,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity15",
                    78,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity15",
            78,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity16 (case 79)
#[test]
fn cast_versus_index_ambiguity_16() {
    let src = r#"(a[b])[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a[b])[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity16",
                    79,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity16",
                    79,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity16",
            79,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity17 (case 80)
#[test]
fn cast_versus_index_ambiguity_17() {
    let src = r#"(a ? b : c)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a ? b : c)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity17",
                    80,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity17",
                    80,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity17",
            80,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity18 (case 81)
#[test]
fn cast_versus_index_ambiguity_18() {
    let src = r#"(a * b)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a * b)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity18",
                    81,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity18",
                    81,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity18",
            81,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity19 (case 82)
#[test]
fn cast_versus_index_ambiguity_19() {
    let src = r#"(a < b > c)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a < b > c)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity19",
                    82,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity19",
                    82,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity19",
            82,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity20 (case 83)
#[test]
fn cast_versus_index_ambiguity_20() {
    let src = r#"(alias::type.member)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (alias::type.member)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity20",
                    83,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity20",
                    83,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity20",
            83,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity21 (case 84)
#[test]
fn cast_versus_index_ambiguity_21() {
    let src = r#"(alias::type<int>)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (alias::type<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity21",
                    84,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity21",
                    84,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity21",
            84,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity22 (case 85)
#[test]
fn cast_versus_index_ambiguity_22() {
    let src = r#"(alias::type.type2<int>)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (alias::type.type2<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity22",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity22",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity22",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity23 (case 86)
#[test]
fn cast_versus_index_ambiguity_23() {
    let src = r#"(A[])[0]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A[])[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity23",
                    86,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity23",
                    86,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity23",
            86,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_A (case 87)
#[test]
fn cast_versus_index_ambiguity_24_a() {
    let src = r#"(A)[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_A",
                    87,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_A",
                    87,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity24_A",
            87,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_B (case 88)
#[test]
fn cast_versus_index_ambiguity_24_b() {
    let src = r#"(A)[1]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_B",
                    88,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_B",
                    88,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity24_B",
            88,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_C (case 89)
#[test]
fn cast_versus_index_ambiguity_24_c() {
    let src = r#"(A)[1:2]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[1:2]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_C",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_C",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity24_C",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_D (case 90)
#[test]
fn cast_versus_index_ambiguity_24_d() {
    let src = r#"(A)[..B]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[..B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_D",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity24_D",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity24_D",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity25 (case 91)
#[test]
fn cast_versus_index_ambiguity_25() {
    let src = r#"(A[])[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A[])[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity25",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity25",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity25",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity26 (case 92)
#[test]
fn cast_versus_index_ambiguity_26() {
    let src = r#"((int, int))[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ((int, int))[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity26",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity26",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity26",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity27 (case 93)
#[test]
fn cast_versus_index_ambiguity_27() {
    let src = r#"(a < b > . c)[1, 2, 3]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a < b > . c)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity27",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity27",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity27",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity28 (case 94)
#[test]
fn cast_versus_index_ambiguity_28() {
    let src = r#"(A<>)[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A<>)[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity28",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity28",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity28",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity29 (case 95)
#[test]
fn cast_versus_index_ambiguity_29() {
    let src = r#"(A<,>)[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A<,>)[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity29",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity29",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity29",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity30 (case 96)
#[test]
fn cast_versus_index_ambiguity_30() {
    let src = r#"(ImmutableArray<List<Int32>>)[[1]]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (ImmutableArray<List<Int32>>)[[1]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity30",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity30",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity30",
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

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity31 (case 97)
#[test]
fn cast_versus_index_ambiguity_31() {
    let src = r#"var x = (A<B>)[1];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity31",
                    97,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity31",
                    97,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity31",
            97,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity31_GlobalStatement (case 98)
#[test]
fn cast_versus_index_ambiguity_31_global_statement() {
    let src = r#"var x = (A<B>)[1];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity31_GlobalStatement",
                    98,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CastVersusIndexAmbiguity31_GlobalStatement",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CastVersusIndexAmbiguity31_GlobalStatement",
            98,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.SpreadOfQuery (case 99)
#[test]
fn spread_of_query() {
    let src = r#"[.. from x in y select x]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. from x in y select x]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "SpreadOfQuery",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "SpreadOfQuery",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "SpreadOfQuery",
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

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpression1 (case 100)
#[test]
fn invoked_collection_expression_1() {
    let src = r#"[A, B]()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A, B](); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpression1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpression1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpression1",
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

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpression2 (case 101)
#[test]
fn invoked_collection_expression_2() {
    let src = r#"++[A, B]()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ++[A, B](); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpression2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpression2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpression2",
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

/// Roslyn: CollectionExpressionParsingTests.TestTrailingComma1 (case 102)
#[test]
fn trailing_comma_1() {
    let src = r#"[A,]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestTrailingComma1",
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

/// Roslyn: CollectionExpressionParsingTests.TestTrailingComma2 (case 103)
#[test]
fn trailing_comma_2() {
    let src = r#"[A,B,]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A,B,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestTrailingComma2",
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

/// Roslyn: CollectionExpressionParsingTests.TestTrailingComma3 (case 104)
#[test]
fn trailing_comma_3() {
    let src = r#"[A,B,,]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A,B,,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestTrailingComma3",
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

/// Roslyn: CollectionExpressionParsingTests.TestTrailingComma4 (case 105)
#[test]
fn trailing_comma_4() {
    let src = r#"[A,B,,,]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A,B,,,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma4",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestTrailingComma4",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestTrailingComma4",
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

/// Roslyn: CollectionExpressionParsingTests.TestNegatedLiteral (case 106)
#[test]
fn negated_literal() {
    let src = r#"-[A]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { -[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNegatedLiteral",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNegatedLiteral",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNegatedLiteral",
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

/// Roslyn: CollectionExpressionParsingTests.TestNullCoalescing1 (case 107)
#[test]
fn null_coalescing_1() {
    let src = r#"[A] ?? [B]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] ?? [B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullCoalescing1",
                    107,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullCoalescing1",
                    107,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNullCoalescing1",
            107,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestNullCoalescing2 (case 108)
#[test]
fn null_coalescing_2() {
    let src = r#"[..x ?? y]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x ?? y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullCoalescing2",
                    108,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullCoalescing2",
                    108,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNullCoalescing2",
            108,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestNullSuppression (case 109)
#[test]
fn null_suppression() {
    let src = r#"[A]!"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A]!; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullSuppression",
                    109,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNullSuppression",
                    109,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNullSuppression",
            109,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestPreIncrement (case 110)
#[test]
fn pre_increment() {
    let src = r#"++[A]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ++[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPreIncrement",
                    110,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPreIncrement",
                    110,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestPreIncrement",
            110,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestPostIncrement (case 111)
#[test]
fn post_increment() {
    let src = r#"[A]++"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A]++; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPostIncrement",
                    111,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPostIncrement",
                    111,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestPostIncrement",
            111,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitParsedAsElementAccess (case 112)
#[test]
fn await_parsed_as_element_access() {
    let src = r#"await [A]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await [A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitParsedAsElementAccess",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitParsedAsElementAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestAwaitParsedAsElementAccess",
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

/// Roslyn: CollectionExpressionParsingTests.TestAwaitParsedAsElementAccessTopLevel (case 113)
#[test]
fn await_parsed_as_element_access_top_level() {
    let src = r#"await [A];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitParsedAsElementAccessTopLevel",
                    113,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitParsedAsElementAccessTopLevel",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestAwaitParsedAsElementAccessTopLevel",
            113,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitInAsyncContext (case 114)
#[test]
fn await_in_async_context() {
    let src = r#"
class C
{
    async void F()
    {
        await [A];
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitInAsyncContext",
                    114,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitInAsyncContext",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestAwaitInAsyncContext",
            114,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitInNonAsyncContext (case 115)
#[test]
fn await_in_non_async_context() {
    let src = r#"
class C
{
    void F()
    {
        await [A];
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitInNonAsyncContext",
                    115,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAwaitInNonAsyncContext",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestAwaitInNonAsyncContext",
            115,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestSimpleSpread (case 116)
#[test]
fn simple_spread() {
    let src = r#"[..e]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSimpleSpread",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSimpleSpread",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestSimpleSpread",
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

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange1 (case 117)
#[test]
fn spread_of_range_1() {
    let src = r#"[.. ..]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. ..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestSpreadOfRange1",
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

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange2 (case 118)
#[test]
fn spread_of_range_2() {
    let src = r#"[.. ..e]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. ..e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestSpreadOfRange2",
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

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange3 (case 119)
#[test]
fn spread_of_range_3() {
    let src = r#"[.. e..]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. e..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestSpreadOfRange3",
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

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange4 (case 120)
#[test]
fn spread_of_range_4() {
    let src = r#"[.. e1..e2]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. e1..e2]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange4",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestSpreadOfRange4",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestSpreadOfRange4",
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

/// Roslyn: CollectionExpressionParsingTests.TestThrowExpression (case 121)
#[test]
fn throw_expression() {
    let src = r#"[..throw e]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..throw e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestThrowExpression",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestThrowExpression",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestThrowExpression",
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

/// Roslyn: CollectionExpressionParsingTests.TestMemberAccess (case 122)
#[test]
fn member_access() {
    let src = r#"[..x.y]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x.y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestMemberAccess",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestMemberAccess",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestMemberAccess",
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

/// Roslyn: CollectionExpressionParsingTests.TestAssignment (case 123)
#[test]
fn assignment() {
    let src = r#"[..x = y]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x = y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAssignment",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestAssignment",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestAssignment",
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

/// Roslyn: CollectionExpressionParsingTests.TestLambda (case 124)
#[test]
fn lambda() {
    let src = r#"[..x => y]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x => y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestLambda",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestLambda",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestLambda",
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

/// Roslyn: CollectionExpressionParsingTests.TestConditional (case 125)
#[test]
fn conditional() {
    let src = r#"[..x ? y : z]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x ? y : z]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestConditional",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestConditional",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestConditional",
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

/// Roslyn: CollectionExpressionParsingTests.TestPartialRange (case 126)
#[test]
fn partial_range() {
    let src = r#"[..e..]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..e..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPartialRange",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestPartialRange",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestPartialRange",
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

/// Roslyn: CollectionExpressionParsingTests.TestNewArray1 (case 127)
#[test]
fn new_array_1() {
    let src = r#"new T?[1]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T?[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNewArray1",
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

/// Roslyn: CollectionExpressionParsingTests.TestNewArray2 (case 128)
#[test]
fn new_array_2() {
    let src = r#"new T?[1] { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T?[1] { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNewArray2",
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

/// Roslyn: CollectionExpressionParsingTests.TestNewArray3 (case 129)
#[test]
fn new_array_3() {
    let src = r#"new T[]?[1]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T[]?[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNewArray3",
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

/// Roslyn: CollectionExpressionParsingTests.TestNewArray4 (case 130)
#[test]
fn new_array_4() {
    let src = r#"new T[]?[1] { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T[]?[1] { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray4",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray4",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNewArray4",
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

/// Roslyn: CollectionExpressionParsingTests.TestNewArray5 (case 131)
#[test]
fn new_array_5() {
    let src = r#"new T[]?[1].Length"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T[]?[1].Length; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray5",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestNewArray5",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestNewArray5",
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

/// Roslyn: CollectionExpressionParsingTests.TestError1 (case 132)
#[test]
fn error_1() {
    let src = r#"[,]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError1",
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

/// Roslyn: CollectionExpressionParsingTests.TestError2 (case 133)
#[test]
fn error_2() {
    let src = r#"[,A]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [,A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError2",
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

/// Roslyn: CollectionExpressionParsingTests.TestError3 (case 134)
#[test]
fn error_3() {
    let src = r#"[,,]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [,,]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError3",
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

/// Roslyn: CollectionExpressionParsingTests.TestError4 (case 135)
#[test]
fn error_4() {
    let src = r#"[..]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError4",
                    135,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError4",
                    135,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError4",
            135,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestError5 (case 136)
#[test]
fn error_5() {
    let src = r#"[...e]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [...e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError5",
                    136,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError5",
                    136,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError5",
            136,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestError6 (case 137)
#[test]
fn error_6() {
    let src = r#"[....]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [....]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError6",
                    137,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestError6",
                    137,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestError6",
            137,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets1 (case 138)
#[test]
fn generic_name_with_brackets_1() {
    let src = r#"A < B?[] > D"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B?[] > D; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets1",
                    138,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets1",
                    138,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets1",
            138,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets2 (case 139)
#[test]
fn generic_name_with_brackets_2() {
    let src = r#"A < B?[] > D"#;
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets2",
                    139,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets2",
            139,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets3 (case 140)
#[test]
fn generic_name_with_brackets_3() {
    let src = r#"nameof(A < B?[] > D)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { nameof(A < B?[] > D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets3",
                    140,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets3",
                    140,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets3",
            140,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets4 (case 141)
#[test]
fn generic_name_with_brackets_4() {
    let src = r#"typeof(A < B?[] > D)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { typeof(A < B?[] > D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets4",
                    141,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets4",
                    141,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets4",
            141,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets5 (case 142)
#[test]
fn generic_name_with_brackets_5() {
    let src = r#"default(A < B?[] > D)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { default(A < B?[] > D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets5",
                    142,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets5",
                    142,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets5",
            142,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets6 (case 143)
#[test]
fn generic_name_with_brackets_6() {
    let src = r#"A < B?[] : D"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B?[] : D; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets6",
                    143,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "GenericNameWithBrackets6",
                    143,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "GenericNameWithBrackets6",
            143,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Interpolation1 (case 144)
#[test]
fn interpolation_1() {
    let src = r#" $"{[A:B]}" "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  $"{[A:B]}" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Interpolation1",
                    144,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Interpolation1",
                    144,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Interpolation1",
            144,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Interpolation2 (case 145)
#[test]
fn interpolation_2() {
    let src = r#" $"{[:]}" "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  $"{[:]}" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Interpolation2",
                    145,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Interpolation2",
                    145,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Interpolation2",
            145,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Addressof1 (case 146)
#[test]
fn addressof_1() {
    let src = r#"&[A]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof1",
                    146,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof1",
                    146,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Addressof1",
            146,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Addressof2 (case 147)
#[test]
fn addressof_2() {
    let src = r#"&[A, B]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof2",
                    147,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof2",
                    147,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Addressof2",
            147,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Addressof3 (case 148)
#[test]
fn addressof_3() {
    let src = r#"&[A, B][C]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A, B][C]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof3",
                    148,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof3",
                    148,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Addressof3",
            148,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Addressof4 (case 149)
#[test]
fn addressof_4() {
    let src = r#"&[A:B]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A:B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof4",
                    149,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Addressof4",
                    149,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Addressof4",
            149,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Deref1 (case 150)
#[test]
fn deref_1() {
    let src = r#"*[]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref1",
                    150,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref1",
                    150,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Deref1",
            150,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Deref2 (case 151)
#[test]
fn deref_2() {
    let src = r#"*[A]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref2",
                    151,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref2",
                    151,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Deref2",
            151,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Deref3 (case 152)
#[test]
fn deref_3() {
    let src = r#"*[A, B]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref3",
                    152,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref3",
                    152,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Deref3",
            152,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Deref4 (case 153)
#[test]
fn deref_4() {
    let src = r#"*[A, B][C]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A, B][C]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref4",
                    153,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref4",
                    153,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Deref4",
            153,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.Deref5 (case 154)
#[test]
fn deref_5() {
    let src = r#"*[A:B]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A:B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref5",
                    154,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "Deref5",
                    154,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "Deref5",
            154,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.New1 (case 155)
#[test]
fn new_1() {
    let src = r#"new [A]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new [A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New1",
                    155,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New1",
                    155,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "New1",
            155,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.New2 (case 156)
#[test]
fn new_2() {
    let src = r#"new [A, B]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new [A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New2",
                    156,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New2",
                    156,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "New2",
            156,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.New3 (case 157)
#[test]
fn new_3() {
    let src = r#"new [A, B][C]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new [A, B][C]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New3",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "New3",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "New3",
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

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda1 (case 158)
#[test]
fn literal_containing_lambda_1() {
    let src = r#"_ = [Main, () => { }]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [Main, () => { }]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda1",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda1",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda1",
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

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda2 (case 159)
#[test]
fn literal_containing_lambda_2() {
    let src = r#"_ = [() => { }, () => { }]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [() => { }, () => { }]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda2",
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda2",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda2",
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

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda3 (case 160)
#[test]
fn literal_containing_lambda_3() {
    let src = r#"_ = [() => { }, Main]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [() => { }, Main]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda3",
                    160,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda3",
                    160,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda3",
            160,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda4 (case 161)
#[test]
fn literal_containing_lambda_4() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main, () => { }]);
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda4",
                    161,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda4",
                    161,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda4",
            161,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda5 (case 162)
#[test]
fn literal_containing_lambda_5() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main, Main, () => { }]);
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda5",
                    162,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda5",
                    162,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda5",
            162,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda6 (case 163)
#[test]
fn literal_containing_lambda_6() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main(), () => { }]);
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda6",
                    163,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda6",
                    163,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda6",
            163,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda7 (case 164)
#[test]
fn literal_containing_lambda_7() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([X () => {});
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda7",
                    164,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda7",
                    164,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda7",
            164,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda8 (case 165)
#[test]
fn literal_containing_lambda_8() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([X, Y () => {});
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda8",
                    165,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda8",
                    165,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda8",
            165,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda9 (case 166)
#[test]
fn literal_containing_lambda_9() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([X Y () => {});
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda9",
                    166,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LiteralContainingLambda9",
                    166,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LiteralContainingLambda9",
            166,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess1 (case 167)
#[test]
fn member_access_1() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [1].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess1",
                    167,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess1",
                    167,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess1",
            167,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess1A (case 168)
#[test]
fn member_access_1_a() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [Main].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess1A",
                    168,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess1A",
                    168,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess1A",
            168,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess2 (case 169)
#[test]
fn member_access_2() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [1]?.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess2",
                    169,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess2",
                    169,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess2",
            169,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess2A (case 170)
#[test]
fn member_access_2_a() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [Main]?.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess2A",
                    170,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess2A",
                    170,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess2A",
            170,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess3 (case 171)
#[test]
fn member_access_3() {
    let src = r#"
            [1].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess3",
                    171,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess3",
                    171,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess3",
            171,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess3A (case 172)
#[test]
fn member_access_3_a() {
    let src = r#"
            [Main].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess3A",
                    172,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess3A",
                    172,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess3A",
            172,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess4 (case 173)
#[test]
fn member_access_4() {
    let src = r#"
            [1]?.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess4",
                    173,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess4",
                    173,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess4",
            173,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess4A (case 174)
#[test]
fn member_access_4_a() {
    let src = r#"
            [Main]?.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess4A",
                    174,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess4A",
                    174,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess4A",
            174,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess5 (case 175)
#[test]
fn member_access_5() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [1][0].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess5",
                    175,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess5",
                    175,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess5",
            175,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess5A (case 176)
#[test]
fn member_access_5_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [1][0].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess5A",
                    176,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess5A",
                    176,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess5A",
            176,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess6 (case 177)
#[test]
fn member_access_6() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [1][Main].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess6",
                    177,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess6",
                    177,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess6",
            177,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess6A (case 178)
#[test]
fn member_access_6_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [1][Main].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess6A",
                    178,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess6A",
                    178,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess6A",
            178,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess7 (case 179)
#[test]
fn member_access_7() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [Main][1].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess7",
                    179,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess7",
                    179,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess7",
            179,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess7A (case 180)
#[test]
fn member_access_7_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [Main][1].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess7A",
                    180,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess7A",
                    180,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess7A",
            180,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess8 (case 181)
#[test]
fn member_access_8() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [Main][Main].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess8",
                    181,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess8",
                    181,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess8",
            181,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess8A (case 182)
#[test]
fn member_access_8_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [Main][Main].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess8A",
                    182,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess8A",
                    182,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess8A",
            182,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess9 (case 183)
#[test]
fn member_access_9() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess9",
                    183,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess9",
                    183,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess9",
            183,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess9A (case 184)
#[test]
fn member_access_9_a() {
    let src = r#"
            [].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess9A",
                    184,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess9A",
                    184,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess9A",
            184,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess10 (case 185)
#[test]
fn member_access_10() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []?.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess10",
                    185,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess10",
                    185,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess10",
            185,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess10A (case 186)
#[test]
fn member_access_10_a() {
    let src = r#"
            []?.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess10A",
                    186,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess10A",
                    186,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess10A",
            186,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess11 (case 187)
#[test]
fn member_access_11() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [][0].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess11",
                    187,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess11",
                    187,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess11",
            187,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess11A (case 188)
#[test]
fn member_access_11_a() {
    let src = r#"
            [][0].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess11A",
                    188,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess11A",
                    188,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess11A",
            188,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess12 (case 189)
#[test]
fn member_access_12() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []!.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess12",
                    189,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess12",
                    189,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess12",
            189,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess12A (case 190)
#[test]
fn member_access_12_a() {
    let src = r#"
            []!.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess12A",
                    190,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess12A",
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess12A",
            190,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess13 (case 191)
#[test]
fn member_access_13() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A]!.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess13",
                    191,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess13",
                    191,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess13",
            191,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess13A (case 192)
#[test]
fn member_access_13_a() {
    let src = r#"
            [A]!.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess13A",
                    192,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess13A",
                    192,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess13A",
            192,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess14 (case 193)
#[test]
fn member_access_14() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A:B]!.GetHashCode();
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess14",
                    193,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess14",
                    193,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess14",
            193,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess14A (case 194)
#[test]
fn member_access_14_a() {
    let src = r#"
            [A:B]!.GetHashCode();
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess14A",
                    194,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess14A",
                    194,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess14A",
            194,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess15 (case 195)
#[test]
fn member_access_15() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A()]!.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess15",
                    195,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess15",
                    195,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess15",
            195,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess15A (case 196)
#[test]
fn member_access_15_a() {
    let src = r#"
            [A()]!.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess15A",
                    196,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess15A",
                    196,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess15A",
            196,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess16 (case 197)
#[test]
fn member_access_16() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A()][0]!.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess16",
                    197,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess16",
                    197,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess16",
            197,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess16A (case 198)
#[test]
fn member_access_16_a() {
    let src = r#"
            [A()][0]!.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess16A",
                    198,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess16A",
                    198,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess16A",
            198,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess17 (case 199)
#[test]
fn member_access_17() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [][0]!.GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess17",
                    199,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess17",
                    199,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess17",
            199,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess17A (case 200)
#[test]
fn member_access_17_a() {
    let src = r#"
            [][0]!.GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess17A",
                    200,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess17A",
                    200,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess17A",
            200,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess18 (case 201)
#[test]
fn member_access_18() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A:B][C:D].GetHashCode();
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess18",
                    201,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess18",
                    201,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess18",
            201,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess18A (case 202)
#[test]
fn member_access_18_a() {
    let src = r#"
            [A:B][C:D].GetHashCode();
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess18A",
                    202,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess18A",
                    202,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess18A",
            202,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess19 (case 203)
#[test]
fn member_access_19() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [..A][..B].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess19",
                    203,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess19",
                    203,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess19",
            203,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess19A (case 204)
#[test]
fn member_access_19_a() {
    let src = r#"
            [..A][..B].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess19A",
                    204,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess19A",
                    204,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess19A",
            204,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess20 (case 205)
#[test]
fn member_access_20() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [[A]].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess20",
                    205,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess20",
                    205,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess20",
            205,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess20A (case 206)
#[test]
fn member_access_20_a() {
    let src = r#"
            [[A]].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess20A",
                    206,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess20A",
                    206,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess20A",
            206,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess21 (case 207)
#[test]
fn member_access_21() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A([B])].GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess21",
                    207,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess21",
                    207,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess21",
            207,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess21A (case 208)
#[test]
fn member_access_21_a() {
    let src = r#"
            [A([B])].GetHashCode();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess21A",
                    208,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess21A",
                    208,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess21A",
            208,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess22 (case 209)
#[test]
fn member_access_22() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A([B])] GetHashCode();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess22",
                    209,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess22",
                    209,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess22",
            209,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess22A (case 210)
#[test]
fn member_access_22_a() {
    let src = r#"
            [A([B])] GetHashCode();
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess22A",
                    210,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess22A",
                    210,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess22A",
            210,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess23 (case 211)
#[test]
fn member_access_23() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []++;
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess23",
                    211,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess23",
                    211,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess23",
            211,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess23A (case 212)
#[test]
fn member_access_23_a() {
    let src = r#"
            []++;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess23A",
                    212,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess23A",
                    212,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess23A",
            212,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess24 (case 213)
#[test]
fn member_access_24() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []--;
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess24",
                    213,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess24",
                    213,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess24",
            213,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess24A (case 214)
#[test]
fn member_access_24_a() {
    let src = r#"
            []--;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess24A",
                    214,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess24A",
                    214,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess24A",
            214,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess25 (case 215)
#[test]
fn member_access_25() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []->Goo();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess25",
                    215,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess25",
                    215,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess25",
            215,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess25A (case 216)
#[test]
fn member_access_25_a() {
    let src = r#"
            []->Goo;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess25A",
                    216,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "MemberAccess25A",
                    216,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "MemberAccess25A",
            216,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelFunction1 (case 217)
#[test]
fn attribute_on_top_level_function_1() {
    let src = r#"
            [A([B])] void Goo() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction1",
                    217,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction1",
                    217,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelFunction1",
            217,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelFunction2 (case 218)
#[test]
fn attribute_on_top_level_function_2() {
    let src = r#"
            [A([B])] A Goo() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction2",
                    218,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction2",
                    218,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelFunction2",
            218,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelFunction3 (case 219)
#[test]
fn attribute_on_top_level_function_3() {
    let src = r#"
            [A([B])] (A, B) Goo() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction3",
                    219,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction3",
                    219,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelFunction3",
            219,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelFunction4 (case 220)
#[test]
fn attribute_on_top_level_function_4() {
    let src = r#"
            [A([B])] (A, B) Goo<A,B>() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction4",
                    220,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction4",
                    220,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelFunction4",
            220,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelFunction5 (case 221)
#[test]
fn attribute_on_top_level_function_5() {
    let src = r#"
            [A([B])] (C, D) Goo<[E]F,[G([H])]I>() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction5",
                    221,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "AttributeOnTopLevelFunction5",
                    221,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "AttributeOnTopLevelFunction5",
            221,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead1 (case 222)
#[test]
fn lambda_attribute_versus_collection_lookahead_1() {
    let src = r#"[A, B]() =>"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A, B]() =>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead1",
                    222,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead1",
                    222,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead1",
            222,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead2 (case 223)
#[test]
fn lambda_attribute_versus_collection_lookahead_2() {
    let src = r#"[A][B] (C, D)? e => f"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? e => f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead2",
                    223,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead2",
                    223,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead2",
            223,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead2A (case 224)
#[test]
fn lambda_attribute_versus_collection_lookahead_2_a() {
    let src = r#"[A][B](C, D) ? e : f"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? e : f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead2A",
                    224,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead2A",
                    224,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead2A",
            224,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead3 (case 225)
#[test]
fn lambda_attribute_versus_collection_lookahead_3() {
    let src = r#"[A][B] (C, D)? (e) => f"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? (e) => f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead3",
                    225,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead3",
                    225,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead3",
            225,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead3A (case 226)
#[test]
fn lambda_attribute_versus_collection_lookahead_3_a() {
    let src = r#"[A][B](C, D) ? (e) : f"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? (e) : f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead3A",
                    226,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead3A",
                    226,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead3A",
            226,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead4 (case 227)
#[test]
fn lambda_attribute_versus_collection_lookahead_4() {
    let src = r#"[A][B] (C, D)? (e, f) => g"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? (e, f) => g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead4",
                    227,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead4",
                    227,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead4",
            227,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead4A (case 228)
#[test]
fn lambda_attribute_versus_collection_lookahead_4_a() {
    let src = r#"[A][B](C, D) ? (e, f) : g"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? (e, f) : g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead4A",
                    228,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead4A",
                    228,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead4A",
            228,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead5 (case 229)
#[test]
fn lambda_attribute_versus_collection_lookahead_5() {
    let src = r#"[A][B] (C, D)? ([e] f) => g"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? ([e] f) => g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead5",
                    229,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead5",
                    229,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead5",
            229,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead5A (case 230)
#[test]
fn lambda_attribute_versus_collection_lookahead_5_a() {
    let src = r#"[A][B](C, D) ? ([e] f) : g"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? ([e] f) : g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead5A",
                    230,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead5A",
                    230,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead5A",
            230,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead6 (case 231)
#[test]
fn lambda_attribute_versus_collection_lookahead_6() {
    let src = r#"[A][B] (C, D)? ((e,f) g) => h"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? ((e,f) g) => h; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead6",
                    231,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead6",
                    231,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead6",
            231,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead6A (case 232)
#[test]
fn lambda_attribute_versus_collection_lookahead_6_a() {
    let src = r#"[A][B](C, D) ? ((e,f) g) : h"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? ((e,f) g) : h; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead6A",
                    232,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead6A",
                    232,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead6A",
            232,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead7 (case 233)
#[test]
fn lambda_attribute_versus_collection_lookahead_7() {
    let src = r#"[A][B] (C, D)? ((e,f)[] g) => h"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B] (C, D)? ((e,f)[] g) => h; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead7",
                    233,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead7",
                    233,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead7",
            233,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead7A (case 234)
#[test]
fn lambda_attribute_versus_collection_lookahead_7_a() {
    let src = r#"[A][B](C, D) ? ((e,f)[] g) : h"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? ((e,f)[] g) : h; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead7A",
                    234,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "LambdaAttributeVersusCollectionLookahead7A",
                    234,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "LambdaAttributeVersusCollectionLookahead7A",
            234,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity1 (case 235)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_1() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()]();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1",
                    235,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1",
                    235,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1",
            235,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A (case 236)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_1_a() {
    let src = r#"
            [() => {}][rand.Next()]();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A",
                    236,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A",
                    236,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A",
            236,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity2 (case 237)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_2() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A);
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2",
                    237,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2",
                    237,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2",
            237,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A (case 238)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_2_a() {
    let src = r#"
            [() => {}][rand.Next()](A);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A",
                    238,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A",
                    238,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A",
            238,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity3 (case 239)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_3() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)[0];
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3",
                    239,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3",
                    239,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3",
            239,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A (case 240)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_3_a() {
    let src = r#"
            [() => {}][rand.Next()](A)[0];
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A",
                    240,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A",
                    240,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A",
            240,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity4 (case 241)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_4() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)(B);
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4",
                    241,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4",
                    241,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4",
            241,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A (case 242)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_4_a() {
    let src = r#"
            [() => {}][rand.Next()](A)(B);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A",
                    242,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A",
                    242,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A",
            242,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity5 (case 243)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_5() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A).B();
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5",
                    243,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5",
                    243,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5",
            243,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A (case 244)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_5_a() {
    let src = r#"
            [() => {}][rand.Next()](A).B();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A",
                    244,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A",
                    244,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A",
            244,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity6 (case 245)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_6() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)++;
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6",
                    245,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6",
                    245,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6",
            245,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A (case 246)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_6_a() {
    let src = r#"
            [() => {}][rand.Next()](A)++;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A",
                    246,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A",
                    246,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A",
            246,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity7 (case 247)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_7() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)[0] = 1;
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7",
                    247,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7",
                    247,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7",
            247,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A (case 248)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_7_a() {
    let src = r#"
            [() => {}][rand.Next()](A)[0] = 1;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A",
                    248,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A",
                    248,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A",
            248,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity8 (case 249)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_8() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr] (A, B) LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8",
                    249,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8",
                    249,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8",
            249,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity8A (case 250)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_8_a() {
    let src = r#"
            [Attr] (A, B) LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8A",
                    250,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8A",
                    250,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity8A",
            250,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity9 (case 251)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_9() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A, B) LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9",
                    251,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9",
                    251,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9",
            251,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity9A (case 252)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_9_a() {
    let src = r#"
            [Attr1][Attr2] (A, B) LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9A",
                    252,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9A",
                    252,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity9A",
            252,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity10 (case 253)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_10() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A, B)? LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10",
                    253,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10",
                    253,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10",
            253,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity10A (case 254)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_10_a() {
    let src = r#"
            [Attr1][Attr2] (A, B)? LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10A",
                    254,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10A",
                    254,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity10A",
            254,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity11 (case 255)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_11() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A, B)[] LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11",
                    255,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11",
                    255,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11",
            255,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity11A (case 256)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_11_a() {
    let src = r#"
            [Attr1][Attr2] (A, B)[] LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11A",
                    256,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11A",
                    256,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity11A",
            256,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity12 (case 257)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_12() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A, B)[,] LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12",
                    257,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12",
                    257,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12",
            257,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity12A (case 258)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_12_a() {
    let src = r#"
            [Attr1][Attr2] (A, B)[,] LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12A",
                    258,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12A",
                    258,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity12A",
            258,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity13 (case 259)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_13() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A, B)* LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13",
                    259,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13",
                    259,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13",
            259,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity13A (case 260)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_13_a() {
    let src = r#"
            [Attr1][Attr2] (A, B)* LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13A",
                    260,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13A",
                    260,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity13A",
            260,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity14 (case 261)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_14() {
    let src = r#"
            class C
            {
                void M()
                {
                    [Attr1][Attr2] (A a, B b) LocalFunc() { }
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14",
                    261,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14",
                    261,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14",
            261,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity14A (case 262)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_14_a() {
    let src = r#"
            [Attr1][Attr2] (A a, B b) LocalFunc() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14A",
                    262,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14A",
                    262,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "InvokedCollectionExpressionVersusLocalFunctionAmbiguity14A",
            262,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.ByteArrayAmbiguityWithAttributes (case 263)
#[test]
fn byte_array_ambiguity_with_attributes() {
    let src = r#"class C { public ReadOnlySpan<byte> B => [0, 1, 2, 3, 4, 5, 6, 7]; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ByteArrayAmbiguityWithAttributes",
                    263,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "ByteArrayAmbiguityWithAttributes",
                    263,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "ByteArrayAmbiguityWithAttributes",
            263,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TreatKeywordAsAttributeTarget (case 264)
#[test]
fn treat_keyword_as_attribute_target() {
    let src = r#"class C { public ReadOnlySpan<byte> B => [true: A] () => { }; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TreatKeywordAsAttributeTarget",
                    264,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TreatKeywordAsAttributeTarget",
                    264,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TreatKeywordAsAttributeTarget",
            264,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TreatKeywordAsCollectionExprElement (case 265)
#[test]
fn treat_keyword_as_collection_expr_element() {
    let src = r#"class C { public bool[] B => [true]; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TreatKeywordAsCollectionExprElement",
                    265,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TreatKeywordAsCollectionExprElement",
                    265,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TreatKeywordAsCollectionExprElement",
            265,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestIncompleteString1 (case 266)
#[test]
fn incomplete_string_1() {
    let src = r#"
            public enum BundleType
            {
                [A("B", "C"), Description("Goo
                bar baz")]
                A,
                [A("B", "C"), Description("Goo
                bar baz")]
                B,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 15,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString1",
                    266,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString1",
                    266,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestIncompleteString1",
            266,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestIncompleteString2 (case 267)
#[test]
fn incomplete_string_2() {
    let src = r#"
            public enum BundleType
            {
                [A("B", "C"), Description("X", "Goo
                bar baz")]
                A,
                [A("B", "C"), Description("X", "Goo
                bar baz")]
                B,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 15,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString2",
                    267,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString2",
                    267,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestIncompleteString2",
            267,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestIncompleteString3 (case 268)
#[test]
fn incomplete_string_3() {
    let src = r#"
            public enum BundleType
            {
                [A("B", "C"), Description($"Goo
                bar baz")]
                A,
                [A("B", "C"), Description($"Goo
                bar baz")]
                B,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 15,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString3",
                    268,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString3",
                    268,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestIncompleteString3",
            268,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.TestIncompleteString4 (case 269)
#[test]
fn incomplete_string_4() {
    let src = r#"
            public enum BundleType
            {
                [A("B", "C"), Description("X", $"Goo
                bar baz")]
                A,
                [A("B", "C"), Description("X", $"Goo
                bar baz")]
                B,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 15,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString4",
                    269,
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
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "TestIncompleteString4",
                    269,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "TestIncompleteString4",
            269,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity1 (case 270)
#[test]
fn collection_expression_conditional_expression_ambiguity_1() {
    let src = r#"var v = x is Y ? [1, 2, 3] : [1];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity1",
                    270,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity1",
                    270,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity1",
            270,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity2 (case 271)
#[test]
fn collection_expression_conditional_expression_ambiguity_2() {
    let src = r#"var v = x is Y ? [] : [1];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity2",
                    271,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity2",
                    271,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity2",
            271,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity3 (case 272)
#[test]
fn collection_expression_conditional_expression_ambiguity_3() {
    let src = r#"var v = x is Y ? [];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity3",
                    272,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity3",
                    272,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity3",
            272,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity4 (case 273)
#[test]
fn collection_expression_conditional_expression_ambiguity_4() {
    let src = r#"var v = x is Y ? [,];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity4",
                    273,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity4",
                    273,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity4",
            273,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity5 (case 274)
#[test]
fn collection_expression_conditional_expression_ambiguity_5() {
    let src = r#"var v = x is Y ? [][];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity5",
                    274,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity5",
                    274,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity5",
            274,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity6 (case 275)
#[test]
fn collection_expression_conditional_expression_ambiguity_6() {
    let src = r#"var v = x is Y ? [] == Complex() : [1];"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity6",
                    275,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity6",
                    275,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity6",
            275,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity7 (case 276)
#[test]
fn collection_expression_conditional_expression_ambiguity_7() {
    let src = r#"var v = x is Y ? [Goo]() => B : [Goo]() => C;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity7",
                    276,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "collection_expression_parsing_tests",
                    "CollectionExpressionParsingTests",
                    "CollectionExpression_ConditionalExpressionAmbiguity7",
                    276,
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
            "collection_expression_parsing_tests",
            "CollectionExpressionParsingTests",
            "CollectionExpression_ConditionalExpressionAmbiguity7",
            276,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
