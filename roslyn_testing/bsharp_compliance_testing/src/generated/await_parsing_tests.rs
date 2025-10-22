// Auto-generated from Roslyn: AwaitParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: AwaitParsingTests.AwaitOnIdentifierInAsynchronousContext (case 1)
#[test]
fn await_on_identifier_in_asynchronous_context() {
    let src = r#"
class C
{
    async void f()
    {
        await goo();
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitOnIdentifierInAsynchronousContext",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitOnIdentifierInAsynchronousContext",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitOnIdentifierInAsynchronousContext",
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

/// Roslyn: AwaitParsingTests.AwaitOnIdentifierInSynchronousContext (case 2)
#[test]
fn await_on_identifier_in_synchronous_context() {
    let src = r#"
class C
{
    void f()
    {
        await goo();
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitOnIdentifierInSynchronousContext",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitOnIdentifierInSynchronousContext",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitOnIdentifierInSynchronousContext",
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

/// Roslyn: AwaitParsingTests.AwaitStatement (case 3)
#[test]
fn await_statement() {
    let src = r#"
class C
{
    async void f()
    {
        await 1;
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitStatement",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitStatement",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitStatement",
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

/// Roslyn: AwaitParsingTests.NestedLambdaAwait (case 4)
#[test]
fn nested_lambda_await() {
    let src = r#"
class C
{
    void f()
    {
        async () => {
            await 1;
            () => {
                int await;
            };
        };
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "NestedLambdaAwait",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "NestedLambdaAwait",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "NestedLambdaAwait",
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

/// Roslyn: AwaitParsingTests.AwaitExpr (case 5)
#[test]
fn await_expr() {
    let src = r#"
class C
{
    async void f()
    {
        int c = await g() || await g();
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitExpr",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitExpr",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitExpr",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern1 (case 6)
#[test]
fn await_in_conditional_expression_after_pattern_1() {
    let src = r#"x is int ? await y : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int ? await y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern1",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern1",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern1",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern2 (case 7)
#[test]
fn await_in_conditional_expression_after_pattern_2() {
    let src = r#"x is int ? await this.SomeMethodAsync() : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int ? await this.SomeMethodAsync() : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern2",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern2",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern2",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern3 (case 8)
#[test]
fn await_in_conditional_expression_after_pattern_3() {
    let src = r#"x is int ? await base.SomeMethodAsync() : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int ? await base.SomeMethodAsync() : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern3",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern3",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern3",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern4 (case 9)
#[test]
fn await_in_conditional_expression_after_pattern_4() {
    let src = r#"x is int ? await (myTask) : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int ? await (myTask) : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern4",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern4",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern4",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern5 (case 10)
#[test]
fn await_in_conditional_expression_after_pattern_5() {
    let src = r#"
                void M()
                {
                    var c = x is X ? await y : z;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    var c = x is X ? await y : z;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern5",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern5",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern5",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern6 (case 11)
#[test]
fn await_in_conditional_expression_after_pattern_6() {
    let src = r#"
                async void M()
                {
                    var c = x is X ? await y : z;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
                async void M()
                {
                    var c = x is X ? await y : z;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern6",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern6",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern6",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern7 (case 12)
#[test]
fn await_in_conditional_expression_after_pattern_7() {
    let src = r#"
                void M()
                {
                    var c = x is X ? await(y) : z;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    var c = x is X ? await(y) : z;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern7",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern7",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern7",
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

/// Roslyn: AwaitParsingTests.AwaitInConditionalExpressionAfterPattern8 (case 13)
#[test]
fn await_in_conditional_expression_after_pattern_8() {
    let src = r#"
                async void M()
                {
                    var c = x is X ? await (y) : z;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
                async void M()
                {
                    var c = x is X ? await (y) : z;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern8",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitInConditionalExpressionAfterPattern8",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitInConditionalExpressionAfterPattern8",
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

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional1 (case 14)
#[test]
fn await_as_start_of_expression_in_conditional_1() {
    let src = r#"f(x is int? await)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { f(x is int? await); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional1",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional1",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitAsStartOfExpressionInConditional1",
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

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional2 (case 15)
#[test]
fn await_as_start_of_expression_in_conditional_2() {
    let src = r#"dict[x is int? await]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { dict[x is int? await]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional2",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional2",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitAsStartOfExpressionInConditional2",
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

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional3 (case 16)
#[test]
fn await_as_start_of_expression_in_conditional_3() {
    let src = r#"x is { Prop: int? await }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is { Prop: int? await }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional3",
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
                    "await_parsing_tests",
                    "AwaitParsingTests",
                    "AwaitAsStartOfExpressionInConditional3",
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
            "await_parsing_tests",
            "AwaitParsingTests",
            "AwaitAsStartOfExpressionInConditional3",
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
