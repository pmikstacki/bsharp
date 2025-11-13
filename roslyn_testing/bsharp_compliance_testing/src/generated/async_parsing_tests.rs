// Auto-generated from Roslyn: AsyncParsingTests
/// Roslyn: AsyncParsingTests.SimpleAsyncMethod (case 1)
#[test]
fn simple_async_method() {
    let src = r#"
class C
{
    async void M() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "SimpleAsyncMethod",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "SimpleAsyncMethod",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "SimpleAsyncMethod",
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

/// Roslyn: AsyncParsingTests.MethodCalledAsync (case 2)
#[test]
fn method_called_async() {
    let src = r#"
class C
{
    void async() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodCalledAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodCalledAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodCalledAsync",
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

/// Roslyn: AsyncParsingTests.MethodReturningAsync (case 3)
#[test]
fn method_returning_async() {
    let src = r#"
class C
{
    async M() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodReturningAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodReturningAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodReturningAsync",
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

/// Roslyn: AsyncParsingTests.MethodAsyncAsync (case 4)
#[test]
fn method_async_async() {
    let src = r#"
class C
{
    async async() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodAsyncAsync",
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

/// Roslyn: AsyncParsingTests.MethodAsyncAsyncAsync (case 5)
#[test]
fn method_async_async_async() {
    let src = r#"
class C
{
    async async async() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.MethodAsyncAsyncAsyncAsync (case 6)
#[test]
fn method_async_async_async_async() {
    let src = r#"
class C
{
    async async async async() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodAsyncAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.MethodAsyncVarAsync (case 7)
#[test]
fn method_async_var_async() {
    let src = r#"class C
{
    static async void M(object async)
    {
        async.F();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncVarAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "MethodAsyncVarAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "MethodAsyncVarAsync",
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

/// Roslyn: AsyncParsingTests.IncompleteAsync (case 8)
#[test]
fn incomplete_async() {
    let src = r#"
class C
{
    async
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsync",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncAsync (case 9)
#[test]
fn incomplete_async_async() {
    let src = r#"
class C
{
    async async
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncAsync",
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

/// Roslyn: AsyncParsingTests.CompleteAsyncAsync1 (case 10)
#[test]
fn complete_async_async_1() {
    let src = r#"
class C
{
    async async;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsync1",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsync1",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "CompleteAsyncAsync1",
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

/// Roslyn: AsyncParsingTests.CompleteAsyncAsync2 (case 11)
#[test]
fn complete_async_async_2() {
    let src = r#"
class C
{
    async async = 1;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsync2",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsync2",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "CompleteAsyncAsync2",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncAsyncAsync (case 12)
#[test]
fn incomplete_async_async_async() {
    let src = r#"
class C
{
    async async async
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.CompleteAsyncAsyncAsync (case 13)
#[test]
fn complete_async_async_async() {
    let src = r#"
class C
{
    async async async;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "CompleteAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncAsyncAsyncAsync (case 14)
#[test]
fn incomplete_async_async_async_async() {
    let src = r#"
class C
{
    async async async async
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.CompleteAsyncAsyncAsyncAsync (case 15)
#[test]
fn complete_async_async_async_async() {
    let src = r#"
class C
{
    async async async async;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "CompleteAsyncAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "CompleteAsyncAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember01 (case 16)
#[test]
fn incomplete_async_member_01() {
    let src = r#"
class C
{
    async Task<
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember01",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember01",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember01",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember02 (case 17)
#[test]
fn incomplete_async_member_02() {
    let src = r#"
class C
{
    async Tasks.Task<
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember02",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember02",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember02",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember03 (case 18)
#[test]
fn incomplete_async_member_03() {
    let src = r#"
class C
{
    static async Tasks.Task<
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember03",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember03",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember03",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember04 (case 19)
#[test]
fn incomplete_async_member_04() {
    let src = r#"
class C
{
    async operator+
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember04",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember04",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember04",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember05 (case 20)
#[test]
fn incomplete_async_member_05() {
    let src = r#"
class C
{
    async Task<T>
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember05",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember05",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember05",
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

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember06 (case 21)
#[test]
fn incomplete_async_member_06() {
    let src = r#"
class C
{
    async Task<T> f
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember06",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "IncompleteAsyncMember06",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "IncompleteAsyncMember06",
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

/// Roslyn: AsyncParsingTests.PropertyAsyncAsync (case 22)
#[test]
fn property_async_async() {
    let src = r#"
class C
{
    async async { get; set; }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "PropertyAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "PropertyAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "PropertyAsyncAsync",
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

/// Roslyn: AsyncParsingTests.PropertyAsyncAsyncAsync (case 23)
#[test]
fn property_async_async_async() {
    let src = r#"
class C
{
    async async async { get; set; }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "PropertyAsyncAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "PropertyAsyncAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "PropertyAsyncAsyncAsync",
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

/// Roslyn: AsyncParsingTests.EventAsyncAsync (case 24)
#[test]
fn event_async_async() {
    let src = r#"
class C
{
    event async async;
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsync",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsync",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "EventAsyncAsync",
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

/// Roslyn: AsyncParsingTests.EventAsyncAsyncAsync1 (case 25)
#[test]
fn event_async_async_async_1() {
    let src = r#"
class C
{
    event async async async;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsyncAsync1",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsyncAsync1",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "EventAsyncAsyncAsync1",
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

/// Roslyn: AsyncParsingTests.EventAsyncAsyncAsync2 (case 26)
#[test]
fn event_async_async_async_2() {
    let src = r#"
class C
{
    async event async async;
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsyncAsync2",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "EventAsyncAsyncAsync2",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "EventAsyncAsyncAsync2",
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

/// Roslyn: AsyncParsingTests.AsyncModifierOnDelegateDeclaration (case 27)
#[test]
fn async_modifier_on_delegate_declaration() {
    let src = r#"
class C
{
    public async delegate void Goo();
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncModifierOnDelegateDeclaration",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncModifierOnDelegateDeclaration",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncModifierOnDelegateDeclaration",
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

/// Roslyn: AsyncParsingTests.AsyncInterface (case 28)
#[test]
fn async_interface() {
    let src = r#"
class C
{
    async interface
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncInterface",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncInterface",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncInterface",
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

/// Roslyn: AsyncParsingTests.AsyncPartialClass (case 29)
#[test]
fn async_partial_class() {
    let src = r#"
class C
{
    async partial class
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialClass",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialClass",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialClass",
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

/// Roslyn: AsyncParsingTests.AsyncEvent (case 30)
#[test]
fn async_event() {
    let src = r#"
class C
{
    async event
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncEvent",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncEvent",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncEvent",
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

/// Roslyn: AsyncParsingTests.AsyncPartialEvent (case 31)
#[test]
fn async_partial_event() {
    let src = r#"
class C
{
    async partial event
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialEvent",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialEvent",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialEvent",
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

/// Roslyn: AsyncParsingTests.AsyncImplicitOperator (case 32)
#[test]
fn async_implicit_operator() {
    let src = r#"
class C
{
    async implicit operator
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncImplicitOperator",
                    32,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncImplicitOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncImplicitOperator",
            32,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncPartialImplicitOperator (case 33)
#[test]
fn async_partial_implicit_operator() {
    let src = r#"
class C
{
    async partial implicit operator
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialImplicitOperator",
                    33,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialImplicitOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialImplicitOperator",
            33,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncExplicitOperator (case 34)
#[test]
fn async_explicit_operator() {
    let src = r#"
class C
{
    async explicit operator
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncExplicitOperator",
                    34,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncExplicitOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncExplicitOperator",
            34,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncPartialExplicitOperator (case 35)
#[test]
fn async_partial_explicit_operator() {
    let src = r#"
class C
{
    async partial explicit operator
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialExplicitOperator",
                    35,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialExplicitOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialExplicitOperator",
            35,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeOperator (case 36)
#[test]
fn async_type_operator() {
    let src = r#"
class C
{
    async C operator
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeOperator",
                    36,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeOperator",
            36,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncPartialTypeOperator (case 37)
#[test]
fn async_partial_type_operator() {
    let src = r#"
class C
{
    async partial int operator
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialTypeOperator",
                    37,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialTypeOperator",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialTypeOperator",
            37,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncField (case 38)
#[test]
fn async_field() {
    let src = r#"
class C
{
    async C C
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncField",
                    38,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncField",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncField",
            38,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncPartialIndexer (case 39)
#[test]
fn async_partial_indexer() {
    let src = r#"
class C
{
    async partial C this
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialIndexer",
                    39,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncPartialIndexer",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncPartialIndexer",
            39,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeEndOfFile (case 40)
#[test]
fn async_type_end_of_file() {
    let src = r#"class C { async T"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeEndOfFile",
                    40,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeEndOfFile",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeEndOfFile",
            40,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeCloseCurly (case 41)
#[test]
fn async_type_close_curly() {
    let src = r#"class C { async T }"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeCloseCurly",
                    41,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeCloseCurly",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeCloseCurly",
            41,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypePredefinedType (case 42)
#[test]
fn async_type_predefined_type() {
    let src = r#"class C {
    async T
    int"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypePredefinedType",
                    42,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypePredefinedType",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypePredefinedType",
            42,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeModifier (case 43)
#[test]
fn async_type_modifier() {
    let src = r#"class C {
    async T
    public"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeModifier",
                    43,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeModifier",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeModifier",
            43,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeFollowedByTypeDecl (case 44)
#[test]
fn async_type_followed_by_type_decl() {
    let src = r#"class C {
    async T
class"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeFollowedByTypeDecl",
                    44,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeFollowedByTypeDecl",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeFollowedByTypeDecl",
            44,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncTypeFollowedByNamespaceDecl (case 45)
#[test]
fn async_type_followed_by_namespace_decl() {
    let src = r#"class C {
    async T
namespace"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeFollowedByNamespaceDecl",
                    45,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncTypeFollowedByNamespaceDecl",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncTypeFollowedByNamespaceDecl",
            45,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncGenericType (case 46)
#[test]
fn async_generic_type() {
    let src = r#"class Program
{
    public async Task<IReadOnlyCollection<ProjectConfiguration>>
}"#;
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncGenericType",
                    46,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncGenericType",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncGenericType",
            46,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncAsType_Property (case 47)
#[test]
fn async_as_type_property() {
    let src = r#"class async { async async { get; } }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncAsType_Property",
                    47,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncAsType_Property",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncAsType_Property",
            47,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncAsType_Indexer (case 48)
#[test]
fn async_as_type_indexer() {
    let src = r#"interface async { async this[async i] { get; } }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncAsType_Indexer",
                    48,
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncAsType_Indexer",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncAsType_Indexer",
            48,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern1 (case 49)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_1() {
    let src = r#"x is A ? async b => 0 : null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A ? async b => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern1",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern1",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncLambdaInConditionalExpressionAfterPattern1",
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

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern2 (case 50)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_2() {
    let src = r#"x is A a ? async b => 0 : null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A a ? async b => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern2",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern2",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncLambdaInConditionalExpressionAfterPattern2",
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

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern3 (case 51)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_3() {
    let src = r#"x is A ? async (b) => 0 : null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A ? async (b) => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern3",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern3",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncLambdaInConditionalExpressionAfterPattern3",
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

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern4 (case 52)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_4() {
    let src = r#"x is A a ? async (b) => 0 : null"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A a ? async (b) => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern4",
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
                    "async_parsing_tests",
                    "AsyncParsingTests",
                    "AsyncLambdaInConditionalExpressionAfterPattern4",
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
            "async_parsing_tests",
            "AsyncParsingTests",
            "AsyncLambdaInConditionalExpressionAfterPattern4",
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
