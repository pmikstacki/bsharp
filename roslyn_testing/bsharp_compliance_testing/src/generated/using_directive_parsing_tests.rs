// Auto-generated from Roslyn: UsingDirectiveParsingTests
/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectiveNamePointer (case 1)
#[test]
fn simple_using_directive_name_pointer() {
    let src = r#"using A*;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveNamePointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveNamePointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectiveNamePointer",
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

/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectiveRefType (case 2)
#[test]
fn simple_using_directive_ref_type() {
    let src = r#"using ref int;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveRefType",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveRefType",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectiveRefType",
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

/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectiveFunctionPointer (case 3)
#[test]
fn simple_using_directive_function_pointer() {
    let src = r#"using delegate*<int, void>;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveFunctionPointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveFunctionPointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectiveFunctionPointer",
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

/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectivePredefinedType (case 4)
#[test]
fn simple_using_directive_predefined_type() {
    let src = r#"using int;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectivePredefinedType",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectivePredefinedType",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectivePredefinedType",
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

/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectivePredefinedTypePointer (case 5)
#[test]
fn simple_using_directive_predefined_type_pointer() {
    let src = r#"using int*;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectivePredefinedTypePointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectivePredefinedTypePointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectivePredefinedTypePointer",
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

/// Roslyn: UsingDirectiveParsingTests.SimpleUsingDirectiveTuple (case 6)
#[test]
fn simple_using_directive_tuple() {
    let src = r#"using (int, int);"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveTuple",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "SimpleUsingDirectiveTuple",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "SimpleUsingDirectiveTuple",
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

/// Roslyn: UsingDirectiveParsingTests.StaticUsingDirectiveNamePointer (case 7)
#[test]
fn static_using_directive_name_pointer() {
    let src = r#"using static A*;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveNamePointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveNamePointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "StaticUsingDirectiveNamePointer",
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

/// Roslyn: UsingDirectiveParsingTests.StaticUsingDirectiveFunctionPointer (case 8)
#[test]
fn static_using_directive_function_pointer() {
    let src = r#"using static delegate*<int, void>;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveFunctionPointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveFunctionPointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "StaticUsingDirectiveFunctionPointer",
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

/// Roslyn: UsingDirectiveParsingTests.StaticUsingDirectivePredefinedType (case 9)
#[test]
fn static_using_directive_predefined_type() {
    let src = r#"using static int;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectivePredefinedType",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectivePredefinedType",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "StaticUsingDirectivePredefinedType",
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

/// Roslyn: UsingDirectiveParsingTests.StaticUsingDirectivePredefinedTypePointer (case 10)
#[test]
fn static_using_directive_predefined_type_pointer() {
    let src = r#"using static int*;"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectivePredefinedTypePointer",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectivePredefinedTypePointer",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "StaticUsingDirectivePredefinedTypePointer",
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

/// Roslyn: UsingDirectiveParsingTests.StaticUsingDirectiveTuple (case 11)
#[test]
fn static_using_directive_tuple() {
    let src = r#"using static (int, int);"#;
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveTuple",
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
                    "using_directive_parsing_tests",
                    "UsingDirectiveParsingTests",
                    "StaticUsingDirectiveTuple",
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
            "using_directive_parsing_tests",
            "UsingDirectiveParsingTests",
            "StaticUsingDirectiveTuple",
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
