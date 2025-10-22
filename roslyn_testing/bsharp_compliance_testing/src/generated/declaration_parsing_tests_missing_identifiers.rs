// Auto-generated from Roslyn: DeclarationParsingTests_MissingIdentifiers
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Fixed (case 1)
#[test]
fn definite_statement_after_generic_type_fixed() {
    let src = r#"
                void M()
                {
                    List<Type>
                    fixed
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 10,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    fixed
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Fixed",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Fixed",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Fixed",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Fixed_DoubleGeneric (case 2)
#[test]
fn definite_statement_after_generic_type_fixed_double_generic() {
    let src = r#"
                void M()
                {
                    List<List<Type>>
                    fixed
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 10,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<List<Type>>
                    fixed
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Fixed_DoubleGeneric",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Fixed_DoubleGeneric",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Fixed_DoubleGeneric",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Break (case 3)
#[test]
fn definite_statement_after_generic_type_break() {
    let src = r#"
                void M()
                {
                    List<Type>
                    break
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    break
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Break",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Break",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Break",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Continue (case 4)
#[test]
fn definite_statement_after_generic_type_continue() {
    let src = r#"
                void M()
                {
                    List<Type>
                    continue
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    continue
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Continue",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Continue",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Continue",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Try (case 5)
#[test]
fn definite_statement_after_generic_type_try() {
    let src = r#"
                void M()
                {
                    List<Type>
                    try
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    try
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Try",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Try",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Try",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Do (case 6)
#[test]
fn definite_statement_after_generic_type_do() {
    let src = r#"
                void M()
                {
                    List<Type>
                    do
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 9,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    do
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Do",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Do",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Do",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_For (case 7)
#[test]
fn definite_statement_after_generic_type_for() {
    let src = r#"
                void M()
                {
                    List<Type>
                    for
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 11,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    for
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_For",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_For",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_For",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Foreach (case 8)
#[test]
fn definite_statement_after_generic_type_foreach() {
    let src = r#"
                void M()
                {
                    List<Type>
                    foreach
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 10,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    foreach
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Foreach",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Foreach",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Foreach",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Goto (case 9)
#[test]
fn definite_statement_after_generic_type_goto() {
    let src = r#"
                void M()
                {
                    List<Type>
                    goto
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    goto
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Goto",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Goto",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Goto",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_If (case 10)
#[test]
fn definite_statement_after_generic_type_if() {
    let src = r#"
                void M()
                {
                    List<Type>
                    if
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    if
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_If",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_If",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_If",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Else (case 11)
#[test]
fn definite_statement_after_generic_type_else() {
    let src = r#"
                void M()
                {
                    List<Type>
                    else
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 10,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    else
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Else",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Else",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Else",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Lock (case 12)
#[test]
fn definite_statement_after_generic_type_lock() {
    let src = r#"
                void M()
                {
                    List<Type>
                    lock
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    lock
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Lock",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Lock",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Lock",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Return (case 13)
#[test]
fn definite_statement_after_generic_type_return() {
    let src = r#"
                void M()
                {
                    List<Type>
                    return
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    return
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Return",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Return",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Return",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Switch (case 14)
#[test]
fn definite_statement_after_generic_type_switch() {
    let src = r#"
                void M()
                {
                    List<Type>
                    switch
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    switch
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Switch",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Switch",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Switch",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Unsafe (case 15)
#[test]
fn definite_statement_after_generic_type_unsafe() {
    let src = r#"
                void M()
                {
                    List<Type>
                    unsafe
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 8,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    unsafe
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Unsafe",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Unsafe",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Unsafe",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Using (case 16)
#[test]
fn definite_statement_after_generic_type_using() {
    let src = r#"
                void M()
                {
                    List<Type>
                    using
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    using
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Using",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Using",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Using",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_While (case 17)
#[test]
fn definite_statement_after_generic_type_while() {
    let src = r#"
                void M()
                {
                    List<Type>
                    while
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 7,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    while
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_While",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_While",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_While",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Volatile (case 18)
#[test]
fn definite_statement_after_generic_type_volatile() {
    let src = r#"
                void M()
                {
                    List<Type>
                    volatile
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 8,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    volatile
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Volatile",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Volatile",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Volatile",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Extern (case 19)
#[test]
fn definite_statement_after_generic_type_extern() {
    let src = r#"
                void M()
                {
                    List<Type>
                    extern
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 8,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    extern
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Extern",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Extern",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Extern",
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

/// Roslyn: DeclarationParsingTests_MissingIdentifiers.DefiniteStatementAfterGenericType_Case (case 20)
#[test]
fn definite_statement_after_generic_type_case() {
    let src = r#"
                void M()
                {
                    List<Type>
                    case
                }
                "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
                void M()
                {
                    List<Type>
                    case
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Case",
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
                    "declaration_parsing_tests_missing_identifiers",
                    "DeclarationParsingTests_MissingIdentifiers",
                    "DefiniteStatementAfterGenericType_Case",
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
            "declaration_parsing_tests_missing_identifiers",
            "DeclarationParsingTests_MissingIdentifiers",
            "DefiniteStatementAfterGenericType_Case",
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
