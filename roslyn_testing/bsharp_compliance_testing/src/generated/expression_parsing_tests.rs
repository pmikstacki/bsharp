// Auto-generated from Roslyn: ExpressionParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: ExpressionParsingTests.TestInterpolatedVerbatimString (case 1)
#[test]
fn interpolated_verbatim_string() {
    let src = r#"$@""hello"""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $@""hello""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedVerbatimString",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedVerbatimString",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TestInterpolatedVerbatimString",
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

/// Roslyn: ExpressionParsingTests.TestInterpolatedSingleLineRawString1 (case 2)
#[test]
fn interpolated_single_line_raw_string_1() {
    let src = r#"$""""""{1 + 1}"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""{1 + 1}""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedSingleLineRawString1",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedSingleLineRawString1",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TestInterpolatedSingleLineRawString1",
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

/// Roslyn: ExpressionParsingTests.TestInterpolatedSingleLineRawString2 (case 3)
#[test]
fn interpolated_single_line_raw_string_2() {
    let src = r#"$$""""""{{{1 + 1}}}"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $$""""""{{{1 + 1}}}""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedSingleLineRawString2",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedSingleLineRawString2",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TestInterpolatedSingleLineRawString2",
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

/// Roslyn: ExpressionParsingTests.TestInterpolatedMultiLineRawString1 (case 4)
#[test]
fn interpolated_multi_line_raw_string_1() {
    let src = r#"$""""""
    {1 + 1}
    """""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""
    {1 + 1}
    """"""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedMultiLineRawString1",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedMultiLineRawString1",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TestInterpolatedMultiLineRawString1",
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

/// Roslyn: ExpressionParsingTests.TestInterpolatedMultiLineRawString2 (case 5)
#[test]
fn interpolated_multi_line_raw_string_2() {
    let src = r#"$$""""""
    {{{1 + 1}}}
    """""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $$""""""
    {{{1 + 1}}}
    """"""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedMultiLineRawString2",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TestInterpolatedMultiLineRawString2",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TestInterpolatedMultiLineRawString2",
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

/// Roslyn: ExpressionParsingTests.TopLevel_NewPartialArray_Incomplete (case 6)
#[test]
fn top_level_new_partial_array_incomplete() {
    let src = r#"new partial["#;
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TopLevel_NewPartialArray_Incomplete",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TopLevel_NewPartialArray_Incomplete",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TopLevel_NewPartialArray_Incomplete",
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

/// Roslyn: ExpressionParsingTests.ShiftOperator (case 7)
#[test]
fn shift_operator() {
    let src = r#"
class C
{
    int x = 1 << 2 << 3;
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ShiftOperator",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ShiftOperator",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ShiftOperator",
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

/// Roslyn: ExpressionParsingTests.TypeArgumentIndexerInitializer (case 8)
#[test]
fn type_argument_indexer_initializer() {
    let src = r#"new C { [0] = op1 < op2, [1] = true }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new C { [0] = op1 < op2, [1] = true }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TypeArgumentIndexerInitializer",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "TypeArgumentIndexerInitializer",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "TypeArgumentIndexerInitializer",
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

/// Roslyn: ExpressionParsingTests.InterpolatedStringExpressionSurroundedByCurlyBraces (case 9)
#[test]
fn interpolated_string_expression_surrounded_by_curly_braces() {
    let src = r#"$"{{{12}}}""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $"{{{12}}}"; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "InterpolatedStringExpressionSurroundedByCurlyBraces",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "InterpolatedStringExpressionSurroundedByCurlyBraces",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "InterpolatedStringExpressionSurroundedByCurlyBraces",
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

/// Roslyn: ExpressionParsingTests.InterpolatedStringExpressionWithFormatClauseSurroundedByCurlyBraces (case 10)
#[test]
fn interpolated_string_expression_with_format_clause_surrounded_by_curly_braces() {
    let src = r#"$"{{{12:X}}}""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $"{{{12:X}}}"; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "InterpolatedStringExpressionWithFormatClauseSurroundedByCurlyBraces",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "InterpolatedStringExpressionWithFormatClauseSurroundedByCurlyBraces",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "InterpolatedStringExpressionWithFormatClauseSurroundedByCurlyBraces",
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

/// Roslyn: ExpressionParsingTests.ConditionalExpressionInInterpolation (case 11)
#[test]
fn conditional_expression_in_interpolation() {
    let src = r#"$"{a ? b : d}""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $"{a ? b : d}"; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ConditionalExpressionInInterpolation",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ConditionalExpressionInInterpolation",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ConditionalExpressionInInterpolation",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpression (case 12)
#[test]
fn null_coalescing_assignment_expression() {
    let src = r#"a ??= b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpression",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpression",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentExpression",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionParenthesized (case 13)
#[test]
fn null_coalescing_assignment_expression_parenthesized() {
    let src = r#"(a) ??= b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a) ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionParenthesized",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionParenthesized",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentExpressionParenthesized",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionInvocation (case 14)
#[test]
fn null_coalescing_assignment_expression_invocation() {
    let src = r#"M(a) ??= b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(a) ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionInvocation",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionInvocation",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentExpressionInvocation",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionAndCoalescingOperator (case 15)
#[test]
fn null_coalescing_assignment_expression_and_coalescing_operator() {
    let src = r#"a ?? b ??= c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ?? b ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionAndCoalescingOperator",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionAndCoalescingOperator",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentExpressionAndCoalescingOperator",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionNested (case 16)
#[test]
fn null_coalescing_assignment_expression_nested() {
    let src = r#"a ??= b ??= c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ??= b ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionNested",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentExpressionNested",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentExpressionNested",
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

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentParenthesizedNested (case 17)
#[test]
fn null_coalescing_assignment_parenthesized_nested() {
    let src = r#"(a ??= b) ??= c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a ??= b) ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentParenthesizedNested",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "NullCoalescingAssignmentParenthesizedNested",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "NullCoalescingAssignmentParenthesizedNested",
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

/// Roslyn: ExpressionParsingTests.IndexExpression (case 18)
#[test]
fn index_expression() {
    let src = r#"^1"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "IndexExpression",
                    18,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "IndexExpression",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "IndexExpression",
            18,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: ExpressionParsingTests.RangeExpression_ThreeDots (case 19)
#[test]
fn range_expression_three_dots() {
    let src = r#"1...2"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1...2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ThreeDots",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ThreeDots",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_ThreeDots",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary (case 20)
#[test]
fn range_expression_binary() {
    let src = r#"1..1"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary",
                    20,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Binary",
            20,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithIndexes (case 21)
#[test]
fn range_expression_binary_with_indexes() {
    let src = r#"^5..^3"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^5..^3; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithIndexes",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithIndexes",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Binary_WithIndexes",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithALowerPrecedenceOperator_01 (case 22)
#[test]
fn range_expression_binary_with_alower_precedence_operator_01() {
    let src = r#"1<<2..3>>4"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1<<2..3>>4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithALowerPrecedenceOperator_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithALowerPrecedenceOperator_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Binary_WithALowerPrecedenceOperator_01",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithALowerPrecedenceOperator_02 (case 23)
#[test]
fn range_expression_binary_with_alower_precedence_operator_02() {
    let src = r#"1<<2..3>>>4"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1<<2..3>>>4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithALowerPrecedenceOperator_02",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithALowerPrecedenceOperator_02",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Binary_WithALowerPrecedenceOperator_02",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithAHigherPrecedenceOperator (case 24)
#[test]
fn range_expression_binary_with_ahigher_precedence_operator() {
    let src = r#"1+2..3-4"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1+2..3-4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithAHigherPrecedenceOperator",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Binary_WithAHigherPrecedenceOperator",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Binary_WithAHigherPrecedenceOperator",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryBadLeft (case 25)
#[test]
fn range_expression_unary_bad_left() {
    let src = r#"a*..b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a*..b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryBadLeft",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryBadLeft",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_UnaryBadLeft",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_BinaryLeftPlus (case 26)
#[test]
fn range_expression_binary_left_plus() {
    let src = r#"a + b..c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_BinaryLeftPlus",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_BinaryLeftPlus",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_BinaryLeftPlus",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryLeftPlus (case 27)
#[test]
fn range_expression_unary_left_plus() {
    let src = r#"a + b.."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryLeftPlus",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryLeftPlus",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_UnaryLeftPlus",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryRightMult (case 28)
#[test]
fn range_expression_unary_right_mult() {
    let src = r#"a.. && b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a.. && b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryRightMult",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryRightMult",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_UnaryRightMult",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryRightMult2 (case 29)
#[test]
fn range_expression_unary_right_mult_2() {
    let src = r#"..a && b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..a && b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryRightMult2",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_UnaryRightMult2",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_UnaryRightMult2",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity1 (case 30)
#[test]
fn range_expression_ambiguity_1() {
    let src = r#".. .."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { .. ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity1",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity1",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity1",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity2 (case 31)
#[test]
fn range_expression_ambiguity_2() {
    let src = r#".. .. e"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { .. .. e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity2",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity2",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity2",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity3 (case 32)
#[test]
fn range_expression_ambiguity_3() {
    let src = r#".. e .."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { .. e ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity3",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity3",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity3",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity4 (case 33)
#[test]
fn range_expression_ambiguity_4() {
    let src = r#"a .. .. b"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a .. .. b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity4",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity4",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity4",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity5 (case 34)
#[test]
fn range_expression_ambiguity_5() {
    let src = r#"a .. b .."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a .. b ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity5",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity5",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity5",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Ambiguity6 (case 35)
#[test]
fn range_expression_ambiguity_6() {
    let src = r#"a .. b .. c"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a .. b .. c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity6",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Ambiguity6",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Ambiguity6",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_NotCast (case 36)
#[test]
fn range_expression_not_cast() {
    let src = r#"(Offset)..(Offset + Count)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Offset)..(Offset + Count); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NotCast",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NotCast",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_NotCast",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Right (case 37)
#[test]
fn range_expression_right() {
    let src = r#"..1"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Right",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Right",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Right",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Right_WithIndexes (case 38)
#[test]
fn range_expression_right_with_indexes() {
    let src = r#"..^3"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..^3; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Right_WithIndexes",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Right_WithIndexes",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Right_WithIndexes",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Left (case 39)
#[test]
fn range_expression_left() {
    let src = r#"1.."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Left",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Left",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Left",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_Left_WithIndexes (case 40)
#[test]
fn range_expression_left_with_indexes() {
    let src = r#"^5.."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^5..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Left_WithIndexes",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_Left_WithIndexes",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_Left_WithIndexes",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_NoOperands (case 41)
#[test]
fn range_expression_no_operands() {
    let src = r#".."#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NoOperands",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NoOperands",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_NoOperands",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_NoOperands_WithOtherOperators (case 42)
#[test]
fn range_expression_no_operands_with_other_operators() {
    let src = r#"1+..<<2"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1+..<<2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NoOperands_WithOtherOperators",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_NoOperands_WithOtherOperators",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_NoOperands_WithOtherOperators",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_DotSpaceDot (case 43)
#[test]
fn range_expression_dot_space_dot() {
    let src = r#"1. .2"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1. .2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_DotSpaceDot",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_DotSpaceDot",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_DotSpaceDot",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_NoOperands (case 44)
#[test]
fn range_expression_method_invocation_no_operands() {
    let src = r#".. .ToString()"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { .. .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_NoOperands",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_NoOperands",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_MethodInvocation_NoOperands",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_LeftOperand (case 45)
#[test]
fn range_expression_method_invocation_left_operand() {
    let src = r#"1.. .ToString()"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1.. .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_LeftOperand",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_LeftOperand",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_MethodInvocation_LeftOperand",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_RightOperand (case 46)
#[test]
fn range_expression_method_invocation_right_operand() {
    let src = r#"..2 .ToString()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..2 .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_RightOperand",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_RightOperand",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_MethodInvocation_RightOperand",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_TwoOperands (case 47)
#[test]
fn range_expression_method_invocation_two_operands() {
    let src = r#"1..2 .ToString()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..2 .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_TwoOperands",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_MethodInvocation_TwoOperands",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_MethodInvocation_TwoOperands",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_ConditionalAccessExpression_01 (case 48)
#[test]
fn range_expression_conditional_access_expression_01() {
    let src = r#"c?..b"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?..b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ConditionalAccessExpression_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ConditionalAccessExpression_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_ConditionalAccessExpression_01",
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

/// Roslyn: ExpressionParsingTests.RangeExpression_ConditionalAccessExpression_02 (case 49)
#[test]
fn range_expression_conditional_access_expression_02() {
    let src = r#"c?.b..a"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.b..a; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ConditionalAccessExpression_02",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "RangeExpression_ConditionalAccessExpression_02",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "RangeExpression_ConditionalAccessExpression_02",
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

/// Roslyn: ExpressionParsingTests.BaseExpression_01 (case 50)
#[test]
fn base_expression_01() {
    let src = r#"base"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { base; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "BaseExpression_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "BaseExpression_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "BaseExpression_01",
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

/// Roslyn: ExpressionParsingTests.ArrayCreation_BadRef (case 51)
#[test]
fn array_creation_bad_ref() {
    let src = r#"new[] { ref }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { ref }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRef",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRef",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_BadRef",
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

/// Roslyn: ExpressionParsingTests.ArrayCreation_BadRefExpression (case 52)
#[test]
fn array_creation_bad_ref_expression() {
    let src = r#"new[] { ref obj }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { ref obj }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRefExpression",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRefExpression",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_BadRefExpression",
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

/// Roslyn: ExpressionParsingTests.ArrayCreation_BadRefElementAccess (case 53)
#[test]
fn array_creation_bad_ref_element_access() {
    let src = r#"new[] { ref[] }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { ref[] }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRefElementAccess",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadRefElementAccess",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_BadRefElementAccess",
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

/// Roslyn: ExpressionParsingTests.AnonymousObjectCreation_BadRef (case 54)
#[test]
fn anonymous_object_creation_bad_ref() {
    let src = r#"new { ref }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new { ref }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "AnonymousObjectCreation_BadRef",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "AnonymousObjectCreation_BadRef",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "AnonymousObjectCreation_BadRef",
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

/// Roslyn: ExpressionParsingTests.ObjectInitializer_BadRef (case 55)
#[test]
fn object_initializer_bad_ref() {
    let src = r#"new C { P = ref }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new C { P = ref }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializer_BadRef",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializer_BadRef",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ObjectInitializer_BadRef",
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

/// Roslyn: ExpressionParsingTests.CollectionInitializer_BadRef_01 (case 56)
#[test]
fn collection_initializer_bad_ref_01() {
    let src = r#"new C { ref }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new C { ref }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "CollectionInitializer_BadRef_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "CollectionInitializer_BadRef_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "CollectionInitializer_BadRef_01",
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

/// Roslyn: ExpressionParsingTests.CollectionInitializer_BadRef_02 (case 57)
#[test]
fn collection_initializer_bad_ref_02() {
    let src = r#"new C { { 0, ref } }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new C { { 0, ref } }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "CollectionInitializer_BadRef_02",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "CollectionInitializer_BadRef_02",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "CollectionInitializer_BadRef_02",
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

/// Roslyn: ExpressionParsingTests.AttributeArgument_BadRef (case 58)
#[test]
fn attribute_argument_bad_ref() {
    let src = r#"class C { [Attr(ref)] void M() { } }"#;
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "AttributeArgument_BadRef",
                    58,
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "AttributeArgument_BadRef",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "AttributeArgument_BadRef",
            58,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: ExpressionParsingTests.ForLoop_BadRefCondition (case 59)
#[test]
fn for_loop_bad_ref_condition() {
    let src = r#"for (int i = 0; ref; i++) { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ForLoop_BadRefCondition",
                    59,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ForLoop_BadRefCondition",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ForLoop_BadRefCondition",
            59,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ExpressionParsingTests.ArrayCreation_BadInElementAccess (case 60)
#[test]
fn array_creation_bad_in_element_access() {
    let src = r#"new[] { in[] }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { in[] }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadInElementAccess",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadInElementAccess",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_BadInElementAccess",
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

/// Roslyn: ExpressionParsingTests.ArrayCreation_BadOutElementAccess (case 61)
#[test]
fn array_creation_bad_out_element_access() {
    let src = r#"new[] { out[] }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { out[] }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadOutElementAccess",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_BadOutElementAccess",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_BadOutElementAccess",
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

/// Roslyn: ExpressionParsingTests.ArrayCreation_ElementAccess (case 62)
#[test]
fn array_creation_element_access() {
    let src = r#"new[] { obj[] }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[] { obj[] }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_ElementAccess",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ArrayCreation_ElementAccess",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ArrayCreation_ElementAccess",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShift_01 (case 63)
#[test]
fn unsigned_right_shift_01() {
    let src = r#"x >>> y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >>> y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShift_01",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShift_02 (case 64)
#[test]
fn unsigned_right_shift_02() {
    let src = r#"x > >> y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x > >> y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_02",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_02",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShift_02",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShift_03 (case 65)
#[test]
fn unsigned_right_shift_03() {
    let src = r#"x >> > y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >> > y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_03",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShift_03",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShift_03",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShiftAssignment_01 (case 66)
#[test]
fn unsigned_right_shift_assignment_01() {
    let src = r#"x >>>= y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >>>= y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_01",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_01",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShiftAssignment_01",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShiftAssignment_02 (case 67)
#[test]
fn unsigned_right_shift_assignment_02() {
    let src = r#"x > >>= y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x > >>= y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_02",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_02",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShiftAssignment_02",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShiftAssignment_03 (case 68)
#[test]
fn unsigned_right_shift_assignment_03() {
    let src = r#"x >> >= y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >> >= y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_03",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_03",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShiftAssignment_03",
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

/// Roslyn: ExpressionParsingTests.UnsignedRightShiftAssignment_04 (case 69)
#[test]
fn unsigned_right_shift_assignment_04() {
    let src = r#"x >>> = y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >>> = y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_04",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "UnsignedRightShiftAssignment_04",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "UnsignedRightShiftAssignment_04",
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

/// Roslyn: ExpressionParsingTests.ObjectInitializerWithColonInsteadOfEqualsSign (case 70)
#[test]
fn object_initializer_with_colon_instead_of_equals_sign() {
    let src = r#"new Class1 { X: 0 }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new Class1 { X: 0 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSign",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSign",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ObjectInitializerWithColonInsteadOfEqualsSign",
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

/// Roslyn: ExpressionParsingTests.ImplicitObjectInitializerWithColonInsteadOfEqualsSign (case 71)
#[test]
fn implicit_object_initializer_with_colon_instead_of_equals_sign() {
    let src = r#"Class1 c1 = new() { X: 0 };"#;
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ImplicitObjectInitializerWithColonInsteadOfEqualsSign",
                    71,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ImplicitObjectInitializerWithColonInsteadOfEqualsSign",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ImplicitObjectInitializerWithColonInsteadOfEqualsSign",
            71,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ExpressionParsingTests.ObjectInitializerWithColonInsteadOfEqualsSignInNestedInitialization (case 72)
#[test]
fn object_initializer_with_colon_instead_of_equals_sign_in_nested_initialization() {
    let src = r#"new Class1 { X = { Y: 0 } }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new Class1 { X = { Y: 0 } }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSignInNestedInitialization",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSignInNestedInitialization",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ObjectInitializerWithColonInsteadOfEqualsSignInNestedInitialization",
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

/// Roslyn: ExpressionParsingTests.ObjectInitializerWithColonInsteadOfEqualsSignInCollectionInitializer (case 73)
#[test]
fn object_initializer_with_colon_instead_of_equals_sign_in_collection_initializer() {
    let src = r#"new Class1 { X = { [0] = { Y: 0 } } }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new Class1 { X = { [0] = { Y: 0 } } }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSignInCollectionInitializer",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonInsteadOfEqualsSignInCollectionInitializer",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ObjectInitializerWithColonInsteadOfEqualsSignInCollectionInitializer",
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

/// Roslyn: ExpressionParsingTests.ObjectInitializerWithColonAndDashInsteadOfEqualsSigns (case 74)
#[test]
fn object_initializer_with_colon_and_dash_instead_of_equals_signs() {
    let src = r#"new Class1 { X: 0, Y - 0 }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new Class1 { X: 0, Y - 0 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonAndDashInsteadOfEqualsSigns",
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
                    "expression_parsing_tests",
                    "ExpressionParsingTests",
                    "ObjectInitializerWithColonAndDashInsteadOfEqualsSigns",
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
            "expression_parsing_tests",
            "ExpressionParsingTests",
            "ObjectInitializerWithColonAndDashInsteadOfEqualsSigns",
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
