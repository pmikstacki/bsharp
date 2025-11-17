// Auto-generated from Roslyn: DeclarationExpressionTests
/// Roslyn: DeclarationExpressionTests.NullaboutOutDeclaration (case 1)
#[test]
fn nullabout_out_declaration() {
    let src = r#"M(out int? x);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullaboutOutDeclaration",
                    1,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullaboutOutDeclaration",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullaboutOutDeclaration",
            1,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_01 (case 2)
#[test]
fn nullable_type_test_01() {
    let src = r#"if (e is int?) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_01",
                    2,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_01",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_01",
            2,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_02 (case 3)
#[test]
fn nullable_type_test_02() {
    let src = r#"if (e is int ? true : false) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_02",
                    3,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_02",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_02",
            3,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_03 (case 4)
#[test]
fn nullable_type_test_03() {
    let src = r#"if (e is int? x) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_03",
                    4,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_03",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_03",
            4,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_03_2 (case 5)
#[test]
fn nullable_type_test_03_2() {
    let src = r#"if (e is int ? x : y) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_03_2",
                    5,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_03_2",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_03_2",
            5,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_04 (case 6)
#[test]
fn nullable_type_test_04() {
    let src = r#"if (e is int x ? true : false) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_04",
                    6,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_04",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_04",
            6,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_05 (case 7)
#[test]
fn nullable_type_test_05() {
    let src = r#"ref object x = o1 is string ? ref o2 : ref o3;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_05",
                    7,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_05",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_05",
            7,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_06 (case 8)
#[test]
fn nullable_type_test_06() {
    let src = r#"ref object x = ref o1 is string ? ref o2 : ref o3;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_06",
                    8,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NullableTypeTest_06",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NullableTypeTest_06",
            8,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.UnderscoreInOldForeach_01 (case 9)
#[test]
fn underscore_in_old_foreach_01() {
    let src = r#"foreach (int _ in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "UnderscoreInOldForeach_01",
                    9,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "UnderscoreInOldForeach_01",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "UnderscoreInOldForeach_01",
            9,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.UnderscoreInOldForeach_02 (case 10)
#[test]
fn underscore_in_old_foreach_02() {
    let src = r#"foreach (var _ in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "UnderscoreInOldForeach_02",
                    10,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "UnderscoreInOldForeach_02",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "UnderscoreInOldForeach_02",
            10,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_01 (case 11)
#[test]
fn new_foreach_01() {
    let src = r#"foreach ((var x, var y) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_01",
                    11,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_01",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_01",
            11,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_02 (case 12)
#[test]
fn new_foreach_02() {
    let src = r#"foreach ((int x, int y) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_02",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_02",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_02",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_03 (case 13)
#[test]
fn new_foreach_03() {
    let src = r#"foreach ((int x, int y) v in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_03",
                    13,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_03",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_03",
            13,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_04 (case 14)
#[test]
fn new_foreach_04() {
    let src = r#"foreach ((1, 2) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_04",
                    14,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_04",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_04",
            14,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_05 (case 15)
#[test]
fn new_foreach_05() {
    let src = r#"foreach (var (x, y) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_05",
                    15,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_05",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_05",
            15,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_06 (case 16)
#[test]
fn new_foreach_06() {
    let src = r#"foreach ((int x, var (y, z)) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_06",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_06",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_06",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_07 (case 17)
#[test]
fn new_foreach_07() {
    let src = r#"foreach ((var (x, y), z) in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_07",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_07",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_07",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_08 (case 18)
#[test]
fn new_foreach_08() {
    let src = r#"foreach (x in e) {}"#;
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
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_08",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_08",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_08",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_09 (case 19)
#[test]
fn new_foreach_09() {
    let src = r#"foreach (_ in e) {}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_09",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_09",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_09",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NewForeach_10 (case 20)
#[test]
fn new_foreach_10() {
    let src = r#"foreach (a.b in e) {}"#;
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
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_10",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NewForeach_10",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NewForeach_10",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.TupleOnTheLeft (case 21)
#[test]
fn tuple_on_the_left() {
    let src = r#"(1, 2) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "TupleOnTheLeft",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "TupleOnTheLeft",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "TupleOnTheLeft",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.OutTuple_01 (case 22)
#[test]
fn out_tuple_01() {
    let src = r#"M(out (1, 2));"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_01",
                    22,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_01",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "OutTuple_01",
            22,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.OutTuple_02 (case 23)
#[test]
fn out_tuple_02() {
    let src = r#"M(out (x, y));"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_02",
                    23,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_02",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "OutTuple_02",
            23,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.OutTuple_03 (case 24)
#[test]
fn out_tuple_03() {
    let src = r#"M(out (1, 2).Field);"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_03",
                    24,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_03",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "OutTuple_03",
            24,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.OutTuple_04 (case 25)
#[test]
fn out_tuple_04() {
    let src = r#"M(out (int x, int y));"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_04",
                    25,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_04",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "OutTuple_04",
            25,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.OutTuple_05 (case 26)
#[test]
fn out_tuple_05() {
    let src = r#"M(out (var x, var y));"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_05",
                    26,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "OutTuple_05",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "OutTuple_05",
            26,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.NamedTupleOnTheLeft (case 27)
#[test]
fn named_tuple_on_the_left() {
    let src = r#"(x: 1, y: 2) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NamedTupleOnTheLeft",
                    27,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "NamedTupleOnTheLeft",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "NamedTupleOnTheLeft",
            27,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationExpressionTests.InvokeMethodNamedVar (case 28)
#[test]
fn invoke_method_named_var() {
    let src = r#"var(1, 2) = e;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "InvokeMethodNamedVar",
                    28,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_expression_tests",
                    "DeclarationExpressionTests",
                    "InvokeMethodNamedVar",
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
            "declaration_expression_tests",
            "DeclarationExpressionTests",
            "InvokeMethodNamedVar",
            28,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
