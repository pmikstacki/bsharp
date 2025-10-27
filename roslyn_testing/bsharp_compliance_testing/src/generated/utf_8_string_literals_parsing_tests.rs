// Auto-generated from Roslyn: Utf8StringLiteralsParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: Utf8StringLiteralsParsingTests.RegularStringLiteral_01 (case 1)
#[test]
fn regular_string_literal_01() {
    let src = r#"""hello"""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RegularStringLiteral_01",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RegularStringLiteral_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RegularStringLiteral_01",
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

/// Roslyn: Utf8StringLiteralsParsingTests.RegularStringLiteral_02 (case 2)
#[test]
fn regular_string_literal_02() {
    let src = r#"@""hello"""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RegularStringLiteral_02",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RegularStringLiteral_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RegularStringLiteral_02",
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

/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_01 (case 3)
#[test]
fn raw_string_literal_01() {
    let src = r#"""""""hello"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_01",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RawStringLiteral_01",
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

/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_02 (case 4)
#[test]
fn raw_string_literal_02() {
    let src = r#"""""""
hello
"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_02",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RawStringLiteral_02",
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

/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_03 (case 5)
#[test]
fn raw_string_literal_03() {
    let src = r#"@""""""hello"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""""""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_03",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_03",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RawStringLiteral_03",
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

/// Roslyn: Utf8StringLiteralsParsingTests.RawStringLiteral_04 (case 6)
#[test]
fn raw_string_literal_04() {
    let src = r#"@""""""
hello
"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""""""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_04",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "RawStringLiteral_04",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "RawStringLiteral_04",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_01 (case 7)
#[test]
fn utf_8_string_literal_01() {
    let src = r#"""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_01",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_01",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_02 (case 8)
#[test]
fn utf_8_string_literal_02() {
    let src = r#"""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_02",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_02",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_03 (case 9)
#[test]
fn utf_8_string_literal_03() {
    let src = r#"""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_03",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_03",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_03",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_04 (case 10)
#[test]
fn utf_8_string_literal_04() {
    let src = r#"@""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_04",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_04",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_04",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_05 (case 11)
#[test]
fn utf_8_string_literal_05() {
    let src = r#"@""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_05",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_05",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_05",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_06 (case 12)
#[test]
fn utf_8_string_literal_06() {
    let src = r#"@""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_06",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_06",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_06",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_07 (case 13)
#[test]
fn utf_8_string_literal_07() {
    let src = r#"""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_07",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_07",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_07",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_08 (case 14)
#[test]
fn utf_8_string_literal_08() {
    let src = r#"""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_08",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_08",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_08",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_09 (case 15)
#[test]
fn utf_8_string_literal_09() {
    let src = r#"""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_09",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_09",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_09",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_10 (case 16)
#[test]
fn utf_8_string_literal_10() {
    let src = r#"@""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_10",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_10",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_10",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_11 (case 17)
#[test]
fn utf_8_string_literal_11() {
    let src = r#"@""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_11",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_11",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_11",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_12 (case 18)
#[test]
fn utf_8_string_literal_12() {
    let src = r#"@""hello""U8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_12",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_12",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_12",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_01 (case 19)
#[test]
fn errors_01() {
    let src = r#"@""hello"" u8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello"" u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_01",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_01",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_02 (case 20)
#[test]
fn errors_02() {
    let src = r#"@""hello""u"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""u; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_02",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_02",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_03 (case 21)
#[test]
fn errors_03() {
    let src = r#"@""hello""8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_03",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_03",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_03",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_04 (case 22)
#[test]
fn errors_04() {
    let src = r#"@""hello""u80"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""u80; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_04",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_04",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_04",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_05 (case 23)
#[test]
fn errors_05() {
    let src = r#"1L0"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1L0; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_05",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_05",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_05",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_06 (case 24)
#[test]
fn errors_06() {
    let src = r#"1 L"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 L; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_06",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_06",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_06",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_07 (case 25)
#[test]
fn errors_07() {
    let src = r#"""hello"" u8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello"" u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_07",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_07",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_07",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_08 (case 26)
#[test]
fn errors_08() {
    let src = r#"""hello""u"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""u; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_08",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_08",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_08",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_09 (case 27)
#[test]
fn errors_09() {
    let src = r#"""hello""8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_09",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_09",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_09",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_10 (case 28)
#[test]
fn errors_10() {
    let src = r#"""hello""u80"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""u80; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_10",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_10",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_10",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_11 (case 29)
#[test]
fn errors_11() {
    let src = r#"@""hello"" U8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello"" U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_11",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_11",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_11",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_12 (case 30)
#[test]
fn errors_12() {
    let src = r#"@""hello""U"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""U; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_12",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_12",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_12",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_13 (case 31)
#[test]
fn errors_13() {
    let src = r#"@""hello""U80"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""hello""U80; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_13",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_13",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_13",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_14 (case 32)
#[test]
fn errors_14() {
    let src = r#"""hello"" U8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello"" U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_14",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_14",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_14",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_15 (case 33)
#[test]
fn errors_15() {
    let src = r#"""hello""U"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""U; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_15",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_15",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_15",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_16 (case 34)
#[test]
fn errors_16() {
    let src = r#"""hello""U80"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ""hello""U80; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_16",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_16",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_16",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_01 (case 35)
#[test]
fn interpolation_01() {
    let src = r#"$""hello""u8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_01",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_01",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_02 (case 36)
#[test]
fn interpolation_02() {
    let src = r#"$@""hello""u8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $@""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_02",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_02",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_03 (case 37)
#[test]
fn interpolation_03() {
    let src = r#"$""hello""U8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_03",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_03",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_03",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_04 (case 38)
#[test]
fn interpolation_04() {
    let src = r#"$@""hello""U8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $@""hello""U8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_04",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_04",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_04",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_13 (case 39)
#[test]
fn utf_8_string_literal_13() {
    let src = r#"""""""hello"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_13",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_13",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_13",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_14 (case 40)
#[test]
fn utf_8_string_literal_14() {
    let src = r#"@""""""hello"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""""""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_14",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_14",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_14",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_17 (case 41)
#[test]
fn errors_17() {
    let src = r#"""""""hello"""""" "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello"""""" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_17",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_17",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_17",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_18 (case 42)
#[test]
fn errors_18() {
    let src = r#"""""""hello"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_18",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_18",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_18",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_19 (case 43)
#[test]
fn errors_19() {
    let src = r#"""""""hello""""""8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_19",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_19",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_19",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_20 (case 44)
#[test]
fn errors_20() {
    let src = r#"""""""hello"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_20",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_20",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_20",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_05 (case 45)
#[test]
fn interpolation_05() {
    let src = r#"$""""""hello"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_05",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_05",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_05",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_06 (case 46)
#[test]
fn interpolation_06() {
    let src = r#"$@""""""hello"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $@""""""hello""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_06",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_06",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_06",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_15 (case 47)
#[test]
fn utf_8_string_literal_15() {
    let src = r#"""""""
hello
"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_15",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_15",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_15",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_16 (case 48)
#[test]
fn utf_8_string_literal_16() {
    let src = r#"@""""""
hello
"""""""#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { @""""""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_16",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_16",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_16",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_21 (case 49)
#[test]
fn errors_21() {
    let src = r#"""""""
hello
"""""" "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
"""""" ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_21",
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_21",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_21",
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

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_22 (case 50)
#[test]
fn errors_22() {
    let src = r#"""""""
hello
"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_22",
                    50,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_22",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_22",
            50,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_23 (case 51)
#[test]
fn errors_23() {
    let src = r#"""""""
hello
""""""8"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_23",
                    51,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_23",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_23",
            51,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Errors_24 (case 52)
#[test]
fn errors_24() {
    let src = r#"""""""
hello
"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { """"""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_24",
                    52,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Errors_24",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Errors_24",
            52,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_07 (case 53)
#[test]
fn interpolation_07() {
    let src = r#"$""""""
hello
"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_07",
                    53,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_07",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_07",
            53,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Interpolation_08 (case 54)
#[test]
fn interpolation_08() {
    let src = r#"$@""""""
hello
"""""""#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $@""""""
hello
""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_08",
                    54,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Interpolation_08",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Interpolation_08",
            54,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_Await_01 (case 55)
#[test]
fn utf_8_string_literal_await_01() {
    let src = r#"await ""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await ""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_01",
                    55,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_01",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_Await_01",
            55,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_Await_02 (case 56)
#[test]
fn utf_8_string_literal_await_02() {
    let src = r#"await @""hello""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await @""hello""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_02",
                    56,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_02",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_Await_02",
            56,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_Await_03 (case 57)
#[test]
fn utf_8_string_literal_await_03() {
    let src = r#"await """"""hello""""""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await """"""hello""""""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_03",
                    57,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_03",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_Await_03",
            57,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: Utf8StringLiteralsParsingTests.Utf8StringLiteral_Await_04 (case 58)
#[test]
fn utf_8_string_literal_await_04() {
    let src = r#"await """"""
hello
""""""u8"#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await """"""
hello
""""""u8; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_04",
                    58,
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
                    "utf_8_string_literals_parsing_tests",
                    "Utf8StringLiteralsParsingTests",
                    "Utf8StringLiteral_Await_04",
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
            "utf_8_string_literals_parsing_tests",
            "Utf8StringLiteralsParsingTests",
            "Utf8StringLiteral_Await_04",
            58,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}
