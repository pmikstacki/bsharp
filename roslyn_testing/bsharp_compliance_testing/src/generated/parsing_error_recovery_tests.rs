// Auto-generated from Roslyn: ParsingErrorRecoveryTests
/// Roslyn: ParsingErrorRecoveryTests.RazorCommentRecovery_Space (case 1)
#[test]
fn razor_comment_recovery_space() {
    let src = r#"@ * *@"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "RazorCommentRecovery_Space",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "RazorCommentRecovery_Space",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "RazorCommentRecovery_Space",
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

/// Roslyn: ParsingErrorRecoveryTests.RazorCommentRecovery_NoStart (case 2)
#[test]
fn razor_comment_recovery_no_start() {
    let src = r#"*@"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "RazorCommentRecovery_NoStart",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "RazorCommentRecovery_NoStart",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "RazorCommentRecovery_NoStart",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_01 (case 3)
#[test]
fn preprocessor_directive_trailing_01() {
    let src = r#"
                if (#if)
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_01",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_01_WhitespaceBeforeHash (case 4)
#[test]
fn preprocessor_directive_trailing_01_whitespace_before_hash() {
    let src = r#"
                if ( #if)
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01_WhitespaceBeforeHash",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01_WhitespaceBeforeHash",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_01_WhitespaceBeforeHash",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_01_WhitespaceAfterHash (case 5)
#[test]
fn preprocessor_directive_trailing_01_whitespace_after_hash() {
    let src = r#"
                if ( # if)
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01_WhitespaceAfterHash",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_01_WhitespaceAfterHash",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_01_WhitespaceAfterHash",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_02 (case 6)
#[test]
fn preprocessor_directive_trailing_02() {
    let src = r#"
                if (#if false
                x
                #else
                y
                #endif
                "#;
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_02",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_02",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_02",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_03 (case 7)
#[test]
fn preprocessor_directive_trailing_03() {
    let src = r#"
                a();
                #if false 
                b();
                /* comment */ #else
                c();
                #endif
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_03",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_03",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_03",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_04 (case 8)
#[test]
fn preprocessor_directive_trailing_04() {
    let src = r#"
                a();
                #if true 
                b();
                /* comment */ #elif false
                c();
                #endif
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_04",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_04",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_04",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_05 (case 9)
#[test]
fn preprocessor_directive_trailing_05() {
    let src = r#"
                a();
                #if true 
                b();
                /* comment */ #endif
                #else
                c();
                #endif
                d();
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_05",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_05",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_05",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_Define (case 10)
#[test]
fn preprocessor_directive_trailing_define() {
    let src = r#"
                /* comment */ #define ABC
                #if ABC
                x();
                #else
                y();
                #endif
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Define",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Define",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_Define",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_Undefine (case 11)
#[test]
fn preprocessor_directive_trailing_undefine() {
    let src = r#"
                #define ABC
                /* comment */ #undefine ABC
                #if ABC
                x();
                #else
                y();
                #endif
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Undefine",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Undefine",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_Undefine",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_ErrorWarning (case 12)
#[test]
fn preprocessor_directive_trailing_error_warning() {
    let src = r#"
                /* comment */ #error E1
                /* comment */ #warning W1
                #error E2
                #warning W2
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_ErrorWarning",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_ErrorWarning",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_ErrorWarning",
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

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_Line (case 13)
#[test]
fn preprocessor_directive_trailing_line() {
    let src = r#"
                #line 200
                /* comment */ #line 100
                #error E1
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Line",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "PreprocessorDirective_Trailing_Line",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "PreprocessorDirective_Trailing_Line",
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

/// Roslyn: ParsingErrorRecoveryTests.MissingNodeWithSkippedTokens1 (case 14)
#[test]
fn missing_node_with_skipped_tokens_1() {
    let src = r#"
                i,(#

                interface
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "MissingNodeWithSkippedTokens1",
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
                    "parsing_error_recovery_tests",
                    "ParsingErrorRecoveryTests",
                    "MissingNodeWithSkippedTokens1",
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
            "parsing_error_recovery_tests",
            "ParsingErrorRecoveryTests",
            "MissingNodeWithSkippedTokens1",
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
