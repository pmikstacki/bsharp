// Auto-generated from Roslyn: LambdaAttributeParsingTests
/// Roslyn: LambdaAttributeParsingTests.ParenthesizedLambdaWithAttribute (case 1)
#[test]
fn parenthesized_lambda_with_attribute() {
    let src = r#"f = ([A] x => x)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { f = ([A] x => x); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParenthesizedLambdaWithAttribute",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParenthesizedLambdaWithAttribute",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParenthesizedLambdaWithAttribute",
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

/// Roslyn: LambdaAttributeParsingTests.CollectionInitializer_01 (case 2)
#[test]
fn collection_initializer_01() {
    let src = r#"new B { [A] x => y }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new B { [A] x => y }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "CollectionInitializer_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "CollectionInitializer_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "CollectionInitializer_01",
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

/// Roslyn: LambdaAttributeParsingTests.CollectionInitializer_02 (case 3)
#[test]
fn collection_initializer_02() {
    let src = r#"new B { ([A] x => y) }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new B { ([A] x => y) }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "CollectionInitializer_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "CollectionInitializer_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "CollectionInitializer_02",
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

/// Roslyn: LambdaAttributeParsingTests.PostfixOperator (case 4)
#[test]
fn postfix_operator() {
    let src = r#"[A] () => { } ++"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] () => { } ++; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "PostfixOperator",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "PostfixOperator",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "PostfixOperator",
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

/// Roslyn: LambdaAttributeParsingTests.PrefixOperator (case 5)
#[test]
fn prefix_operator() {
    let src = r#"-- [A] () => { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { -- [A] () => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "PrefixOperator",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "PrefixOperator",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "PrefixOperator",
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

/// Roslyn: LambdaAttributeParsingTests.UnaryOperator (case 6)
#[test]
fn unary_operator() {
    let src = r#"! [A] () => { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ! [A] () => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "UnaryOperator",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "UnaryOperator",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "UnaryOperator",
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

/// Roslyn: LambdaAttributeParsingTests.Cast (case 7)
#[test]
fn cast() {
    let src = r#"(F) [A] () => { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (F) [A] () => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Cast",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Cast",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Cast",
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

/// Roslyn: LambdaAttributeParsingTests.BinaryOperator_01 (case 8)
#[test]
fn binary_operator_01() {
    let src = r#"[A] () => { } + y"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] () => { } + y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "BinaryOperator_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "BinaryOperator_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "BinaryOperator_01",
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

/// Roslyn: LambdaAttributeParsingTests.BinaryOperator_02 (case 9)
#[test]
fn binary_operator_02() {
    let src = r#"x * [A] () => { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x * [A] () => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "BinaryOperator_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "BinaryOperator_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "BinaryOperator_02",
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

/// Roslyn: LambdaAttributeParsingTests.Is (case 10)
#[test]
fn is() {
    let src = r#"[A] () => { } is E"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] () => { } is E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Is",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Is",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Is",
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

/// Roslyn: LambdaAttributeParsingTests.As (case 11)
#[test]
fn as_() {
    let src = r#"[A] () => x as E"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] () => x as E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "As",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "As",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "As",
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

/// Roslyn: LambdaAttributeParsingTests.ConditionalExpression_01 (case 12)
#[test]
fn conditional_expression_01() {
    let src = r#"x ? [A] () => { } : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x ? [A] () => { } : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ConditionalExpression_01",
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

/// Roslyn: LambdaAttributeParsingTests.ConditionalExpression_01_A (case 13)
#[test]
fn conditional_expression_01_a() {
    let src = r#"x ? () => { } : z"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x ? () => { } : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_01_A",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_01_A",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ConditionalExpression_01_A",
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

/// Roslyn: LambdaAttributeParsingTests.ConditionalExpression_02 (case 14)
#[test]
fn conditional_expression_02() {
    let src = r#"x ? y : [A] () => { }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x ? y : [A] () => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ConditionalExpression_02",
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

/// Roslyn: LambdaAttributeParsingTests.ConditionalExpression_03 (case 15)
#[test]
fn conditional_expression_03() {
    let src = r#"x ? ([A] () => { }) : y"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x ? ([A] () => { }) : y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_03",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ConditionalExpression_03",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ConditionalExpression_03",
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

/// Roslyn: LambdaAttributeParsingTests.SwitchExpression_01 (case 16)
#[test]
fn switch_expression_01() {
    let src = r#"[A] () => { } switch { }"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] () => { } switch { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "SwitchExpression_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "SwitchExpression_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "SwitchExpression_01",
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

/// Roslyn: LambdaAttributeParsingTests.SwitchExpression_02 (case 17)
#[test]
fn switch_expression_02() {
    let src = r#"x switch { y => [A] () => { }, _ => [A] () => z }"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x switch { y => [A] () => { }, _ => [A] () => z }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "SwitchExpression_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "SwitchExpression_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "SwitchExpression_02",
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

/// Roslyn: LambdaAttributeParsingTests.Tuple_01 (case 18)
#[test]
fn tuple_01() {
    let src = r#"([A] () => { }, y)"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ([A] () => { }, y); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Tuple_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Tuple_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Tuple_01",
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

/// Roslyn: LambdaAttributeParsingTests.Tuple_02 (case 19)
#[test]
fn tuple_02() {
    let src = r#"(x, [A] () => { })"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (x, [A] () => { }); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Tuple_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Tuple_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Tuple_02",
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

/// Roslyn: LambdaAttributeParsingTests.Range_01 (case 20)
#[test]
fn range_01() {
    let src = r#"s[[A] x => x..]"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { s[[A] x => x..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Range_01",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Range_01",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Range_01",
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

/// Roslyn: LambdaAttributeParsingTests.Range_02 (case 21)
#[test]
fn range_02() {
    let src = r#"s[..[A] () => { }]"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { s[..[A] () => { }]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Range_02",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "Range_02",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "Range_02",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda1 (case 22)
#[test]
fn parse_attribute_with_lambda_1() {
    let src = r#"
                // Lambda inside attribute with attributes of its own will cause us to bail out.
                [A([B]() => {})]
                class C
                {
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda1",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda1",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda1",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda2 (case 23)
#[test]
fn parse_attribute_with_lambda_2() {
    let src = r#"
                // Lambda inside attribute without attributes of its own is fine for parsing.
                [A(() => {})]
                class C
                {
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda2",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda2",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda2",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda3 (case 24)
#[test]
fn parse_attribute_with_lambda_3() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        // Because we're already in an expression, parsing an attribute list, we'll bail out of parsing the `[B]`
                        // as an attribute on an inner lambda.
                        var v = [A([B]() => {})]
                            () => {};
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda3",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda3",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda3",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda3_B (case 25)
#[test]
fn parse_attribute_with_lambda_3_b() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        // This is an attributed lambda *within* a collection expression.  This is fine.
                        var v = [A([B]() => {})];
                    }
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda3_B",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda3_B",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda3_B",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda4 (case 26)
#[test]
fn parse_attribute_with_lambda_4() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var v = [A(() => {})] () => {};
                    }
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda4",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda4",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda4",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda5 (case 27)
#[test]
fn parse_attribute_with_lambda_5() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        var v = [A] () =>
                        {
                            // This attribute, within an expression of a lambda, should be totally fine to parse.  It
                            // is not within an attribute argument itself.
                            var y = [B] () => { };
                        };
                    }
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda5",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda5",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda5",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda6 (case 28)
#[test]
fn parse_attribute_with_lambda_6() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        // We won't recognize this as a lambda because we'll bail out from default-parameter parsing
                        // when we see the `[` after the `=`.
                        var v = (X x = [A] () => {}) => { };
                    }
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda6",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithLambda6",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithLambda6",
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

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithCollectionExpression (case 29)
#[test]
fn parse_attribute_with_collection_expression() {
    let src = r#"
                class C
                {
                    void M()
                    {
                        // We won't recognize this as a lambda because we'll bail out from default-parameter parsing
                        // when we see the `[` after the `=`.
                        var v = (X x = [0]) => { };
                    }
                }
                "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithCollectionExpression",
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
                    "lambda_attribute_parsing_tests",
                    "LambdaAttributeParsingTests",
                    "ParseAttributeWithCollectionExpression",
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
            "lambda_attribute_parsing_tests",
            "LambdaAttributeParsingTests",
            "ParseAttributeWithCollectionExpression",
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
