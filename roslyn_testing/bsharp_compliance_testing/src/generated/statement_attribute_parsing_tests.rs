// Auto-generated from Roslyn: StatementAttributeParsingTests
/// Roslyn: StatementAttributeParsingTests.AttributeOnBlock (case 1)
#[test]
fn attribute_on_block() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]{}
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnEmptyStatement (case 2)
#[test]
fn attribute_on_empty_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A];
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnEmptyStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnEmptyStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnEmptyStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnLabeledStatement (case 3)
#[test]
fn attribute_on_labeled_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]
        bar:
            Goo();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLabeledStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLabeledStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLabeledStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnGotoStatement (case 4)
#[test]
fn attribute_on_goto_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]
        goto bar;
        bar:
            Goo();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnGotoStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnGotoStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnGotoStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnBreakStatement (case 5)
#[test]
fn attribute_on_break_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        while (true)
        {
            [A]
            break;
        }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnBreakStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnBreakStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnBreakStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnContinueStatement (case 6)
#[test]
fn attribute_on_continue_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        while (true)
        {
            [A]
            continue;
        }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnContinueStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnContinueStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnContinueStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnReturn (case 7)
#[test]
fn attribute_on_return() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]return;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnReturn",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnReturn",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnReturn",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnThrow (case 8)
#[test]
fn attribute_on_throw() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]throw;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnThrow",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnThrow",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnThrow",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnYieldReturn (case 9)
#[test]
fn attribute_on_yield_return() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]yield return 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnYieldReturn",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnYieldReturn",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnYieldReturn",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnYieldBreak (case 10)
#[test]
fn attribute_on_yield_break() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]yield return 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnYieldBreak",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnYieldBreak",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnYieldBreak",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnNakedYield (case 11)
#[test]
fn attribute_on_naked_yield() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]yield
    }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnNakedYield",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnNakedYield",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnNakedYield",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnWhileStatement (case 12)
