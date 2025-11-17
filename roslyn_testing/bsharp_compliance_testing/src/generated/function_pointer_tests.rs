// Auto-generated from Roslyn: FunctionPointerTests
/// Roslyn: FunctionPointerTests.SimpleFunctionPointerTest (case 1)
#[test]
fn simple_function_pointer_test() {
    let src = r#"delegate*<string, Goo, int> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SimpleFunctionPointerTest",
                    1,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SimpleFunctionPointerTest",
                    1,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "SimpleFunctionPointerTest",
            1,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.KeywordInCallingConventionList (case 2)
#[test]
fn keyword_in_calling_convention_list() {
    let src = r#"delegate* unmanaged[void]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "KeywordInCallingConventionList",
                    2,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "KeywordInCallingConventionList",
                    2,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "KeywordInCallingConventionList",
            2,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.ManagedWithUnmanagedSpecifiers (case 3)
#[test]
fn managed_with_unmanaged_specifiers() {
    let src = r#"delegate* managed[Cdecl]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ManagedWithUnmanagedSpecifiers",
                    3,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ManagedWithUnmanagedSpecifiers",
                    3,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "ManagedWithUnmanagedSpecifiers",
            3,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.InvalidConventionWithUnmanagedSpecifiers (case 4)
#[test]
fn invalid_convention_with_unmanaged_specifiers() {
    let src = r#"delegate* invalid[Cdecl]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "InvalidConventionWithUnmanagedSpecifiers",
                    4,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "InvalidConventionWithUnmanagedSpecifiers",
                    4,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "InvalidConventionWithUnmanagedSpecifiers",
            4,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.InvalidConventionFollowedByTypeArguments (case 5)
#[test]
fn invalid_convention_followed_by_type_arguments() {
    let src = r#"delegate* invalid<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "InvalidConventionFollowedByTypeArguments",
                    5,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "InvalidConventionFollowedByTypeArguments",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "InvalidConventionFollowedByTypeArguments",
            5,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.EmptyUnmanagedSpecifierBraces (case 6)
#[test]
fn empty_unmanaged_specifier_braces() {
    let src = r#"delegate* unmanaged[]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "EmptyUnmanagedSpecifierBraces",
                    6,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "EmptyUnmanagedSpecifierBraces",
                    6,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "EmptyUnmanagedSpecifierBraces",
            6,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MultipleUnmanagedSpecifiers (case 7)
#[test]
fn multiple_unmanaged_specifiers() {
    let src = r#"delegate* unmanaged[Cdecl, Thiscall, Stdcall, Fastcall, Vectorcall, SuppressGCTransition]<void> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MultipleUnmanagedSpecifiers",
                    7,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MultipleUnmanagedSpecifiers",
                    7,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MultipleUnmanagedSpecifiers",
            7,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.VoidsAsType (case 8)
#[test]
fn voids_as_type() {
    let src = r#"delegate*<void, void, void> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "VoidsAsType",
                    8,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "VoidsAsType",
                    8,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "VoidsAsType",
            8,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.NestedFunctionPointers (case 9)
#[test]
fn nested_function_pointers() {
    let src =
        r#"delegate*<delegate* unmanaged[cdecl]<int*, void*>, delegate* managed<string*>> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "NestedFunctionPointers",
                    9,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "NestedFunctionPointers",
                    9,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "NestedFunctionPointers",
            9,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.PointerToAFunctionPointer (case 10)
#[test]
fn pointer_to_afunction_pointer() {
    let src = r#"delegate*<Goo, void>* ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "PointerToAFunctionPointer",
                    10,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "PointerToAFunctionPointer",
                    10,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "PointerToAFunctionPointer",
            10,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.RefModifiers (case 11)
#[test]
fn ref_modifiers() {
    let src = r#"delegate*<ref Goo, in Bar, out Baz, ref readonly void*> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "RefModifiers",
                    11,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "RefModifiers",
                    11,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "RefModifiers",
            11,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_01 (case 12)
#[test]
fn unterminated_01() {
    let src = r#"delegate*< ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_01",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_01",
                    12,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_01",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_02 (case 13)
#[test]
fn unterminated_02() {
    let src = r#"delegate*<ref ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_02",
                    13,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_02",
                    13,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_02",
            13,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_03 (case 14)
#[test]
fn unterminated_03() {
    let src = r#"delegate*<ref bar ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_03",
                    14,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_03",
                    14,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_03",
            14,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_04 (case 15)
#[test]
fn unterminated_04() {
    let src = r#"delegate*<ref bar, ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_04",
                    15,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_04",
                    15,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_04",
            15,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_05 (case 16)
#[test]
fn unterminated_05() {
    let src = r#"delegate* unmanaged[ptr];"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_05",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_05",
                    16,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_05",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_06 (case 17)
#[test]
fn unterminated_06() {
    let src = r#"delegate* unmanaged[cdecl] ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_06",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_06",
                    17,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_06",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_07 (case 18)
#[test]
fn unterminated_07() {
    let src = r#"delegate* unmanaged[cdecl] ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_07",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_07",
                    18,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_07",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_08 (case 19)
#[test]
fn unterminated_08() {
    let src = r#"delegate* ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_08",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_08",
                    19,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_08",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_09 (case 20)
#[test]
fn unterminated_09() {
    let src = r#"delegate* unmanaged.Name[Dotted]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_09",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_09",
                    20,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_09",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_10 (case 21)
#[test]
fn unterminated_10() {
    let src = r#"delegate*( ;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_10",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_10",
                    21,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_10",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_11 (case 22)
#[test]
fn unterminated_11() {
    let src = r#"delegate* @cdecl>"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_11",
                    22,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_11",
                    22,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_11",
            22,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_12 (case 23)
#[test]
fn unterminated_12() {
    let src = r#"delegate* unmanaged[ ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_12",
                    23,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_12",
                    23,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_12",
            23,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_13 (case 24)
#[test]
fn unterminated_13() {
    let src = r#"delegate* unmanaged[Cdecl ;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_13",
                    24,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_13",
                    24,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_13",
            24,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.Unterminated_14 (case 25)
#[test]
fn unterminated_14() {
    let src = r#"delegate* unmanaged[Cdecl,"#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_14",
                    25,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "Unterminated_14",
                    25,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "Unterminated_14",
            25,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.NoParamOrReturnTypes (case 26)
#[test]
fn no_param_or_return_types() {
    let src = r#"delegate*<> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "NoParamOrReturnTypes",
                    26,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "NoParamOrReturnTypes",
                    26,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "NoParamOrReturnTypes",
            26,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.UsingParensInsteadOfAngles (case 27)
#[test]
fn using_parens_instead_of_angles() {
    let src = r#"delegate*(int, void)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "UsingParensInsteadOfAngles",
                    27,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "UsingParensInsteadOfAngles",
                    27,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "UsingParensInsteadOfAngles",
            27,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MethodTypes (case 28)
#[test]
fn method_types() {
    let src = r#"
class C
{
    public delegate*<int, string> M(delegate*<C, void> param1, delegate* unmanaged[cdecl]<D> param2) {}
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MethodTypes",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MethodTypes",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MethodTypes",
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

/// Roslyn: FunctionPointerTests.HardCast (case 29)
#[test]
fn hard_cast() {
    let src = r#"(delegate* unmanaged[thiscall]<int, C>)ptr"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (delegate* unmanaged[thiscall]<int, C>)ptr; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "HardCast",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "HardCast",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "HardCast",
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

/// Roslyn: FunctionPointerTests.AsCast (case 30)
#[test]
fn as_cast() {
    let src = r#"ptr as delegate* unmanaged[stdcall]<int, void>"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ptr as delegate* unmanaged[stdcall]<int, void>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "AsCast",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "AsCast",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "AsCast",
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

/// Roslyn: FunctionPointerTests.TupleType (case 31)
#[test]
fn tuple_type() {
    let src = r#"((delegate*<int, void> i1, delegate* managed<C, D> i2))ptr"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { ((delegate*<int, void> i1, delegate* managed<C, D> i2))ptr; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "TupleType",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "TupleType",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "TupleType",
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

/// Roslyn: FunctionPointerTests.GenericArguments (case 32)
#[test]
fn generic_arguments() {
    let src = r#"new M<delegate* unmanaged[thiscall]<void>, delegate*<C, D>>()"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new M<delegate* unmanaged[thiscall]<void>, delegate*<C, D>>(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "GenericArguments",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "GenericArguments",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "GenericArguments",
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

/// Roslyn: FunctionPointerTests.TypeOf (case 33)
#[test]
fn type_of() {
    let src = r#"typeof(delegate* unmanaged[cdecl]<ref int, readonly ref D>)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 =
        r#"class C { void M() { typeof(delegate* unmanaged[cdecl]<ref int, readonly ref D>); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "TypeOf",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "TypeOf",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "TypeOf",
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

/// Roslyn: FunctionPointerTests.ArrayType (case 34)
#[test]
fn array_type() {
    let src = r#"delegate*<ref C>[] ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ArrayType",
                    34,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ArrayType",
                    34,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "ArrayType",
            34,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.LambdaParameterType (case 35)
#[test]
fn lambda_parameter_type() {
    let src = r#"(delegate*<void> p1) => {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (delegate*<void> p1) => {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "LambdaParameterType",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "LambdaParameterType",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "LambdaParameterType",
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

/// Roslyn: FunctionPointerTests.IsExpression (case 36)
#[test]
fn is_expression() {
    let src = r#"o is delegate*<void>"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is delegate*<void>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IsExpression",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IsExpression",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "IsExpression",
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

/// Roslyn: FunctionPointerTests.IsNamedExpression (case 37)
#[test]
fn is_named_expression() {
    let src = r#"o is delegate*<void> ptr"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is delegate*<void> ptr; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IsNamedExpression",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IsNamedExpression",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "IsNamedExpression",
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

/// Roslyn: FunctionPointerTests.SwitchStatementCase (case 38)
#[test]
fn switch_statement_case() {
    let src = r#"
switch (o)
{
    case delegate*<void> { } _:
    case delegate*<void> (var x, var y):
        break;
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SwitchStatementCase",
                    38,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SwitchStatementCase",
                    38,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "SwitchStatementCase",
            38,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.SwitchExpressions (case 39)
#[test]
fn switch_expressions() {
    let src = r#"
o switch
{
    delegate*<void> _ => 1,
    delegate*<void> (var a, 2) ptr => 2,
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
o switch
{
    delegate*<void> _ => 1,
    delegate*<void> (var a, 2) ptr => 2,
}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SwitchExpressions",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SwitchExpressions",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "SwitchExpressions",
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

/// Roslyn: FunctionPointerTests.SizeOf (case 40)
#[test]
fn size_of() {
    let src = r#"sizeof(delegate*<void>)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { sizeof(delegate*<void>); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SizeOf",
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
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SizeOf",
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "SizeOf",
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

/// Roslyn: FunctionPointerTests.SpecifiedParameterNamesAndDefaults (case 41)
#[test]
fn specified_parameter_names_and_defaults() {
    let src = r#"delegate*<int param1, string param2 = default, void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SpecifiedParameterNamesAndDefaults",
                    41,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "SpecifiedParameterNamesAndDefaults",
                    41,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "SpecifiedParameterNamesAndDefaults",
            41,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MissingListStart_01 (case 42)
#[test]
fn missing_list_start_01() {
    let src = r#"delegate*void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_01",
                    42,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_01",
                    42,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MissingListStart_01",
            42,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MissingListStart_02 (case 43)
#[test]
fn missing_list_start_02() {
    let src = r#"delegate* unmanaged[cdecl] void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_02",
                    43,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_02",
                    43,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MissingListStart_02",
            43,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MissingListStart_03 (case 44)
#[test]
fn missing_list_start_03() {
    let src = r#"delegate*> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_03",
                    44,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_03",
                    44,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MissingListStart_03",
            44,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MissingListStart_04 (case 45)
#[test]
fn missing_list_start_04() {
    let src = r#"delegate* unmanaged Cdecl]<void> ptr;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_04",
                    45,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MissingListStart_04",
                    45,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MissingListStart_04",
            45,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.ManyInvalidModifiers (case 46)
#[test]
fn many_invalid_modifiers() {
    let src = r#"delegate*<this params readonly ref ref this int> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ManyInvalidModifiers",
                    46,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "ManyInvalidModifiers",
                    46,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "ManyInvalidModifiers",
            46,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.IncompleteAtEndOfFile (case 47)
#[test]
fn incomplete_at_end_of_file() {
    let src = r#"delegate*"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IncompleteAtEndOfFile",
                    47,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IncompleteAtEndOfFile",
                    47,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "IncompleteAtEndOfFile",
            47,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.IncompleteAtEndOfFileWithCallingConvention (case 48)
#[test]
fn incomplete_at_end_of_file_with_calling_convention() {
    let src = r#"delegate* unmanaged[cdecl]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IncompleteAtEndOfFileWithCallingConvention",
                    48,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "IncompleteAtEndOfFileWithCallingConvention",
                    48,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "IncompleteAtEndOfFileWithCallingConvention",
            48,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MixedParensAndAngles_01 (case 49)
#[test]
fn mixed_parens_and_angles_01() {
    let src = r#"delegate* unmanaged[cdecl]<void) ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MixedParensAndAngles_01",
                    49,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MixedParensAndAngles_01",
                    49,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MixedParensAndAngles_01",
            49,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.MixedParensAndAngles_02 (case 50)
#[test]
fn mixed_parens_and_angles_02() {
    let src = r#"delegate* unmanaged[cdecl](void> ptr;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MixedParensAndAngles_02",
                    50,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "MixedParensAndAngles_02",
                    50,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "MixedParensAndAngles_02",
            50,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: FunctionPointerTests.FunctionPointerArrayInTypeArgument (case 51)
#[test]
fn function_pointer_array_in_type_argument() {
    let src = r#"I<delegate*<void>[]> i;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "FunctionPointerArrayInTypeArgument",
                    51,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "function_pointer_tests",
                    "FunctionPointerTests",
                    "FunctionPointerArrayInTypeArgument",
                    51,
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
            "function_pointer_tests",
            "FunctionPointerTests",
            "FunctionPointerArrayInTypeArgument",
            51,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
