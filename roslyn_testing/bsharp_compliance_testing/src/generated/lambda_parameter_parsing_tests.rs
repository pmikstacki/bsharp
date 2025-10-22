// Auto-generated from Roslyn: LambdaParameterParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
/// Roslyn: LambdaParameterParsingTests.EndOfFileAfterOut (case 1)
#[test]
fn end_of_file_after_out() {
    let src = r#"
class C {
     void Goo() {
          System.Func<int, int> f = (out 
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOut",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOut",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "EndOfFileAfterOut",
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

/// Roslyn: LambdaParameterParsingTests.EndOfFileAfterOutType (case 2)
#[test]
fn end_of_file_after_out_type() {
    let src = r#"
class C {
     void Goo() {
          System.Func<int, int> f = (out C
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutType",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutType",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "EndOfFileAfterOutType",
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

/// Roslyn: LambdaParameterParsingTests.EndOfFileAfterOutTypeIdentifier (case 3)
#[test]
fn end_of_file_after_out_type_identifier() {
    let src = r#"
class C {
     void Goo() {
          System.Func<int, int> f = (out C c
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifier",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifier",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "EndOfFileAfterOutTypeIdentifier",
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

/// Roslyn: LambdaParameterParsingTests.EndOfFileAfterOutTypeIdentifierParen (case 4)
#[test]
fn end_of_file_after_out_type_identifier_paren() {
    let src = r#"
class C {
     void Goo() {
          System.Func<int, int> f = (out C c
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifierParen",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifierParen",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "EndOfFileAfterOutTypeIdentifierParen",
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

/// Roslyn: LambdaParameterParsingTests.EndOfFileAfterOutTypeIdentifierComma (case 5)
#[test]
fn end_of_file_after_out_type_identifier_comma() {
    let src = r#"
class C {
     void Goo() {
          System.Func<int, int> f = (out C c,
"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifierComma",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "EndOfFileAfterOutTypeIdentifierComma",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "EndOfFileAfterOutTypeIdentifierComma",
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

/// Roslyn: LambdaParameterParsingTests.TestLambdaWithNullValidation (case 6)
#[test]
fn lambda_with_null_validation() {
    let src = r#"Func<string, string> func1 = x!! => x + "1";"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func1 = x!! => x + "1"; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestLambdaWithNullValidation",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestLambdaWithNullValidation",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestLambdaWithNullValidation",
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

/// Roslyn: LambdaParameterParsingTests.TestLambdaWithNullValidationParams (case 7)
#[test]
fn lambda_with_null_validation_params() {
    let src = r#"Func<int, int, bool> func1 = (x!!, y) => x == y;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, bool> func1 = (x!!, y) => x == y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestLambdaWithNullValidationParams",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestLambdaWithNullValidationParams",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestLambdaWithNullValidationParams",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSingleParamInParens (case 8)
#[test]
fn null_checked_single_param_in_parens() {
    let src = r#"Func<int, int> func1 = (x!!) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (x!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSingleParamInParens",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSingleParamInParens",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSingleParamInParens",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSingleParamNoSpaces (case 9)
#[test]
fn null_checked_single_param_no_spaces() {
    let src = r#"Func<int, int> func1 = x!!=>x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = x!!=>x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSingleParamNoSpaces",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSingleParamNoSpaces",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSingleParamNoSpaces",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedTypedSingleParamInParen (case 10)
#[test]
fn null_checked_typed_single_param_in_paren() {
    let src = r#"Func<int, int> func1 = (int x!!) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (int x!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedTypedSingleParamInParen",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedTypedSingleParamInParen",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedTypedSingleParamInParen",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedTypedManyParams (case 11)
#[test]
fn null_checked_typed_many_params() {
    let src = r#"Func<int, int, int> func1 = (int x!!, int y) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, int> func1 = (int x!!, int y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedTypedManyParams",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedTypedManyParams",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedTypedManyParams",
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

/// Roslyn: LambdaParameterParsingTests.TestManyNullCheckedTypedParams (case 12)
#[test]
fn many_null_checked_typed_params() {
    let src = r#"Func<int, int, int> func1 = (int x!!, int y!!) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, int> func1 = (int x!!, int y!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestManyNullCheckedTypedParams",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestManyNullCheckedTypedParams",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestManyNullCheckedTypedParams",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedNoParams (case 13)
#[test]
fn null_checked_no_params() {
    let src = r#"Func<int> func1 = (!!) => 42;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int> func1 = (!!) => 42; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedNoParams",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedNoParams",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedNoParams",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDiscard (case 14)
#[test]
fn null_checked_discard() {
    let src = r#"Func<int, int> func1 = (_!!) => 42;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (_!!) => 42; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDiscard",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDiscard",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDiscard",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection0 (case 15)
#[test]
fn null_checked_syntax_correction_0() {
    let src = r#"Func<string, string> func0 = x!=> x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x!=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection0",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection0",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection0",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection1 (case 16)
#[test]
fn null_checked_syntax_correction_1() {
    let src = r#"Func<string, string> func1 = x !=> x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func1 = x !=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection1",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection2 (case 17)
#[test]
fn null_checked_syntax_correction_2() {
    let src = r#"Func<string, string> func2 = x != > x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func2 = x != > x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection2",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection2",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection3 (case 18)
#[test]
fn null_checked_syntax_correction_3() {
    let src = r#"Func<string, string> func3 = x! => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func3 = x! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection3",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection3",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection3",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection4 (case 19)
#[test]
fn null_checked_syntax_correction_4() {
    let src = r#"Func<string, string> func4 = x ! => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func4 = x ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection4",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection4",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection4",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection5 (case 20)
#[test]
fn null_checked_syntax_correction_5() {
    let src = r#"Func<string, string> func5 = x !!=> x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func5 = x !!=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection5",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection5",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection5",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection6 (case 21)
#[test]
fn null_checked_syntax_correction_6() {
    let src = r#"Func<string, string> func6 = x !!= > x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func6 = x !!= > x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection6",
                    21,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection6",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection6",
            21,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection7 (case 22)
#[test]
fn null_checked_syntax_correction_7() {
    let src = r#"Func<string, string> func7 = x!! => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func7 = x!! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection7",
                    22,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection7",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection7",
            22,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection8 (case 23)
#[test]
fn null_checked_syntax_correction_8() {
    let src = r#"Func<string, string> func8 = x! !=> x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func8 = x! !=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection8",
                    23,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection8",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection8",
            23,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection9 (case 24)
#[test]
fn null_checked_syntax_correction_9() {
    let src = r#"Func<string, string> func9 = x! ! => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func9 = x! ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection9",
                    24,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSyntaxCorrection9",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSyntaxCorrection9",
            24,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterSimpleLambdaName (case 25)
#[test]
fn braces_after_simple_lambda_name() {
    let src = r#"Func<string[], string> func0 = x[] => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = x[] => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterSimpleLambdaName",
                    25,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterSimpleLambdaName",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestBracesAfterSimpleLambdaName",
            25,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterParenthesizedLambdaName (case 26)
#[test]
fn braces_after_parenthesized_lambda_name() {
    let src = r#"Func<string[], string> func0 = (x[]) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = (x[]) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterParenthesizedLambdaName",
                    26,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterParenthesizedLambdaName",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestBracesAfterParenthesizedLambdaName",
            26,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterParenthesizedLambdaTypeAndName (case 27)
#[test]
fn braces_after_parenthesized_lambda_type_and_name() {
    let src = r#"Func<string[], string> func0 = (string x[]) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = (string x[]) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterParenthesizedLambdaTypeAndName",
                    27,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestBracesAfterParenthesizedLambdaTypeAndName",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestBracesAfterParenthesizedLambdaTypeAndName",
            27,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueSimpleLambda (case 28)
#[test]
fn default_value_simple_lambda() {
    let src = r#"Func<string, string> func0 = x = null => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x = null => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueSimpleLambda",
                    28,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueSimpleLambda",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueSimpleLambda",
            28,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValue_TypedSimpleLambda (case 29)
#[test]
fn default_value_typed_simple_lambda() {
    let src = r#"var f = int x = 3 => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { var f = int x = 3 => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValue_TypedSimpleLambda",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValue_TypedSimpleLambda",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValue_TypedSimpleLambda",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambda1 (case 30)
#[test]
fn default_value_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x = null) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambda1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambda1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueParenthesizedLambda1",
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

/// Roslyn: LambdaParameterParsingTests.TestImplicitDefaultValue_DelegateSyntax (case 31)
#[test]
fn implicit_default_value_delegate_syntax() {
    let src = r#"delegate(x = 3) { return x; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate(x = 3) { return x; }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestImplicitDefaultValue_DelegateSyntax",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestImplicitDefaultValue_DelegateSyntax",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestImplicitDefaultValue_DelegateSyntax",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambda2 (case 32)
#[test]
fn default_value_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x = null) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambda2",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambda2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueParenthesizedLambda2",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambdaWithType1 (case 33)
#[test]
fn default_value_parenthesized_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x = null) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambdaWithType1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambdaWithType1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueParenthesizedLambdaWithType1",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambdaWithType2 (case 34)
#[test]
fn default_value_parenthesized_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x = null) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambdaWithType2",
                    34,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueParenthesizedLambdaWithType2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueParenthesizedLambdaWithType2",
            34,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultMissingValueClauseSyntax_DelegateSyntax1 (case 35)
#[test]
fn default_missing_value_clause_syntax_delegate_syntax_1() {
    let src = r#"delegate(int x = , int y) { return x; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate(int x = , int y) { return x; }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultMissingValueClauseSyntax_DelegateSyntax1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultMissingValueClauseSyntax_DelegateSyntax1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultMissingValueClauseSyntax_DelegateSyntax1",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultMissingValueClause_DelegateSyntax2 (case 36)
#[test]
fn default_missing_value_clause_delegate_syntax_2() {
    let src = r#"delegate(int x = , int y) { return x; }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate(int x = , int y) { return x; }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultMissingValueClause_DelegateSyntax2",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultMissingValueClause_DelegateSyntax2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultMissingValueClause_DelegateSyntax2",
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

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueWithAttributeOnParam_DelegateSyntax (case 37)
#[test]
fn default_value_with_attribute_on_param_delegate_syntax() {
    let src = r#"delegate ([MyAttribute(3, arg1=true)] int x = -1) { return x; }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate ([MyAttribute(3, arg1=true)] int x = -1) { return x; }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueWithAttributeOnParam_DelegateSyntax",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestDefaultValueWithAttributeOnParam_DelegateSyntax",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestDefaultValueWithAttributeOnParam_DelegateSyntax",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueSimpleLambda (case 38)
#[test]
fn null_checked_default_value_simple_lambda() {
    let src = r#"Func<string, string> func0 = x!! = null => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x!! = null => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueSimpleLambda",
                    38,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueSimpleLambda",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDefaultValueSimpleLambda",
            38,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambda1 (case 39)
#[test]
fn null_checked_default_value_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x!! = null) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambda1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambda1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDefaultValueParenthesizedLambda1",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambda2 (case 40)
#[test]
fn null_checked_default_value_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x!! = null) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambda2",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambda2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDefaultValueParenthesizedLambda2",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambdaWithType1 (case 41)
#[test]
fn null_checked_default_value_parenthesized_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x!! = null) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambdaWithType1",
                    41,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambdaWithType1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDefaultValueParenthesizedLambdaWithType1",
            41,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambdaWithType2 (case 42)
#[test]
fn null_checked_default_value_parenthesized_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x!! = null) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambdaWithType2",
                    42,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedDefaultValueParenthesizedLambdaWithType2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedDefaultValueParenthesizedLambdaWithType2",
            42,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestGreaterThanTokenInEqualsValueClause (case 43)
#[test]
fn greater_than_token_in_equals_value_clause() {
    let src = r#"(int x = > 0) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int x = > 0) => x;; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestGreaterThanTokenInEqualsValueClause",
                    43,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestGreaterThanTokenInEqualsValueClause",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestGreaterThanTokenInEqualsValueClause",
            43,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestArgListWithDefaultParameterValue (case 44)
#[test]
fn arg_list_with_default_parameter_value() {
    let src = r#"(__arglist = null) => { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (__arglist = null) => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestArgListWithDefaultParameterValue",
                    44,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestArgListWithDefaultParameterValue",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestArgListWithDefaultParameterValue",
            44,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenSimpleLambda (case 45)
#[test]
fn null_checked_space_between_simple_lambda() {
    let src = r#"Func<string, string> func0 = x! ! => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x! ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenSimpleLambda",
                    45,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenSimpleLambda",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSpaceBetweenSimpleLambda",
            45,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenParenthesizedLambda1 (case 46)
#[test]
fn null_checked_space_between_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x! !) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenParenthesizedLambda1",
                    46,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenParenthesizedLambda1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSpaceBetweenParenthesizedLambda1",
            46,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenParenthesizedLambda2 (case 47)
#[test]
fn null_checked_space_between_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x! !) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenParenthesizedLambda2",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenParenthesizedLambda2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSpaceBetweenParenthesizedLambda2",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenLambdaWithType1 (case 48)
#[test]
fn null_checked_space_between_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x! !) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenLambdaWithType1",
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenLambdaWithType1",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSpaceBetweenLambdaWithType1",
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

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenLambdaWithType2 (case 49)
#[test]
fn null_checked_space_between_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x! !) => x;"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenLambdaWithType2",
                    49,
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
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "TestNullCheckedSpaceBetweenLambdaWithType2",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "TestNullCheckedSpaceBetweenLambdaWithType2",
            49,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: LambdaParameterParsingTests.AsyncAwaitInLambda (case 50)
#[test]
fn async_await_in_lambda() {
    let src = r#"F(async () => await Task.FromResult(4));"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "AsyncAwaitInLambda",
                    50,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "lambda_parameter_parsing_tests",
                    "LambdaParameterParsingTests",
                    "AsyncAwaitInLambda",
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
            "lambda_parameter_parsing_tests",
            "LambdaParameterParsingTests",
            "AsyncAwaitInLambda",
            50,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}
