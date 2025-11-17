// Auto-generated from Roslyn: LocalFunctionParsingTests
/// Roslyn: LocalFunctionParsingTests.IncompleteLocalFunc (case 1)
#[test]
fn incomplete_local_func() {
    let src = r#"
class C
{
    void M1()
    {
        await L<
    }
    void M2()
    {
        int L<
    }
    void M3()
    {
        int? L<
    }
    void M4()
    {
        await L(
    }
    void M5()
    {
        int L(
    }
    void M6()
    {
        int? L(
    }
}"#;
    let expected = Some(ExpectedDiagnostics {
        count: 12,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "IncompleteLocalFunc",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "IncompleteLocalFunc",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "IncompleteLocalFunc",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionAttribute (case 2)
#[test]
fn local_function_attribute() {
    let src = r#"
class C
{
    void M()
    {
        [A]
        void local() { }

        [return: A]
        void local() { }

        [A]
        int local() => 42;

        [A][B] void local() { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionAttribute",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionModifier_Error_LocalVariable (case 3)
#[test]
fn local_function_modifier_error_local_variable() {
    let src = r#"
class C
{
    void M()
    {
        public object local;
    }
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionModifier_Error_LocalVariable",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionModifier_Error_LocalVariable",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionModifier_Error_LocalVariable",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunction_NoBody (case 4)
#[test]
fn local_function_no_body() {
    let src = r#"
class C
{
    void M()
    {
        void local();
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunction_NoBody",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunction_NoBody",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunction_NoBody",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionAttribute_Error_LocalVariable (case 5)
#[test]
fn local_function_attribute_error_local_variable() {
    let src = r#"
class C
{
    void M()
    {
        [A] object local;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_LocalVariable",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_LocalVariable",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionAttribute_Error_LocalVariable",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionAttribute_Error_LocalVariable_MultipleDeclarators (case 6)
#[test]
fn local_function_attribute_error_local_variable_multiple_declarators() {
    let src = r#"
class C
{
    void M()
    {
        [A] object local1, local2;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_LocalVariable_MultipleDeclarators",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_LocalVariable_MultipleDeclarators",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionAttribute_Error_LocalVariable_MultipleDeclarators",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionAttribute_Error_IncompleteMember (case 7)
#[test]
fn local_function_attribute_error_incomplete_member() {
    let src = r#"
class C
{
    void M()
    {
        [A]
    }
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_IncompleteMember",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionAttribute_Error_IncompleteMember",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionAttribute_Error_IncompleteMember",
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

/// Roslyn: LocalFunctionParsingTests.LocalFunctionsWithAwait (case 8)
#[test]
fn local_functions_with_await() {
    let src = r#"
class c
{
    void m1() { await await() => new await(); }
    void m2() { await () => new await(); }
    async void m3() { await () => new await(); }
    void m4() { async await() => new await(); }
    async void m5() { await async () => new await(); }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionsWithAwait",
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
                    "local_function_parsing_tests",
                    "LocalFunctionParsingTests",
                    "LocalFunctionsWithAwait",
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
            "local_function_parsing_tests",
            "LocalFunctionParsingTests",
            "LocalFunctionsWithAwait",
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
