// Auto-generated from Roslyn: ValueTupleTests
/// Roslyn: ValueTupleTests.SimpleTuple (case 1)
#[test]
fn simple_tuple() {
    let src = r#"
class C
{
    (int, string) Goo()
    {
        return (1, ""Alice"");
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "SimpleTuple",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "SimpleTuple",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "SimpleTuple",
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

/// Roslyn: ValueTupleTests.LongTuple (case 2)
#[test]
fn long_tuple() {
    let src = r#"
class C
{
    (int, int, int, string, string, string, int, int, int) Goo()
    {
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "LongTuple",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "LongTuple",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "LongTuple",
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

/// Roslyn: ValueTupleTests.TuplesInLambda (case 3)
#[test]
fn tuples_in_lambda() {
    let src = r#"
class C
{
    var x = ((string, string) a, (int, int) b) => { };
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TuplesInLambda",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TuplesInLambda",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TuplesInLambda",
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

/// Roslyn: ValueTupleTests.TuplesWithNamesInLambda (case 4)
#[test]
fn tuples_with_names_in_lambda() {
    let src = r#"
class C
{
    var x = ((string a, string) a, (int, int b) b) => { };
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TuplesWithNamesInLambda",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TuplesWithNamesInLambda",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TuplesWithNamesInLambda",
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

/// Roslyn: ValueTupleTests.TupleInParameters (case 5)
#[test]
fn tuple_in_parameters() {
    let src = r#"
class C
{
    void Goo((int, string) a)
    {
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleInParameters",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleInParameters",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TupleInParameters",
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

/// Roslyn: ValueTupleTests.TupleTypeWithTooFewElements (case 6)
#[test]
fn tuple_type_with_too_few_elements() {
    let src = r#"
class C
{
    void M(int x, () y, (int a) z) { }
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleTypeWithTooFewElements",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleTypeWithTooFewElements",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TupleTypeWithTooFewElements",
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

/// Roslyn: ValueTupleTests.TupleExpressionWithTooFewElements (case 7)
#[test]
fn tuple_expression_with_too_few_elements() {
    let src = r#"
class C
{
    object x = ((Alice: 1), ());
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleExpressionWithTooFewElements",
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
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TupleExpressionWithTooFewElements",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TupleExpressionWithTooFewElements",
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

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_01 (case 8)
#[test]
fn ternary_versus_declaration_01() {
    let src = r#"return (i, isValid ? Errors.IsValid : Errors.HasErrors);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_01",
                    8,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_01",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TernaryVersusDeclaration_01",
            8,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_02 (case 9)
#[test]
fn ternary_versus_declaration_02() {
    let src = r#"return (isValid ? Errors.IsValid : Errors.HasErrors, i);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_02",
                    9,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_02",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TernaryVersusDeclaration_02",
            9,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_03 (case 10)
#[test]
fn ternary_versus_declaration_03() {
    let src = r#"return (i, a < b, c > d);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_03",
                    10,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_03",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TernaryVersusDeclaration_03",
            10,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_04 (case 11)
#[test]
fn ternary_versus_declaration_04() {
    let src = r#"return (i, a < b, c > d.x);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_04",
                    11,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_04",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TernaryVersusDeclaration_04",
            11,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_05 (case 12)
#[test]
fn ternary_versus_declaration_05() {
    let src = r#"return (i, a < b, c > d && x);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_05",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "value_tuple_tests",
                    "ValueTupleTests",
                    "TernaryVersusDeclaration_05",
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
            "value_tuple_tests",
            "ValueTupleTests",
            "TernaryVersusDeclaration_05",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
