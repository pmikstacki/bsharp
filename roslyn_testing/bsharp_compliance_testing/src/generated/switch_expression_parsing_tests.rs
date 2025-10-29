// Auto-generated from Roslyn: SwitchExpressionParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws_spanned;
use bsharp_syntax::span::Span;
/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression1 (case 1)
#[test]
fn errant_case_in_switch_expression_1() {
    let src = r#"
            x switch
            {
                case 0 => 1,
                case 1 => 2,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                case 0 => 1,
                case 1 => 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression1",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression1_Semicolons (case 2)
#[test]
fn errant_case_in_switch_expression_1_semicolons() {
    let src = r#"
            x switch
            {
                case 0 => 1;
                case 1 => 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                case 0 => 1;
                case 1 => 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression1_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression1_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression1_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression2 (case 3)
#[test]
fn errant_case_in_switch_expression_2() {
    let src = r#"
            x switch
            {
                case 0: 1,
                case 1: 2,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                case 0: 1,
                case 1: 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression2",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression2_Semicolons (case 4)
#[test]
fn errant_case_in_switch_expression_2_semicolons() {
    let src = r#"
            x switch
            {
                case 0: 1;
                case 1: 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                case 0: 1;
                case 1: 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression2_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression2_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression2_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression3 (case 5)
#[test]
fn errant_case_in_switch_expression_3() {
    let src = r#"
            {
                var y = x switch
                {
                    case 0:
                        Goo();
                        return Bar;
                    case 1:
                    {
                        Baz();
                        throw new Quux();
                    }
                };
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression3",
                    5,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression3",
                    5,
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression3",
            5,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression5 (case 6)
#[test]
fn errant_case_in_switch_expression_5() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case,
                    };
            }
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression5",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression5",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression5",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression6 (case 7)
#[test]
fn errant_case_in_switch_expression_6() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case;
                    };
            }
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression6",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression6",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression6",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression7 (case 8)
#[test]
fn errant_case_in_switch_expression_7() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case =>
                    };
            }
            "#;
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression7",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression7",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression7",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression8 (case 9)
#[test]
fn errant_case_in_switch_expression_8() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when true
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression8",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression8",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression8",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression9 (case 10)
#[test]
fn errant_case_in_switch_expression_9() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when true =>
                    };
            }
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression9",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression9",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression9",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression10 (case 11)
#[test]
fn errant_case_in_switch_expression_10() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case true =>
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression10",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression10",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression10",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression11 (case 12)
#[test]
fn errant_case_in_switch_expression_11() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when
                    };
            }
            "#;
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression11",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression11",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression11",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression12 (case 13)
#[test]
fn errant_case_in_switch_expression_12() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when =>
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression12",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression12",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression12",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression13 (case 14)
#[test]
fn errant_case_in_switch_expression_13() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when case
                    };
            }
            "#;
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression13",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression13",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression13",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression14 (case 15)
#[test]
fn errant_case_in_switch_expression_14() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when case 0
                    };
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression14",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantCaseInSwitchExpression14",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantCaseInSwitchExpression14",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression1 (case 16)
#[test]
fn errant_when_in_switch_expression_1() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression1",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression2 (case 17)
#[test]
fn errant_when_in_switch_expression_2() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when,
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression2",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression3 (case 18)
#[test]
fn errant_when_in_switch_expression_3() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when;
                    };
            }
            "#;
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression3",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression3",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression3",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression4 (case 19)
#[test]
fn errant_when_in_switch_expression_4() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when =>
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression4",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression4",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression4",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression5 (case 20)
#[test]
fn errant_when_in_switch_expression_5() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when => true
                    };
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression5",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression5",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression5",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression6 (case 21)
#[test]
fn errant_when_in_switch_expression_6() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression6",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression6",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression6",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression7 (case 22)
#[test]
fn errant_when_in_switch_expression_7() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true,
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression7",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression7",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression7",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression8 (case 23)
#[test]
fn errant_when_in_switch_expression_8() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true;
                    };
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression8",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression8",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression8",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression9 (case 24)
#[test]
fn errant_when_in_switch_expression_9() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true =>
                    };
            }
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression9",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantWhenInSwitchExpression9",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantWhenInSwitchExpression9",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantColonsInSwitchExpression1 (case 25)
#[test]
fn errant_colons_in_switch_expression_1() {
    let src = r#"
            x switch
            {
                0: 1,
                1: 2,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0: 1,
                1: 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantColonsInSwitchExpression1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantColonsInSwitchExpression1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantColonsInSwitchExpression1",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantColonsInSwitchExpression1_Semicolons (case 26)
#[test]
fn errant_colons_in_switch_expression_1_semicolons() {
    let src = r#"
            x switch
            {
                0: 1;
                1: 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0: 1;
                1: 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantColonsInSwitchExpression1_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantColonsInSwitchExpression1_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantColonsInSwitchExpression1_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantDefaultInSwitchExpression1 (case 27)
#[test]
fn errant_default_in_switch_expression_1() {
    let src = r#"
            x switch
            {
                0 => 1,
                default: 2,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1,
                default: 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantDefaultInSwitchExpression1",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantDefaultInSwitchExpression1_Semicolons (case 28)
#[test]
fn errant_default_in_switch_expression_1_semicolons() {
    let src = r#"
            x switch
            {
                0 => 1;
                default: 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1;
                default: 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression1_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression1_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantDefaultInSwitchExpression1_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantDefaultInSwitchExpression2 (case 29)
#[test]
fn errant_default_in_switch_expression_2() {
    let src = r#"
            x switch
            {
                0 => 1,
                default(int): 2,
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1,
                default(int): 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantDefaultInSwitchExpression2",
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

/// Roslyn: SwitchExpressionParsingTests.TestErrantDefaultInSwitchExpression2_Semicolons (case 30)
#[test]
fn errant_default_in_switch_expression_2_semicolons() {
    let src = r#"
            x switch
            {
                0 => 1;
                default(int): 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1;
                default(int): 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression2_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestErrantDefaultInSwitchExpression2_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestErrantDefaultInSwitchExpression2_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestNormalDefaultInSwitchExpression2 (case 31)
#[test]
fn normal_default_in_switch_expression_2() {
    let src = r#"
            x switch
            {
                0 => 1,
                default(int) => 2,
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1,
                default(int) => 2,
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNormalDefaultInSwitchExpression2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNormalDefaultInSwitchExpression2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNormalDefaultInSwitchExpression2",
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

/// Roslyn: SwitchExpressionParsingTests.TestNormalDefaultInSwitchExpression2_Semicolons (case 32)
#[test]
fn normal_default_in_switch_expression_2_semicolons() {
    let src = r#"
            x switch
            {
                0 => 1;
                default(int) => 2;
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            x switch
            {
                0 => 1;
                default(int) => 2;
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNormalDefaultInSwitchExpression2_Semicolons",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNormalDefaultInSwitchExpression2_Semicolons",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNormalDefaultInSwitchExpression2_Semicolons",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern1 (case 33)
#[test]
fn unclosed_recursive_pattern_1() {
    let src = r#"
            obj switch
            {
                Type { Prop: Type { } => 1,
                Type { Prop: Type { } => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: Type { } => 1,
                Type { Prop: Type { } => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern1",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern1_Colon (case 34)
#[test]
fn unclosed_recursive_pattern_1_colon() {
    let src = r#"
            obj switch
            {
                Type { Prop: Type { } : 1,
                Type { Prop: Type { } : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: Type { } : 1,
                Type { Prop: Type { } : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern1_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern1_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern1_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern2 (case 35)
#[test]
fn unclosed_recursive_pattern_2() {
    let src = r#"
            obj switch
            {
                Type { Prop: Type { => 1,
                Type { Prop: Type { => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: Type { => 1,
                Type { Prop: Type { => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern2",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern2_Colon (case 36)
#[test]
fn unclosed_recursive_pattern_2_colon() {
    let src = r#"
            obj switch
            {
                Type { Prop: Type { : 1,
                Type { Prop: Type { : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: Type { : 1,
                Type { Prop: Type { : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern2_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern2_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern2_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern3 (case 37)
#[test]
fn unclosed_recursive_pattern_3() {
    let src = r#"
            obj switch
            {
                Type { Prop: { Prop: { => 1,
                Type { Prop: { Prop: { => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: { Prop: { => 1,
                Type { Prop: { Prop: { => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern3",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern3",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern3",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedRecursivePattern3_Colon (case 38)
#[test]
fn unclosed_recursive_pattern_3_colon() {
    let src = r#"
            obj switch
            {
                Type { Prop: { Prop: { : 1,
                Type { Prop: { Prop: { : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 8,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type { Prop: { Prop: { : 1,
                Type { Prop: { Prop: { : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern3_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedRecursivePattern3_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedRecursivePattern3_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern1 (case 39)
#[test]
fn unclosed_list_pattern_1() {
    let src = r#"
            obj switch
            {
                [ => 1,
                [ => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [ => 1,
                [ => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern1",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern1_Colon (case 40)
#[test]
fn unclosed_list_pattern_1_colon() {
    let src = r#"
            obj switch
            {
                [ : 1,
                [ : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [ : 1,
                [ : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern1_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern1_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern1_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern2 (case 41)
#[test]
fn unclosed_list_pattern_2() {
    let src = r#"
            obj switch
            {
                [[ => 1,
                [[ => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [[ => 1,
                [[ => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern2",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern2_Colon (case 42)
#[test]
fn unclosed_list_pattern_2_colon() {
    let src = r#"
            obj switch
            {
                [[ : 1,
                [[ : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [[ : 1,
                [[ : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern2_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern2_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern2_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern3 (case 43)
#[test]
fn unclosed_list_pattern_3() {
    let src = r#"
            obj switch
            {
                [[[ => 1,
                [[[ => 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [[[ => 1,
                [[[ => 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern3",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern3",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern3",
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

/// Roslyn: SwitchExpressionParsingTests.TestUnclosedListPattern3_Colon (case 44)
#[test]
fn unclosed_list_pattern_3_colon() {
    let src = r#"
            obj switch
            {
                [[[ : 1,
                [[[ : 2
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 8,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                [[[ : 1,
                [[[ : 2
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern3_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestUnclosedListPattern3_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestUnclosedListPattern3_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestIncompleteSwitchExpression (case 45)
#[test]
fn incomplete_switch_expression() {
    let src = r#"
            obj switch
            {
                { Prop: 1, { Prop: 2 }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                { Prop: 1, { Prop: 2 }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestIncompleteSwitchExpression",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestIncompleteSwitchExpression",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestIncompleteSwitchExpression",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern1 (case 46)
#[test]
fn nullable_type_in_pattern_1() {
    let src = r#"
            obj switch
            {
                Type? => 1
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern1",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern1",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern1",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern1_Colon (case 47)
#[test]
fn nullable_type_in_pattern_1_colon() {
    let src = r#"
            obj switch
            {
                Type? : 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? : 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern1_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern1_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern1_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern2 (case 48)
#[test]
fn nullable_type_in_pattern_2() {
    let src = r#"
            obj switch
            {
                Type? varName => 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern2",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern2",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern2",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern2_Colon (case 49)
#[test]
fn nullable_type_in_pattern_2_colon() {
    let src = r#"
            obj switch
            {
                Type? varName : 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName : 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern2_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern2_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern2_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern3 (case 50)
#[test]
fn nullable_type_in_pattern_3() {
    let src = r#"
            obj switch
            {
                Type? when x > 0 => 1
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? when x > 0 => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern3",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern3",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern3",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern3_Colon (case 51)
#[test]
fn nullable_type_in_pattern_3_colon() {
    let src = r#"
            obj switch
            {
                Type? when x > 0 : 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? when x > 0 : 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern3_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern3_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern3_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern4 (case 52)
#[test]
fn nullable_type_in_pattern_4() {
    let src = r#"
            obj switch
            {
                Type? varName when x > 0 => 1
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName when x > 0 => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern4",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern4",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern4",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern4_Colon (case 53)
#[test]
fn nullable_type_in_pattern_4_colon() {
    let src = r#"
            obj switch
            {
                Type? varName when x > 0 : 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName when x > 0 : 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern4_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern4_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern4_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern5 (case 54)
#[test]
fn nullable_type_in_pattern_5() {
    let src = r#"
            obj switch
            {
                (Type? when) when x > 0 => 1
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                (Type? when) when x > 0 => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern5",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern5",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern5",
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

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern5_Colon (case 55)
#[test]
fn nullable_type_in_pattern_5_colon() {
    let src = r#"
            obj switch
            {
                (Type? when) when x > 0 : 1
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                (Type? when) when x > 0 : 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern5_Colon",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestNullableTypeInPattern5_Colon",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestNullableTypeInPattern5_Colon",
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

/// Roslyn: SwitchExpressionParsingTests.TestConditionalExpressionAsPattern (case 56)
#[test]
fn conditional_expression_as_pattern() {
    let src = r#"
            obj switch
            {
                (flag ? a : b) => 1
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                (flag ? a : b) => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestConditionalExpressionAsPattern",
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
                    "switch_expression_parsing_tests",
                    "SwitchExpressionParsingTests",
                    "TestConditionalExpressionAsPattern",
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
            "switch_expression_parsing_tests",
            "SwitchExpressionParsingTests",
            "TestConditionalExpressionAsPattern",
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