#[test]
fn attribute_on_while_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]while (true);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnWhileStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnWhileStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnWhileStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnDoStatement (case 13)
#[test]
fn attribute_on_do_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]do { } while (true);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnDoStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnDoStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnDoStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnForStatement (case 14)
#[test]
fn attribute_on_for_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]for (;;) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnForStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnForStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnForStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnNormalForEachStatement (case 15)
#[test]
fn attribute_on_normal_for_each_statement() {
    let src = r#"
class C
{
    void Goo(string[] vals)
    {
        [A]foreach (var v in vals) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnNormalForEachStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnNormalForEachStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnNormalForEachStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnForEachVariableStatement (case 16)
#[test]
fn attribute_on_for_each_variable_statement() {
    let src = r#"
class C
{
    void Goo((int, string)[] vals)
    {
        [A]foreach (var (i, s) in vals) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnForEachVariableStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnForEachVariableStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnForEachVariableStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnUsingStatement (case 17)
#[test]
fn attribute_on_using_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]using (null) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUsingStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUsingStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnUsingStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnAwaitUsingStatement1 (case 18)
#[test]
fn attribute_on_await_using_statement_1() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]await using (null) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnAwaitUsingStatement1",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnAwaitUsingStatement1",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnAwaitUsingStatement1",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnAwaitUsingStatement2 (case 19)
#[test]
fn attribute_on_await_using_statement_2() {
    let src = r#"
class C
{
    async void Goo()
    {
        [A]await using (null) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnAwaitUsingStatement2",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnAwaitUsingStatement2",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnAwaitUsingStatement2",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnFixedStatement (case 20)
#[test]
fn attribute_on_fixed_statement() {
    let src = r#"
class C
{
    unsafe void Goo(int[] vals)
    {
        [A]fixed (int* p = vals) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFixedStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFixedStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnFixedStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnCheckedStatement (case 21)
#[test]
fn attribute_on_checked_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]checked { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCheckedStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCheckedStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnCheckedStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnCheckedBlock (case 22)
#[test]
fn attribute_on_checked_block() {
    let src = r#"
class C
{
    void Goo()
    {
        checked [A]{ }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCheckedBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCheckedBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnCheckedBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnUncheckedStatement (case 23)
#[test]
fn attribute_on_unchecked_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]unchecked { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUncheckedStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUncheckedStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnUncheckedStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnUnsafeStatement (case 24)
#[test]
fn attribute_on_unsafe_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]unsafe { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUnsafeStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUnsafeStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnUnsafeStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnUnsafeBlock (case 25)
#[test]
fn attribute_on_unsafe_block() {
    let src = r#"
class C
{
    void Goo()
    {
        unsafe [A]{ }
    }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUnsafeBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnUnsafeBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnUnsafeBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnLockStatement (case 26)
#[test]
fn attribute_on_lock_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]lock (null) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLockStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLockStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLockStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnIfStatement (case 27)
#[test]
fn attribute_on_if_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]if (true) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnIfStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnIfStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnIfStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnSwitchStatement (case 28)
#[test]
fn attribute_on_switch_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]switch (0) { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnSwitchStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnSwitchStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnSwitchStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnStatementInSwitchSection (case 29)
#[test]
fn attribute_on_statement_in_switch_section() {
    let src = r#"
class C
{
    void Goo()
    {
        switch (0)
        {
            default:
                [A]return;
        }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementInSwitchSection",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementInSwitchSection",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnStatementInSwitchSection",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnStatementAboveCase (case 30)
#[test]
fn attribute_on_statement_above_case() {
    let src = r#"
class C
{
    void Goo()
    {
        switch (0)
        {
            [A]
            case 0:
                return;
        }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementAboveCase",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementAboveCase",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnStatementAboveCase",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnStatementAboveDefaultCase (case 31)
#[test]
fn attribute_on_statement_above_default_case() {
    let src = r#"
class C
{
    void Goo()
    {
        switch (0)
        {
            [A]
            default:
                return;
        }
    }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementAboveDefaultCase",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnStatementAboveDefaultCase",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnStatementAboveDefaultCase",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnTryStatement (case 32)
#[test]
fn attribute_on_try_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]try { } finally { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnTryStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnTryStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnTryStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnTryBlock (case 33)
#[test]
fn attribute_on_try_block() {
    let src = r#"
class C
{
    void Goo()
    {
        try [A] { } finally { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnTryBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnTryBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnTryBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnFinally (case 34)
#[test]
fn attribute_on_finally() {
    let src = r#"
class C
{
    void Goo()
    {
        try { } [A] finally { }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFinally",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFinally",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnFinally",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnFinallyBlock (case 35)
#[test]
fn attribute_on_finally_block() {
    let src = r#"
class C
{
    void Goo()
    {
        try { } finally [A] { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFinallyBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnFinallyBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnFinallyBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnCatch (case 36)
#[test]
fn attribute_on_catch() {
    let src = r#"
class C
{
    void Goo()
    {
        try { } [A] catch { }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCatch",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCatch",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnCatch",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnCatchBlock (case 37)
#[test]
fn attribute_on_catch_block() {
    let src = r#"
class C
{
    void Goo()
    {
        try { } catch [A] { }
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCatchBlock",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnCatchBlock",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnCatchBlock",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnEmbeddedStatement (case 38)
#[test]
fn attribute_on_embedded_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        if (true) [A]return;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnEmbeddedStatement",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnEmbeddedStatement",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnEmbeddedStatement",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AnonymousMethod_NoParameters (case 39)
#[test]
fn attribute_on_expression_statement_anonymous_method_no_parameters() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]delegate { }
    }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_NoParameters",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_NoParameters",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AnonymousMethod_NoParameters",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AnonymousMethod_NoBody (case 40)
#[test]
fn attribute_on_expression_statement_anonymous_method_no_body() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]delegate
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_NoBody",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_NoBody",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AnonymousMethod_NoBody",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AnonymousMethod_Parameters (case 41)
#[test]
fn attribute_on_expression_statement_anonymous_method_parameters() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]delegate () { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_Parameters",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousMethod_Parameters",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AnonymousMethod_Parameters",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Lambda_NoParameters (case 42)
#[test]
fn attribute_on_expression_statement_lambda_no_parameters() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]() => { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_NoParameters",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_NoParameters",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Lambda_NoParameters",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Lambda_Parameters1 (case 43)
#[test]
fn attribute_on_expression_statement_lambda_parameters_1() {
    let src = r#"
class C
{
    void Goo()
    {
        [A](int i) => { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_Parameters1",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_Parameters1",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Lambda_Parameters1",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Lambda_Parameters2 (case 44)
#[test]
fn attribute_on_expression_statement_lambda_parameters_2() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]i => { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_Parameters2",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Lambda_Parameters2",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Lambda_Parameters2",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AnonymousObject (case 45)
#[test]
fn attribute_on_expression_statement_anonymous_object() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]new { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousObject",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousObject",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AnonymousObject",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ArrayCreation (case 46)
#[test]
fn attribute_on_expression_statement_array_creation() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]new int[] { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ArrayCreation",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ArrayCreation",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ArrayCreation",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AnonymousArrayCreation (case 47)
#[test]
fn attribute_on_expression_statement_anonymous_array_creation() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]new [] { 0 };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousArrayCreation",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AnonymousArrayCreation",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AnonymousArrayCreation",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Assignment (case 48)
#[test]
fn attribute_on_expression_statement_assignment() {
    let src = r#"
class C
{
    void Goo(int a)
    {
        [A]a = 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Assignment",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Assignment",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Assignment",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_CompoundAssignment (case 49)
#[test]
fn attribute_on_expression_statement_compound_assignment() {
    let src = r#"
class C
{
    void Goo(int a)
    {
        [A]a += 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_CompoundAssignment",
                    49,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_CompoundAssignment",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_CompoundAssignment",
            49,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AwaitExpression_NonAsyncContext (case 50)
#[test]
fn attribute_on_expression_statement_await_expression_non_async_context() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]await a;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AwaitExpression_NonAsyncContext",
                    50,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AwaitExpression_NonAsyncContext",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AwaitExpression_NonAsyncContext",
            50,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_AwaitExpression_AsyncContext (case 51)
#[test]
fn attribute_on_expression_statement_await_expression_async_context() {
    let src = r#"
class C
{
    async void Goo()
    {
        [A]await a;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AwaitExpression_AsyncContext",
                    51,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_AwaitExpression_AsyncContext",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_AwaitExpression_AsyncContext",
            51,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_BinaryExpression (case 52)
#[test]
fn attribute_on_expression_statement_binary_expression() {
    let src = r#"
class C
{
    void Goo(int a)
    {
        [A]a + a;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_BinaryExpression",
                    52,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_BinaryExpression",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_BinaryExpression",
            52,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_CastExpression (case 53)
#[test]
fn attribute_on_expression_statement_cast_expression() {
    let src = r#"
class C
{
    void Goo(int a)
    {
        [A](object)a;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_CastExpression",
                    53,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_CastExpression",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_CastExpression",
            53,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ConditionalAccess (case 54)
#[test]
fn attribute_on_expression_statement_conditional_access() {
    let src = r#"
class C
{
    void Goo(string a)
    {
        [A]a?.ToString();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ConditionalAccess",
                    54,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ConditionalAccess",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ConditionalAccess",
            54,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_DefaultExpression (case 55)
#[test]
fn attribute_on_expression_statement_default_expression() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]default(int);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_DefaultExpression",
                    55,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_DefaultExpression",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_DefaultExpression",
            55,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_DefaultLiteral (case 56)
#[test]
fn attribute_on_expression_statement_default_literal() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]default;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_DefaultLiteral",
                    56,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_DefaultLiteral",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_DefaultLiteral",
            56,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ElementAccess (case 57)
#[test]
fn attribute_on_expression_statement_element_access() {
    let src = r#"
class C
{
    void Goo(string s)
    {
        [A]s[0];
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ElementAccess",
                    57,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ElementAccess",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ElementAccess",
            57,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ElementBinding (case 58)
#[test]
fn attribute_on_expression_statement_element_binding() {
    let src = r#"
class C
{
    void Goo(string s)
    {
        [A]s?[0];
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ElementBinding",
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ElementBinding",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ElementBinding",
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

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Invocation (case 59)
#[test]
fn attribute_on_expression_statement_invocation() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]Goo();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Invocation",
                    59,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Invocation",
                    59,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Invocation",
            59,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Literal (case 60)
#[test]
fn attribute_on_expression_statement_literal() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Literal",
                    60,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Literal",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Literal",
            60,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_MemberAccess (case 61)
#[test]
fn attribute_on_expression_statement_member_access() {
    let src = r#"
class C
{
    void Goo(int i)
    {
        [A]i.ToString;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_MemberAccess",
                    61,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_MemberAccess",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_MemberAccess",
            61,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ObjectCreation_Builtin (case 62)
#[test]
fn attribute_on_expression_statement_object_creation_builtin() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]new int();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ObjectCreation_Builtin",
                    62,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ObjectCreation_Builtin",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ObjectCreation_Builtin",
            62,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_ObjectCreation_TypeName (case 63)
#[test]
fn attribute_on_expression_statement_object_creation_type_name() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]new System.Int32();
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ObjectCreation_TypeName",
                    63,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_ObjectCreation_TypeName",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_ObjectCreation_TypeName",
            63,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Parenthesized (case 64)
#[test]
fn attribute_on_expression_statement_parenthesized() {
    let src = r#"
class C
{
    void Goo()
    {
        [A](1);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Parenthesized",
                    64,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Parenthesized",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Parenthesized",
            64,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_PostfixUnary (case 65)
#[test]
fn attribute_on_expression_statement_postfix_unary() {
    let src = r#"
class C
{
    void Goo(int i)
    {
        [A]i++;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_PostfixUnary",
                    65,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_PostfixUnary",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_PostfixUnary",
            65,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_PrefixUnary (case 66)
#[test]
fn attribute_on_expression_statement_prefix_unary() {
    let src = r#"
                class C
                {
                    void Goo(int i)
                    {
                        [A]++i;
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_PrefixUnary",
                    66,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_PrefixUnary",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_PrefixUnary",
            66,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Query (case 67)
#[test]
fn attribute_on_expression_statement_query() {
    let src = r#"
using System.Linq;
class C
{
    void Goo(string s)
    {
        [A]from c in s select c;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Query",
                    67,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Query",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Query",
            67,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range1 (case 68)
#[test]
fn attribute_on_expression_statement_range_1() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]a..b;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range1",
                    68,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range1",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Range1",
            68,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range2 (case 69)
#[test]
fn attribute_on_expression_statement_range_2() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]a..;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range2",
                    69,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range2",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Range2",
            69,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range3 (case 70)
#[test]
fn attribute_on_expression_statement_range_3() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]..b;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range3",
                    70,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range3",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Range3",
            70,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range4 (case 71)
#[test]
fn attribute_on_expression_statement_range_4() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]..;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range4",
                    71,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Range4",
                    71,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Range4",
            71,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Sizeof (case 72)
#[test]
fn attribute_on_expression_statement_sizeof() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]sizeof(int);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Sizeof",
                    72,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_Sizeof",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_Sizeof",
            72,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_SwitchExpression (case 73)
#[test]
fn attribute_on_expression_statement_switch_expression() {
    let src = r#"
class C
{
    void Goo(int a)
    {
        [A]a switch { };
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_SwitchExpression",
                    73,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_SwitchExpression",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_SwitchExpression",
            73,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_TypeOf (case 74)
#[test]
fn attribute_on_expression_statement_type_of() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]typeof(int);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_TypeOf",
                    74,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnExpressionStatement_TypeOf",
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
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnExpressionStatement_TypeOf",
            74,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember1 (case 75)
#[test]
fn attribute_on_local_decl_or_member_1() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]int i;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember1",
                    75,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember1",
                    75,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember1",
            75,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember2 (case 76)
#[test]
fn attribute_on_local_decl_or_member_2() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]int i, j;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember2",
                    76,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember2",
                    76,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember2",
            76,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember3 (case 77)
#[test]
fn attribute_on_local_decl_or_member_3() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]int i = 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember3",
                    77,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember3",
                    77,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember3",
            77,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember4 (case 78)
#[test]
fn attribute_on_local_decl_or_member_4() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]int this[int i] => 0;
    }
}"#;
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember4",
                    78,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember4",
                    78,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember4",
            78,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember5 (case 79)
#[test]
fn attribute_on_local_decl_or_member_5() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]const int i = 0;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember5",
                    79,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember5",
                    79,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember5",
            79,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AccessModOnLocalDeclOrMember_01 (case 80)
#[test]
fn access_mod_on_local_decl_or_member_01() {
    let src = r#"
class C
{
    void Goo()
    {
        public extern int i = 1;
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AccessModOnLocalDeclOrMember_01",
                    80,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AccessModOnLocalDeclOrMember_01",
                    80,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AccessModOnLocalDeclOrMember_01",
            80,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AccessModOnLocalDeclOrMember_02 (case 81)
#[test]
fn access_mod_on_local_decl_or_member_02() {
    let src = r#"
class C
{
    void Goo()
    {
        extern public int i = 1;
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AccessModOnLocalDeclOrMember_02",
                    81,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AccessModOnLocalDeclOrMember_02",
                    81,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AccessModOnLocalDeclOrMember_02",
            81,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember6 (case 82)
#[test]
fn attribute_on_local_decl_or_member_6() {
    let src = r#"
class C
{
    void Goo()
    {
        [A]public int i = 0;
    }
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember6",
                    82,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember6",
                    82,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember6",
            82,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember7 (case 83)
#[test]
fn attribute_on_local_decl_or_member_7() {
    let src = r#"
class C
{
    void Goo(System.IDisposable d)
    {
        [A]using var i = d;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember7",
                    83,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember7",
                    83,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember7",
            83,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember8 (case 84)
#[test]
fn attribute_on_local_decl_or_member_8() {
    let src = r#"
class C
{
    void Goo(System.IAsyncDisposable d)
    {
        [A]await using var i = d;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember8",
                    84,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember8",
                    84,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember8",
            84,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnLocalDeclOrMember9 (case 85)
#[test]
fn attribute_on_local_decl_or_member_9() {
    let src = r#"
class C
{
    async void Goo(System.IAsyncDisposable d)
    {
        [A]await using var i = d;
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember9",
                    85,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttributeOnLocalDeclOrMember9",
                    85,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttributeOnLocalDeclOrMember9",
            85,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: StatementAttributeParsingTests.AttrDeclOnStatementWhereMemberExpected (case 86)
#[test]
fn attr_decl_on_statement_where_member_expected() {
    let src = r#"
class C
{
    [Attr] x.y();
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttrDeclOnStatementWhereMemberExpected",
                    86,
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
                    "statement_attribute_parsing_tests",
                    "StatementAttributeParsingTests",
                    "AttrDeclOnStatementWhereMemberExpected",
                    86,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "statement_attribute_parsing_tests",
            "StatementAttributeParsingTests",
            "AttrDeclOnStatementWhereMemberExpected",
            86,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}
