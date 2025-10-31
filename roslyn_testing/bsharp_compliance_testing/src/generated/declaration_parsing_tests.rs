// Auto-generated from Roslyn: DeclarationParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws_spanned;
use bsharp_syntax::span::Span;
/// Roslyn: DeclarationParsingTests.CS0071_01 (case 1)
#[test]
fn cs_0071_01() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.P10;
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_01",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_01",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "CS0071_01",
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

/// Roslyn: DeclarationParsingTests.CS0071_02 (case 2)
#[test]
fn cs_0071_02() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.
P10;
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_02",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_02",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "CS0071_02",
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

/// Roslyn: DeclarationParsingTests.CS0071_03 (case 3)
#[test]
fn cs_0071_03() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.
P10
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_03",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_03",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "CS0071_03",
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

/// Roslyn: DeclarationParsingTests.CS0071_04 (case 4)
#[test]
fn cs_0071_04() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.P10
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_04",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "CS0071_04",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "CS0071_04",
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

/// Roslyn: DeclarationParsingTests.NonAccessorAfterIncompleteProperty (case 5)
#[test]
fn non_accessor_after_incomplete_property() {
    let src = r#"
class C
{
    int A { get { return this.
    public int B;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "NonAccessorAfterIncompleteProperty",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "NonAccessorAfterIncompleteProperty",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "NonAccessorAfterIncompleteProperty",
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

/// Roslyn: DeclarationParsingTests.ExpressionBodiedCtorDtorProp (case 6)
#[test]
fn expression_bodied_ctor_dtor_prop() {
    let src = r#"
class C
{
    C() : base() => M();
    C() => M();
    ~C() => M();
    int P { set => M(); }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "ExpressionBodiedCtorDtorProp",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "ExpressionBodiedCtorDtorProp",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "ExpressionBodiedCtorDtorProp",
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

/// Roslyn: DeclarationParsingTests.ParseOutVar (case 7)
#[test]
fn parse_out_var() {
    let src = r#"
class C
{
    void Goo()
    {
        M(out var x);
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "ParseOutVar",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "ParseOutVar",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "ParseOutVar",
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

/// Roslyn: DeclarationParsingTests.TestPartiallyWrittenConstraintClauseInBaseList1 (case 8)
#[test]
fn partially_written_constraint_clause_in_base_list_1() {
    let src = r#"
class C<T> : where
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList1",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList1",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestPartiallyWrittenConstraintClauseInBaseList1",
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

/// Roslyn: DeclarationParsingTests.TestPartiallyWrittenConstraintClauseInBaseList2 (case 9)
#[test]
fn partially_written_constraint_clause_in_base_list_2() {
    let src = r#"
class C<T> : where T
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList2",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList2",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestPartiallyWrittenConstraintClauseInBaseList2",
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

/// Roslyn: DeclarationParsingTests.TestPartiallyWrittenConstraintClauseInBaseList3 (case 10)
#[test]
fn partially_written_constraint_clause_in_base_list_3() {
    let src = r#"
class C<T> : where T :
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList3",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList3",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestPartiallyWrittenConstraintClauseInBaseList3",
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

/// Roslyn: DeclarationParsingTests.TestPartiallyWrittenConstraintClauseInBaseList4 (case 11)
#[test]
fn partially_written_constraint_clause_in_base_list_4() {
    let src = r#"
class C<T> : where T : X
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList4",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestPartiallyWrittenConstraintClauseInBaseList4",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestPartiallyWrittenConstraintClauseInBaseList4",
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

/// Roslyn: DeclarationParsingTests.TestMethodDeclarationNullValidation (case 12)
#[test]
fn method_declaration_null_validation() {
    let src = r#"void M(string name!!) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation",
                    12,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestMethodDeclarationNullValidation",
            12,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestMethodDeclarationNullValidation_SingleExclamation (case 13)
#[test]
fn method_declaration_null_validation_single_exclamation() {
    let src = r#"void M(string name!) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_SingleExclamation",
                    13,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_SingleExclamation",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestMethodDeclarationNullValidation_SingleExclamation",
            13,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestMethodDeclarationNullValidation_SingleExclamation_ExtraTrivia (case 14)
#[test]
fn method_declaration_null_validation_single_exclamation_extra_trivia() {
    let src = r#"void M(string name
                /*comment1*/!/*comment2*/) { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_SingleExclamation_ExtraTrivia",
                    14,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_SingleExclamation_ExtraTrivia",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestMethodDeclarationNullValidation_SingleExclamation_ExtraTrivia",
            14,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestOptParamMethodDeclarationWithNullValidation (case 15)
#[test]
fn opt_param_method_declaration_with_null_validation() {
    let src = r#"void M(string name!! = null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestOptParamMethodDeclarationWithNullValidation",
                    15,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestOptParamMethodDeclarationWithNullValidation",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestOptParamMethodDeclarationWithNullValidation",
            15,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestOptParamMethodDeclarationWithNullValidationNoSpaces (case 16)
#[test]
fn opt_param_method_declaration_with_null_validation_no_spaces() {
    let src = r#"void M(string name!!=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestOptParamMethodDeclarationWithNullValidationNoSpaces",
                    16,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestOptParamMethodDeclarationWithNullValidationNoSpaces",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestOptParamMethodDeclarationWithNullValidationNoSpaces",
            16,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgList1 (case 17)
#[test]
fn null_checked_arg_list_1() {
    let src = r#"void M(__arglist!) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList1",
                    17,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList1",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgList1",
            17,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgList2 (case 18)
#[test]
fn null_checked_arg_list_2() {
    let src = r#"void M(__arglist!!) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList2",
                    18,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList2",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgList2",
            18,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgList3 (case 19)
#[test]
fn null_checked_arg_list_3() {
    let src = r#"void M(__arglist!! = null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList3",
                    19,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList3",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgList3",
            19,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgList4 (case 20)
#[test]
fn null_checked_arg_list_4() {
    let src = r#"void M(__arglist!!= null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList4",
                    20,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList4",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgList4",
            20,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgList5 (case 21)
#[test]
fn null_checked_arg_list_5() {
    let src = r#"void M(__arglist[]!!= null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList5",
                    21,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgList5",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgList5",
            21,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestArgListWithBrackets (case 22)
#[test]
fn arg_list_with_brackets() {
    let src = r#"void M(__arglist[]) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestArgListWithBrackets",
                    22,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestArgListWithBrackets",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestArgListWithBrackets",
            22,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestArgListWithDefaultValue (case 23)
#[test]
fn arg_list_with_default_value() {
    let src = r#"void M(__arglist = null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestArgListWithDefaultValue",
                    23,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestArgListWithDefaultValue",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestArgListWithDefaultValue",
            23,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithLeadingSpace (case 24)
#[test]
fn null_checked_arg_with_leading_space() {
    let src = r#"void M(string name !!=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithLeadingSpace",
                    24,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithLeadingSpace",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithLeadingSpace",
            24,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithLeadingNewLine (case 25)
#[test]
fn null_checked_arg_with_leading_new_line() {
    let src = r#"void M(string name!!=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithLeadingNewLine",
                    25,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithLeadingNewLine",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithLeadingNewLine",
            25,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithTrailingSpace (case 26)
#[test]
fn null_checked_arg_with_trailing_space() {
    let src = r#"void M(string name!!= null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithTrailingSpace",
                    26,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithTrailingSpace",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithTrailingSpace",
            26,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithTrailingNewLine (case 27)
#[test]
fn null_checked_arg_with_trailing_new_line() {
    let src = r#"void M(string name!!=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithTrailingNewLine",
                    27,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithTrailingNewLine",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithTrailingNewLine",
            27,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithSpaceInbetween (case 28)
#[test]
fn null_checked_arg_with_space_inbetween() {
    let src = r#"void M(string name! !=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceInbetween",
                    28,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceInbetween",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithSpaceInbetween",
            28,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithSpaceAfterParam (case 29)
#[test]
fn null_checked_arg_with_space_after_param() {
    let src = r#"void M(string name !!=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterParam",
                    29,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterParam",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithSpaceAfterParam",
            29,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithSpaceAfterBangs (case 30)
#[test]
fn null_checked_arg_with_space_after_bangs() {
    let src = r#"void M(string name! ! =null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterBangs",
                    30,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterBangs",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithSpaceAfterBangs",
            30,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithSpaceBeforeBangs (case 31)
#[test]
fn null_checked_arg_with_space_before_bangs() {
    let src = r#"void M(string name ! !=null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceBeforeBangs",
                    31,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceBeforeBangs",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithSpaceBeforeBangs",
            31,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedArgWithSpaceAfterEquals (case 32)
#[test]
fn null_checked_arg_with_space_after_equals() {
    let src = r#"void M(string name!!= null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterEquals",
                    32,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedArgWithSpaceAfterEquals",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedArgWithSpaceAfterEquals",
            32,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestMethodDeclarationNullValidation_ExtraEquals (case 33)
#[test]
fn method_declaration_null_validation_extra_equals() {
    let src = r#"void M(string name!!= = null) { }"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_ExtraEquals",
                    33,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestMethodDeclarationNullValidation_ExtraEquals",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestMethodDeclarationNullValidation_ExtraEquals",
            33,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: DeclarationParsingTests.TestNullCheckedMethod (case 34)
#[test]
fn null_checked_method() {
    let src = r#"
class C
{
    public void M(string x!!) { }
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedMethod",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedMethod",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedMethod",
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

/// Roslyn: DeclarationParsingTests.TestNullCheckedConstructor (case 35)
#[test]
fn null_checked_constructor() {
    let src = r#"
class C
{
    public C(string x!!) { }
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedConstructor",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedConstructor",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedConstructor",
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

/// Roslyn: DeclarationParsingTests.TestNullCheckedOperator (case 36)
#[test]
fn null_checked_operator() {
    let src = r#"
class Box
{
    public static int operator+ (Box b!!, Box c) 
    {
        return 2;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedOperator",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestNullCheckedOperator",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestNullCheckedOperator",
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

/// Roslyn: DeclarationParsingTests.TestAnonymousDelegateNullChecking (case 37)
#[test]
fn anonymous_delegate_null_checking() {
    let src = r#"
delegate void Del(int x!!);
Del d = delegate(int k!!) { /* ... */ };"#;
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestAnonymousDelegateNullChecking",
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
                    "declaration_parsing_tests",
                    "DeclarationParsingTests",
                    "TestAnonymousDelegateNullChecking",
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
            "declaration_parsing_tests",
            "DeclarationParsingTests",
            "TestAnonymousDelegateNullChecking",
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
