// Auto-generated from Roslyn: StatementParsingTests
/// Roslyn: StatementParsingTests.ParsePrivate (case 1)
#[test]
fn parse_private() {
    let src = r#"private"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePrivate",
                    1,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePrivate",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParsePrivate",
            1,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarWithDeclarationTree (case 2)
#[test]
fn using_var_with_declaration_tree() {
    let src = r#"using T a = b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithDeclarationTree",
                    2,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithDeclarationTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarWithDeclarationTree",
            2,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarWithInvalidDeclaration (case 3)
#[test]
fn using_var_with_invalid_declaration() {
    let src = r#"using public readonly var a = b;"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithInvalidDeclaration",
                    3,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithInvalidDeclaration",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarWithInvalidDeclaration",
            3,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarWithVarDeclarationTree (case 4)
#[test]
fn using_var_with_var_declaration_tree() {
    let src = r#"using var a = b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithVarDeclarationTree",
                    4,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithVarDeclarationTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarWithVarDeclarationTree",
            4,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestAwaitUsingVarWithDeclarationTree (case 5)
#[test]
fn await_using_var_with_declaration_tree() {
    let src = r#"await using T a = b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithDeclarationTree",
                    5,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithDeclarationTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestAwaitUsingVarWithDeclarationTree",
            5,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestAwaitUsingVarWithVarDeclarationTree (case 6)
#[test]
fn await_using_var_with_var_declaration_tree() {
    let src = r#"await using var a = b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithVarDeclarationTree",
                    6,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithVarDeclarationTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestAwaitUsingVarWithVarDeclarationTree",
            6,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.AwaitUsingVarWithVarDecl_Reversed (case 7)
#[test]
fn await_using_var_with_var_decl_reversed() {
    let src = r#"
class C
{
    async void M()
    {
        using await var x = null;
    }
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "AwaitUsingVarWithVarDecl_Reversed",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "AwaitUsingVarWithVarDecl_Reversed",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "AwaitUsingVarWithVarDecl_Reversed",
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

/// Roslyn: StatementParsingTests.TestAwaitUsingVarWithVarAndNoUsingDeclarationTree (case 8)
#[test]
fn await_using_var_with_var_and_no_using_declaration_tree() {
    let src = r#"await var a = b;"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithVarAndNoUsingDeclarationTree",
                    8,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestAwaitUsingVarWithVarAndNoUsingDeclarationTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestAwaitUsingVarWithVarAndNoUsingDeclarationTree",
            8,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarWithDeclarationMultipleVariablesTree (case 9)
#[test]
fn using_var_with_declaration_multiple_variables_tree() {
    let src = r#"using T a = b, c = d;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithDeclarationMultipleVariablesTree",
                    9,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarWithDeclarationMultipleVariablesTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarWithDeclarationMultipleVariablesTree",
            9,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase1Tree (case 10)
#[test]
fn using_var_special_case_1_tree() {
    let src = r#"using var x = f ? a : b;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase1Tree",
                    10,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase1Tree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarSpecialCase1Tree",
            10,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase2Tree (case 11)
#[test]
fn using_var_special_case_2_tree() {
    let src = r#"using f ? x = a;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase2Tree",
                    11,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase2Tree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarSpecialCase2Tree",
            11,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase3Tree (case 12)
#[test]
fn using_var_special_case_3_tree() {
    let src = r#"using f? x, y;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase3Tree",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarSpecialCase3Tree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarSpecialCase3Tree",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarRefTree (case 13)
#[test]
fn using_var_ref_tree() {
    let src = r#"using ref int x = ref y;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefTree",
                    13,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarRefTree",
            13,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarRefReadonlyTree (case 14)
#[test]
fn using_var_ref_readonly_tree() {
    let src = r#"using ref readonly int x = ref y;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefReadonlyTree",
                    14,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefReadonlyTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarRefReadonlyTree",
            14,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarRefVarTree (case 15)
#[test]
fn using_var_ref_var_tree() {
    let src = r#"using ref var x = ref y;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefVarTree",
                    15,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefVarTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarRefVarTree",
            15,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarRefVarIsYTree (case 16)
#[test]
fn using_var_ref_var_is_ytree() {
    let src = r#"using ref var x = y;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefVarIsYTree",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarRefVarIsYTree",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarRefVarIsYTree",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestUsingVarReadonlyMultipleDeclarations (case 17)
#[test]
fn using_var_readonly_multiple_declarations() {
    let src = r#"using readonly var x, y = ref z;"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarReadonlyMultipleDeclarations",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestUsingVarReadonlyMultipleDeclarations",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestUsingVarReadonlyMultipleDeclarations",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.NullExceptionInLabeledStatement (case 18)
#[test]
fn null_exception_in_labeled_statement() {
    let src = r#"{ label: public"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "NullExceptionInLabeledStatement",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "NullExceptionInLabeledStatement",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "NullExceptionInLabeledStatement",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseElseWithoutPrecedingIfStatement (case 19)
#[test]
fn parse_else_without_preceding_if_statement() {
    let src = r#"else {}"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseWithoutPrecedingIfStatement",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseWithoutPrecedingIfStatement",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseElseWithoutPrecedingIfStatement",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseElseAndElseWithoutPrecedingIfStatement (case 20)
#[test]
fn parse_else_and_else_without_preceding_if_statement() {
    let src = r#"{ else {} else {} }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 12,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseAndElseWithoutPrecedingIfStatement",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseAndElseWithoutPrecedingIfStatement",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseElseAndElseWithoutPrecedingIfStatement",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSubsequentElseWithoutPrecedingIfStatement (case 21)
#[test]
fn parse_subsequent_else_without_preceding_if_statement() {
    let src = r#"{ if (a) { } else { } else { } }"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSubsequentElseWithoutPrecedingIfStatement",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSubsequentElseWithoutPrecedingIfStatement",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSubsequentElseWithoutPrecedingIfStatement",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseElseKeywordPlacedAsIfEmbeddedStatement (case 22)
#[test]
fn parse_else_keyword_placed_as_if_embedded_statement() {
    let src = r#"if (a) else {}"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseKeywordPlacedAsIfEmbeddedStatement",
                    22,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseElseKeywordPlacedAsIfEmbeddedStatement",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseElseKeywordPlacedAsIfEmbeddedStatement",
            22,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitch01 (case 23)
#[test]
fn parse_switch_01() {
    let src = r#"switch 1+2 {}"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch01",
                    23,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch01",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitch01",
            23,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitch02 (case 24)
#[test]
fn parse_switch_02() {
    let src = r#"switch (a: 0) {}"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch02",
                    24,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch02",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitch02",
            24,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitch03 (case 25)
#[test]
fn parse_switch_03() {
    let src = r#"switch (a: 0, b: 4) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch03",
                    25,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch03",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitch03",
            25,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitch04 (case 26)
#[test]
fn parse_switch_04() {
    let src = r#"switch (1) + (2) {}"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch04",
                    26,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitch04",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitch04",
            26,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseCreateNullableTuple_01 (case 27)
#[test]
fn parse_create_nullable_tuple_01() {
    let src = r#"_ = new (int, int)? {};"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCreateNullableTuple_01",
                    27,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCreateNullableTuple_01",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseCreateNullableTuple_01",
            27,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseCreateNullableTuple_02 (case 28)
#[test]
fn parse_create_nullable_tuple_02() {
    let src = r#"_ = new (int, int) ? (x) : (y);"#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCreateNullableTuple_02",
                    28,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCreateNullableTuple_02",
                    28,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseCreateNullableTuple_02",
            28,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParsePointerToArray (case 29)
#[test]
fn parse_pointer_to_array() {
    let src = r#"int []* p;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePointerToArray",
                    29,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePointerToArray",
                    29,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParsePointerToArray",
            29,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParsePointerToNullableType (case 30)
#[test]
fn parse_pointer_to_nullable_type() {
    let src = r#"int?* p;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePointerToNullableType",
                    30,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParsePointerToNullableType",
                    30,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParsePointerToNullableType",
            30,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseNewNullableWithInitializer (case 31)
#[test]
fn parse_new_nullable_with_initializer() {
    let src = r#"_ = new int? {};"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseNewNullableWithInitializer",
                    31,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseNewNullableWithInitializer",
                    31,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseNewNullableWithInitializer",
            31,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseCaseWithoutSwitch (case 32)
#[test]
fn parse_case_without_switch() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        case int when SomeTest():
                            Console.WriteLine("answer");
                            break;
                        }
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCaseWithoutSwitch",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseCaseWithoutSwitch",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseCaseWithoutSwitch",
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

/// Roslyn: StatementParsingTests.ParseErrantStatementInCase1 (case 33)
#[test]
fn parse_errant_statement_in_case_1() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        switch (expr)
                        {
                            int i;

                            case int when SomeTest():
                                Console.WriteLine("answer");
                                break;
                        }
                    }
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseErrantStatementInCase1",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseErrantStatementInCase1",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseErrantStatementInCase1",
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

/// Roslyn: StatementParsingTests.ParseErrantStatementInCase2 (case 34)
#[test]
fn parse_errant_statement_in_case_2() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        switch (new object())
                        {
                            bool SomeTest() => o is 42;

                            case int when SomeTest():
                                Console.WriteLine("answer");
                                break;
                        }
                    }
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseErrantStatementInCase2",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseErrantStatementInCase2",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseErrantStatementInCase2",
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

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedRecursivePattern1 (case 35)
#[test]
fn parse_switch_statement_with_unclosed_recursive_pattern_1() {
    let src = r#"
                switch (obj)
                {
                    case Type { Prop: Type { }:
                    case Type { Prop: Type { }:
                       break;
                }
                "#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern1",
                    35,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern1",
                    35,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedRecursivePattern1",
            35,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedRecursivePattern2 (case 36)
#[test]
fn parse_switch_statement_with_unclosed_recursive_pattern_2() {
    let src = r#"
                switch (obj)
                {
                    case Type { Prop: Type {:
                    case Type { Prop: Type {:
                       break;
                }
                "#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern2",
                    36,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern2",
                    36,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedRecursivePattern2",
            36,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedRecursivePattern3 (case 37)
#[test]
fn parse_switch_statement_with_unclosed_recursive_pattern_3() {
    let src = r#"
                switch (obj)
                {
                    case { Prop: { Prop: {:
                    case { Prop: { Prop: {:
                       break;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern3",
                    37,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedRecursivePattern3",
                    37,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedRecursivePattern3",
            37,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedListPattern1 (case 38)
#[test]
fn parse_switch_statement_with_unclosed_list_pattern_1() {
    let src = r#"
                switch (obj)
                {
                    case [:
                    case [:
                       break;
                }
                "#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern1",
                    38,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern1",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedListPattern1",
            38,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedListPattern2 (case 39)
#[test]
fn parse_switch_statement_with_unclosed_list_pattern_2() {
    let src = r#"
                switch (obj)
                {
                    case [[:
                    case [[:
                       break;
                }
                "#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern2",
                    39,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern2",
                    39,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedListPattern2",
            39,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedListPattern3 (case 40)
#[test]
fn parse_switch_statement_with_unclosed_list_pattern_3() {
    let src = r#"
                switch (obj)
                {
                    case [[[:
                    case [[[:
                       break;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern3",
                    40,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedListPattern3",
                    40,
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedListPattern3",
            40,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.ParseSwitchStatementWithUnclosedPatternAndArrow (case 41)
#[test]
fn parse_switch_statement_with_unclosed_pattern_and_arrow() {
    let src = r#"
                switch (obj)
                {
                    case { =>
                        break;
                    case { =>
                        break;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedPatternAndArrow",
                    41,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "ParseSwitchStatementWithUnclosedPatternAndArrow",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "ParseSwitchStatementWithUnclosedPatternAndArrow",
            41,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern1 (case 42)
#[test]
fn switch_statement_with_nullable_type_in_pattern_1() {
    let src = r#"
                switch (obj)
                {
                    case Type?:
                        break;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern1",
                    42,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern1",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestSwitchStatementWithNullableTypeInPattern1",
            42,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern2 (case 43)
#[test]
fn switch_statement_with_nullable_type_in_pattern_2() {
    let src = r#"
                switch (obj)
                {
                    case Type? varName:
                        break;
                }
                "#;
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern2",
                    43,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern2",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestSwitchStatementWithNullableTypeInPattern2",
            43,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern3 (case 44)
#[test]
fn switch_statement_with_nullable_type_in_pattern_3() {
    let src = r#"
                switch (obj)
                {
                    case Type? when x > 0:
                        break;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern3",
                    44,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern3",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestSwitchStatementWithNullableTypeInPattern3",
            44,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern4 (case 45)
#[test]
fn switch_statement_with_nullable_type_in_pattern_4() {
    let src = r#"
                switch (obj)
                {
                    case Type? varName when x > 0:
                        break;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern4",
                    45,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern4",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestSwitchStatementWithNullableTypeInPattern4",
            45,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern5 (case 46)
#[test]
fn switch_statement_with_nullable_type_in_pattern_5() {
    let src = r#"
                switch (obj)
                {
                    case (Type? when) when x > 0:
                        break;
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern5",
                    46,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestSwitchStatementWithNullableTypeInPattern5",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestSwitchStatementWithNullableTypeInPattern5",
            46,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: StatementParsingTests.TestYieldReturnTokensAfterPattern (case 47)
#[test]
fn yield_return_tokens_after_pattern() {
    let src = r#"
                void M()
                {
                    var res = x is X? yield
                    return res;
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
                    var res = x is X? yield
                    return res;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestYieldReturnTokensAfterPattern",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestYieldReturnTokensAfterPattern",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestYieldReturnTokensAfterPattern",
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

/// Roslyn: StatementParsingTests.TestYieldBreakTokensAfterPattern (case 48)
#[test]
fn yield_break_tokens_after_pattern() {
    let src = r#"
                void M()
                {
                    var res = x is X? yield
                    break;
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
                    var res = x is X? yield
                    break;
                }
                 }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestYieldBreakTokensAfterPattern",
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
                    "statement_parsing_tests",
                    "StatementParsingTests",
                    "TestYieldBreakTokensAfterPattern",
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
            "statement_parsing_tests",
            "StatementParsingTests",
            "TestYieldBreakTokensAfterPattern",
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
