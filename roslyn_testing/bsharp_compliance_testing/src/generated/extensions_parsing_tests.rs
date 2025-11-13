// Auto-generated from Roslyn: ExtensionsParsingTests
/// Roslyn: ExtensionsParsingTests.MultipleConstraints (case 1)
#[test]
fn multiple_constraints() {
    let src = r#"
class C
{
    extension<T1, T2>(object o) where T1 : struct where T2 : class { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MultipleConstraints",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MultipleConstraints",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MultipleConstraints",
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

/// Roslyn: ExtensionsParsingTests.MultipleConstraints_Incomplete (case 2)
#[test]
fn multiple_constraints_incomplete() {
    let src = r#"
class C
{
    extension<T1, T2>(object o) where T1 where T2 : class { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MultipleConstraints_Incomplete",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MultipleConstraints_Incomplete",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MultipleConstraints_Incomplete",
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

/// Roslyn: ExtensionsParsingTests.WithName (case 3)
#[test]
fn with_name() {
    let src = r#"
class C
{
    extension Name(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithName",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithName",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithName",
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

/// Roslyn: ExtensionsParsingTests.WithName_02 (case 4)
#[test]
fn with_name_02() {
    let src = r#"
class C
{
    extension Name<T>(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithName_02",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithName_02",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithName_02",
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

/// Roslyn: ExtensionsParsingTests.TypeNamedExtension (case 5)
#[test]
fn type_named_extension() {
    let src = r#"
class extension
{
    extension(Type constructorParameter) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "TypeNamedExtension",
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

/// Roslyn: ExtensionsParsingTests.TypeNamedExtension (case 6)
#[test]
fn type_named_extension_case_2() {
    let src = r#"
class extension
{
    extension(Type constructorParameter) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "TypeNamedExtension",
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

/// Roslyn: ExtensionsParsingTests.TypeNamedExtension (case 7)
#[test]
fn type_named_extension_case_3() {
    let src = r#"
class extension
{
    @extension(Type constructorParameter) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "TypeNamedExtension",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "TypeNamedExtension",
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

/// Roslyn: ExtensionsParsingTests.ReceiverParameter_NoName (case 8)
#[test]
fn receiver_parameter_no_name() {
    let src = r#"
class C
{
    extension(Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ReceiverParameter_NoName",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ReceiverParameter_NoName",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "ReceiverParameter_NoName",
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

/// Roslyn: ExtensionsParsingTests.ReceiverParameter_NoName_02 (case 9)
#[test]
fn receiver_parameter_no_name_02() {
    let src = r#"
class C
{
    extension(object) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ReceiverParameter_NoName_02",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ReceiverParameter_NoName_02",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "ReceiverParameter_NoName_02",
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

/// Roslyn: ExtensionsParsingTests.NoClosingBrace (case 10)
#[test]
fn no_closing_brace() {
    let src = r#"
class C
{
    extension(Type) { void M() { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "NoClosingBrace",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "NoClosingBrace",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "NoClosingBrace",
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

/// Roslyn: ExtensionsParsingTests.WithAttributes (case 11)
#[test]
fn with_attributes() {
    let src = r#"
class C
{
    [MyAttribute]
    extension(Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithAttributes",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithAttributes",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithAttributes",
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

/// Roslyn: ExtensionsParsingTests.WithModifiers_Scoped (case 12)
#[test]
fn with_modifiers_scoped() {
    let src = r#"
class C
{
    scoped extension(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Scoped",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Scoped",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifiers_Scoped",
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

/// Roslyn: ExtensionsParsingTests.WithModifiers_Async (case 13)
#[test]
fn with_modifiers_async() {
    let src = r#"
class C
{
    async extension(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Async",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Async",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifiers_Async",
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

/// Roslyn: ExtensionsParsingTests.WithModifiers_Const (case 14)
#[test]
fn with_modifiers_const() {
    let src = r#"
class C
{
    const extension(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Const",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Const",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifiers_Const",
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

/// Roslyn: ExtensionsParsingTests.WithModifiers_Fixed (case 15)
#[test]
fn with_modifiers_fixed() {
    let src = r#"
class C
{
    fixed extension(Type) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Fixed",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Fixed",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifiers_Fixed",
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

/// Roslyn: ExtensionsParsingTests.WithModifiers_Ref (case 16)
#[test]
fn with_modifiers_ref() {
    let src = r#"
class C
{
    ref extension(Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Ref",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifiers_Ref",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifiers_Ref",
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

/// Roslyn: ExtensionsParsingTests.Member_MethodAndProperty (case 17)
#[test]
fn member_method_and_property() {
    let src = r#"
class C
{
    extension(Type)
    {
        void M() { }
        int Property { get; set; }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "Member_MethodAndProperty",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "Member_MethodAndProperty",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "Member_MethodAndProperty",
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

/// Roslyn: ExtensionsParsingTests.WithRef (case 18)
#[test]
fn with_ref() {
    let src = r#"
class C
{
    ref extension(Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithRef",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithRef",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithRef",
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

/// Roslyn: ExtensionsParsingTests.WithAttributeOnParameter (case 19)
#[test]
fn with_attribute_on_parameter() {
    let src = r#"
class C
{
    extension([MyAttribute] Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithAttributeOnParameter",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithAttributeOnParameter",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithAttributeOnParameter",
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

/// Roslyn: ExtensionsParsingTests.WithModifierOnParameter (case 20)
#[test]
fn with_modifier_on_parameter() {
    let src = r#"
class C
{
    extension(ref Type) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifierOnParameter",
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

/// Roslyn: ExtensionsParsingTests.WithModifierOnParameter_Scoped (case 21)
#[test]
fn with_modifier_on_parameter_scoped() {
    let src = r#"
class C
{
    extension(scoped Type x) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter_Scoped",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter_Scoped",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifierOnParameter_Scoped",
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

/// Roslyn: ExtensionsParsingTests.WithModifierOnParameter_ScopedRef (case 22)
#[test]
fn with_modifier_on_parameter_scoped_ref() {
    let src = r#"
class C
{
    extension(scoped ref Type x) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter_ScopedRef",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithModifierOnParameter_ScopedRef",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithModifierOnParameter_ScopedRef",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon (case 23)
#[test]
fn with_terminator_semi_colon() {
    let src = r#"
class C
{
    extension(Type) { ;
    class D { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_SemiColon",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon_02 (case 24)
#[test]
fn with_terminator_semi_colon_02() {
    let src = r#"
class C
{
    extension(Type) { ;
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon_02",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon_02",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_SemiColon_02",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon_03 (case 25)
#[test]
fn with_terminator_semi_colon_03() {
    let src = r#"
class C
{
    extension<T ;
    class D { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon_03",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_SemiColon_03",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_SemiColon_03",
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

/// Roslyn: ExtensionsParsingTests.MissingParameterList (case 26)
#[test]
fn missing_parameter_list() {
    let src = r#"
class C
{
    extension ;
    class D { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingParameterList",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingParameterList",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MissingParameterList",
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

/// Roslyn: ExtensionsParsingTests.SemiColonBody (case 27)
#[test]
fn semi_colon_body() {
    let src = r#"
class C
{
    extension<T>(Type) where T : struct;
    class D { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "SemiColonBody",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "SemiColonBody",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "SemiColonBody",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace (case 28)
#[test]
fn with_terminator_open_brace() {
    let src = r#"
class C
{
    extension(Type) { {
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace_02 (case 29)
#[test]
fn with_terminator_open_brace_02() {
    let src = r#"
class C
{
    extension<T>(Type) where { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_02",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_02",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace_02",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace_03 (case 30)
#[test]
fn with_terminator_open_brace_03() {
    let src = r#"
class C
{
    extension<T>(Type) where T { }
    class D { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_03",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_03",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace_03",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace_04 (case 31)
#[test]
fn with_terminator_open_brace_04() {
    let src = r#"
class C
{
    extension<T>(Type) where T : { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_04",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_04",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace_04",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace_05 (case 32)
#[test]
fn with_terminator_open_brace_05() {
    let src = r#"
class C
{
    extension<T>(Type) where T : struct, { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_05",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_05",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace_05",
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

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace_06 (case 33)
#[test]
fn with_terminator_open_brace_06() {
    let src = r#"
class C
{
    extension<T {
    class D { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_06",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithTerminator_OpenBrace_06",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithTerminator_OpenBrace_06",
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

/// Roslyn: ExtensionsParsingTests.MissingBraces_WithMethod (case 34)
#[test]
fn missing_braces_with_method() {
    let src = r#"
class C
{
    extension(Type)
    void M() { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingBraces_WithMethod",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingBraces_WithMethod",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MissingBraces_WithMethod",
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

/// Roslyn: ExtensionsParsingTests.MissingTypeAndIdentifier (case 35)
#[test]
fn missing_type_and_identifier() {
    let src = r#"
class C
{
    extension() { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingTypeAndIdentifier",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingTypeAndIdentifier",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MissingTypeAndIdentifier",
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

/// Roslyn: ExtensionsParsingTests.MissingTypeAndIdentifier_Ref (case 36)
#[test]
fn missing_type_and_identifier_ref() {
    let src = r#"
class C
{
    extension(ref) { }
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingTypeAndIdentifier_Ref",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MissingTypeAndIdentifier_Ref",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MissingTypeAndIdentifier_Ref",
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

/// Roslyn: ExtensionsParsingTests.MethodReturningExtension (case 37)
#[test]
fn method_returning_extension() {
    let src = r#"
class C
{
    @extension M() { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MethodReturningExtension",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MethodReturningExtension",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MethodReturningExtension",
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

/// Roslyn: ExtensionsParsingTests.MethodReturningExtension_02 (case 38)
#[test]
fn method_returning_extension_02() {
    let src = r#"
class C
{
    @extension M(Type x) { }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MethodReturningExtension_02",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "MethodReturningExtension_02",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "MethodReturningExtension_02",
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

/// Roslyn: ExtensionsParsingTests.ExtensionInExpression (case 39)
#[test]
fn extension_in_expression() {
    let src = r#"
class C
{
    void extension() { extension(); }
    void M()
    {
        extension extension = null;
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ExtensionInExpression",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ExtensionInExpression",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "ExtensionInExpression",
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

/// Roslyn: ExtensionsParsingTests.ParameterNameIsWhereOfConstraint (case 40)
#[test]
fn parameter_name_is_where_of_constraint() {
    let src = r#"
class C
{
    extension(object where T :
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ParameterNameIsWhereOfConstraint",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "ParameterNameIsWhereOfConstraint",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "ParameterNameIsWhereOfConstraint",
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

/// Roslyn: ExtensionsParsingTests.WithBodyAndSemiColon (case 41)
#[test]
fn with_body_and_semi_colon() {
    let src = r#"
class C
{
    extension(object) { };
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithBodyAndSemiColon",
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
                    "extensions_parsing_tests",
                    "ExtensionsParsingTests",
                    "WithBodyAndSemiColon",
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
            "extensions_parsing_tests",
            "ExtensionsParsingTests",
            "WithBodyAndSemiColon",
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
