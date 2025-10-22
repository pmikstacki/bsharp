// Auto-generated from Roslyn: ForStatementParsingTest
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
/// Roslyn: ForStatementParsingTest.TestCommaSeparators1 (case 1)
#[test]
fn comma_separators_1() {
    let src = r#"for (int i = 0, j = 0; i < 10; i++) ;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators1",
                    1,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators1",
            1,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators2 (case 2)
#[test]
fn comma_separators_2() {
    let src = r#"for (int i = 0, i < 10; i++) ;"#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators2",
                    2,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators2",
            2,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators3 (case 3)
#[test]
fn comma_separators_3() {
    let src = r#"for (int i = 0, i < 10, i++) ;"#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators3",
                    3,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators3",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators3",
            3,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators4 (case 4)
#[test]
fn comma_separators_4() {
    let src = r#"for (int i = 0, i) ;"#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators4",
                    4,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators4",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators4",
            4,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators5 (case 5)
#[test]
fn comma_separators_5() {
    let src = r#"for (int i = 0,,) ;"#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators5",
                    5,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators5",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators5",
            5,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators6 (case 6)
#[test]
fn comma_separators_6() {
    let src = r#"for (int i = 0, j; i < 10; i++) ;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators6",
                    6,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestCommaSeparators6",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestCommaSeparators6",
            6,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariableDeclaratorVersusCondition1 (case 7)
#[test]
fn variable_declarator_versus_condition_1() {
    let src = r#"for (int i = 0, i++; i < 10; i++) ;"#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariableDeclaratorVersusCondition1",
                    7,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariableDeclaratorVersusCondition1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariableDeclaratorVersusCondition1",
            7,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers1 (case 8)
#[test]
fn multiple_declarators_with_initializers_1() {
    let src = r#"
            for (int offset = 0, c1, c2; offset < length;)
            {
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers1",
                    8,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers1",
            8,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers2 (case 9)
#[test]
fn multiple_declarators_with_initializers_2() {
    let src = r#"
            for (int offset = 0, c1 = 1, c2; offset < length;)
            {
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers2",
                    9,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers2",
            9,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers3 (case 10)
#[test]
fn multiple_declarators_with_initializers_3() {
    let src = r#"
            for (int offset = 0, c1, c2 = 1; offset < length;)
            {
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers3",
                    10,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers3",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers3",
            10,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers4 (case 11)
#[test]
fn multiple_declarators_with_initializers_4() {
    let src = r#"
            for (int offset = 0, c1,; offset < length;)
            {
            }
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers4",
                    11,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers4",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers4",
            11,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers5 (case 12)
#[test]
fn multiple_declarators_with_initializers_5() {
    let src = r#"
            for (int offset = 0, c1, c2,; offset < length;)
            {
            }
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers5",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers5",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers5",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers6 (case 13)
#[test]
fn multiple_declarators_with_initializers_6() {
    let src = r#"
            for (int offset = 0, c1 = ,; offset < length;)
            {
            }
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers6",
                    13,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers6",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers6",
            13,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers7 (case 14)
#[test]
fn multiple_declarators_with_initializers_7() {
    let src = r#"
            for (int offset = 0, c1 = , c2; offset < length;)
            {
            }
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers7",
                    14,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithInitializers7",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithInitializers7",
            14,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithExpression1 (case 15)
#[test]
fn multiple_declarators_with_expression_1() {
    let src = r#"
            for (Console.WriteLine("Blah"); true;)
            {
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithExpression1",
                    15,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestMultipleDeclaratorsWithExpression1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestMultipleDeclaratorsWithExpression1",
            15,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer1 (case 16)
#[test]
fn incomplete_initializer_1() {
    let src = r#"
            for (MyType m = new() { A = 1,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer1",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer1",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer2 (case 17)
#[test]
fn incomplete_initializer_2() {
    let src = r#"
            for (MyType m = new() { A = 1, B; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer2",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer2",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer3 (case 18)
#[test]
fn incomplete_initializer_3() {
    let src = r#"
            for (MyType m = new() { A = 1, B, ; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer3",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer3",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer3",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer4 (case 19)
#[test]
fn incomplete_initializer_4() {
    let src = r#"
            for (MyType m = new() { A = 1, B = ; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer4",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer4",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer4",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer5 (case 20)
#[test]
fn incomplete_initializer_5() {
    let src = r#"
            for (MyType m = new() { A = 1, B = ,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 6,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer5",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer5",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer5",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteInitializer6 (case 21)
#[test]
fn incomplete_initializer_6() {
    let src = r#"
            for (MyType m = new() { A = 1, B = 1,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 5,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer6",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteInitializer6",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteInitializer6",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith1 (case 22)
#[test]
fn incomplete_with_1() {
    let src = r#"
            for (MyType m = x with { A = 1,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith1",
                    22,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith1",
            22,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith2 (case 23)
#[test]
fn incomplete_with_2() {
    let src = r#"
            for (MyType m = x with { A = 1, B; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith2",
                    23,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith2",
            23,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith3 (case 24)
#[test]
fn incomplete_with_3() {
    let src = r#"
            for (MyType m = x with { A = 1, B, ; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith3",
                    24,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith3",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith3",
            24,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith4 (case 25)
#[test]
fn incomplete_with_4() {
    let src = r#"
            for (MyType m = x with { A = 1, B = ; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith4",
                    25,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith4",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith4",
            25,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith5 (case 26)
#[test]
fn incomplete_with_5() {
    let src = r#"
            for (MyType m = x with { A = 1, B = ,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith5",
                    26,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith5",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith5",
            26,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestIncompleteWith6 (case 27)
#[test]
fn incomplete_with_6() {
    let src = r#"
            for (MyType m = x with { A = 1, B = 1,; true; m++)
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith6",
                    27,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestIncompleteWith6",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestIncompleteWith6",
            27,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_AnonymousFunction (case 28)
#[test]
fn various_expressions_anonymous_function() {
    let src = r#"
            for (delegate() {};delegate() {};delegate() {});
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_AnonymousFunction",
                    28,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_AnonymousFunction",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_AnonymousFunction",
            28,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_AnonymousObjectCreation (case 29)
#[test]
fn various_expressions_anonymous_object_creation() {
    let src = r#"
            for (new();new();new());
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_AnonymousObjectCreation",
                    29,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_AnonymousObjectCreation",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_AnonymousObjectCreation",
            29,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ArrayCreation (case 30)
#[test]
fn various_expressions_array_creation() {
    let src = r#"
            for (new int[] { };new int[] { };new int[] { });
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ArrayCreation",
                    30,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ArrayCreation",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ArrayCreation",
            30,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Assignment1 (case 31)
#[test]
fn various_expressions_assignment_1() {
    let src = r#"
            for (a=1;a=1;a=1);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Assignment1",
                    31,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Assignment1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Assignment1",
            31,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Assignment2 (case 32)
#[test]
fn various_expressions_assignment_2() {
    let src = r#"
            for (a+=1;a+=1;a+=1);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Assignment2",
                    32,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Assignment2",
                    32,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Assignment2",
            32,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Cast (case 33)
#[test]
fn various_expressions_cast() {
    let src = r#"
            for ((int)0;(int)0;(int)0);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Cast",
                    33,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Cast",
                    33,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Cast",
            33,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Checked (case 34)
#[test]
fn various_expressions_checked() {
    let src = r#"
            for (checked(0);checked(0);checked(0));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Checked",
                    34,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Checked",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Checked",
            34,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Collection (case 35)
#[test]
fn various_expressions_collection() {
    let src = r#"
            for ([];[];[]);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Collection",
                    35,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Collection",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Collection",
            35,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ConditionalAccess (case 36)
#[test]
fn various_expressions_conditional_access() {
    let src = r#"
            for (a?.b;a?.b;a?.b);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ConditionalAccess",
                    36,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ConditionalAccess",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ConditionalAccess",
            36,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_DefaultExpression1 (case 37)
#[test]
fn various_expressions_default_expression_1() {
    let src = r#"
            for (default(int);default(int);default(int));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_DefaultExpression1",
                    37,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_DefaultExpression1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_DefaultExpression1",
            37,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_DefaultExpression2 (case 38)
#[test]
fn various_expressions_default_expression_2() {
    let src = r#"
            for (default;default;default);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_DefaultExpression2",
                    38,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_DefaultExpression2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_DefaultExpression2",
            38,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ElementAccess (case 39)
#[test]
fn various_expressions_element_access() {
    let src = r#"
            for (a[0];a[0];a[0]);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ElementAccess",
                    39,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ElementAccess",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ElementAccess",
            39,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ImplicitArrayCreation (case 40)
#[test]
fn various_expressions_implicit_array_creation() {
    let src = r#"
            for (new[]{};new[]{};new[]{});
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ImplicitArrayCreation",
                    40,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ImplicitArrayCreation",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ImplicitArrayCreation",
            40,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_InterpolatedString (case 41)
#[test]
fn various_expressions_interpolated_string() {
    let src = r#"
            for ($"";$"";$"");
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_InterpolatedString",
                    41,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_InterpolatedString",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_InterpolatedString",
            41,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Invocation (case 42)
#[test]
fn various_expressions_invocation() {
    let src = r#"
            for (a();a();a());
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Invocation",
                    42,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Invocation",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Invocation",
            42,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_IsPattern (case 43)
#[test]
fn various_expressions_is_pattern() {
    let src = r#"
            for (a is B b;a is B b;a is B b);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_IsPattern",
                    43,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_IsPattern",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_IsPattern",
            43,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Literal (case 44)
#[test]
fn various_expressions_literal() {
    let src = r#"
            for (true;true;true);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Literal",
                    44,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Literal",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Literal",
            44,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_MemberAccess (case 45)
#[test]
fn various_expressions_member_access() {
    let src = r#"
            for (a.b;a.b;a.b);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_MemberAccess",
                    45,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_MemberAccess",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_MemberAccess",
            45,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Parenthesized (case 46)
#[test]
fn various_expressions_parenthesized() {
    let src = r#"
            for ((a);(a);(a));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Parenthesized",
                    46,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Parenthesized",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Parenthesized",
            46,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Postfix (case 47)
#[test]
fn various_expressions_postfix() {
    let src = r#"
            for (a++;a++;a++);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Postfix",
                    47,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Postfix",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Postfix",
            47,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation1 (case 48)
#[test]
fn various_expressions_object_creation_1() {
    let src = r#"
            for (new A();new A();new A());
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation1",
                    48,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation1",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ObjectCreation1",
            48,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation2 (case 49)
#[test]
fn various_expressions_object_creation_2() {
    let src = r#"
            for (new A() { };new A() { };new A() { });
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation2",
                    49,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation2",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ObjectCreation2",
            49,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation3 (case 50)
#[test]
fn various_expressions_object_creation_3() {
    let src = r#"
            for (new A { };new A { };new A { });
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation3",
                    50,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_ObjectCreation3",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_ObjectCreation3",
            50,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Prefix (case 51)
#[test]
fn various_expressions_prefix() {
    let src = r#"
            for (++a;++a;++a);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Prefix",
                    51,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Prefix",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Prefix",
            51,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Query (case 52)
#[test]
fn various_expressions_query() {
    let src = r#"
            for (from a in b select c;from a in b select c;from a in b select c);
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Query",
                    52,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Query",
                    52,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Query",
            52,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range1 (case 53)
#[test]
fn various_expressions_range_1() {
    let src = r#"
            for (..;..;..);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range1",
                    53,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range1",
                    53,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Range1",
            53,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range2 (case 54)
#[test]
fn various_expressions_range_2() {
    let src = r#"
            for (a..;a..;a..);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range2",
                    54,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range2",
                    54,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Range2",
            54,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range3 (case 55)
#[test]
fn various_expressions_range_3() {
    let src = r#"
            for (..a;..a;..a);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range3",
                    55,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range3",
                    55,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Range3",
            55,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range4 (case 56)
#[test]
fn various_expressions_range_4() {
    let src = r#"
            for (a..a;a..a;a..a);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range4",
                    56,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Range4",
                    56,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Range4",
            56,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Ref1 (case 57)
#[test]
fn various_expressions_ref_1() {
    let src = r#"
            for (ref a; ref a; ref a);
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Ref1",
                    57,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Ref1",
                    57,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Ref1",
            57,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Ref2 (case 58)
#[test]
fn various_expressions_ref_2() {
    let src = r#"
            for (ref int a; ref a; ref a);
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Ref2",
                    58,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Ref2",
                    58,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Ref2",
            58,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Sizeof (case 59)
#[test]
fn various_expressions_sizeof() {
    let src = r#"
            for (sizeof(a);sizeof(a);sizeof(a));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Sizeof",
                    59,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Sizeof",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Sizeof",
            59,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Switch (case 60)
#[test]
fn various_expressions_switch() {
    let src = r#"
            for (a switch {};a switch {};a switch {});
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Switch",
                    60,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Switch",
                    60,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Switch",
            60,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Throw (case 61)
#[test]
fn various_expressions_throw() {
    let src = r#"
            for (throw a;throw a;throw a);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Throw",
                    61,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Throw",
                    61,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Throw",
            61,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Tuple (case 62)
#[test]
fn various_expressions_tuple() {
    let src = r#"
            for ((a, b);(a, b);(a, b));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Tuple",
                    62,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Tuple",
                    62,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Tuple",
            62,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Typeof (case 63)
#[test]
fn various_expressions_typeof() {
    let src = r#"
            for (typeof(int);typeof(int);typeof(int));
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Typeof",
                    63,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_Typeof",
                    63,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_Typeof",
            63,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_With1 (case 64)
#[test]
fn various_expressions_with_1() {
    let src = r#"
            for (a with { }; a with { }; a with { })
            {
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_With1",
                    64,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_With1",
                    64,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_With1",
            64,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_With2 (case 65)
#[test]
fn various_expressions_with_2() {
    let src = r#"
            for (; a with { }; a with { })
            {
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_With2",
                    65,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestVariousExpressions_With2",
                    65,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestVariousExpressions_With2",
            65,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer1 (case 66)
#[test]
fn complex_initializer_1() {
    let src = r#"
            for (;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer1",
                    66,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer1",
                    66,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer1",
            66,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer2 (case 67)
#[test]
fn complex_initializer_2() {
    let src = r#"
            for (int i;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer2",
                    67,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer2",
                    67,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer2",
            67,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer3 (case 68)
#[test]
fn complex_initializer_3() {
    let src = r#"
            for (int i, j, k;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer3",
                    68,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer3",
                    68,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer3",
            68,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer4 (case 69)
#[test]
fn complex_initializer_4() {
    let src = r#"
            for (int i = 0;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer4",
                    69,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer4",
                    69,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer4",
            69,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer5 (case 70)
#[test]
fn complex_initializer_5() {
    let src = r#"
            for (A b;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer5",
                    70,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer5",
                    70,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer5",
            70,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer6 (case 71)
#[test]
fn complex_initializer_6() {
    let src = r#"
            for (A b, c, d;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer6",
                    71,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer6",
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer6",
            71,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer7 (case 72)
#[test]
fn complex_initializer_7() {
    let src = r#"
            for (A b = null, c, d = null;;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer7",
                    72,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer7",
                    72,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer7",
            72,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer8 (case 73)
#[test]
fn complex_initializer_8() {
    let src = r#"
            for (A b = c switch { A => x, _ => y };;);
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer8",
                    73,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer8",
                    73,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer8",
            73,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer9 (case 74)
#[test]
fn complex_initializer_9() {
    let src = r#"
            for (int i =;;);
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer9",
                    74,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer9",
                    74,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer9",
            74,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer10 (case 75)
#[test]
fn complex_initializer_10() {
    let src = r#"
            for (int i = 0, j =;;);
            "#;
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
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer10",
                    75,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "for_statement_parsing_test",
                    "ForStatementParsingTest",
                    "TestComplexInitializer10",
                    75,
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
            "for_statement_parsing_test",
            "ForStatementParsingTest",
            "TestComplexInitializer10",
            75,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
