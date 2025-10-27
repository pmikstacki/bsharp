// Auto-generated from Roslyn: RefReadonlyTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: RefReadonlyTests.TestNewRefArray (case 1)
#[test]
fn new_ref_array() {
    let src = r#"new ref[];"#;
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
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "TestNewRefArray",
                    1,
                    Some(expected.clone()),
                    CaseData::Statement { ast: &ast, src },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "TestNewRefArray",
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
            "ref_readonly_tests",
            "RefReadonlyTests",
            "TestNewRefArray",
            1,
            None,
            CaseData::Statement { ast: &ast, src },
        );
    }
}

/// Roslyn: RefReadonlyTests.ArgumentModifier_RefReadonly (case 2)
#[test]
fn argument_modifier_ref_readonly() {
    let src = r#"M(ref x, in y, ref readonly z)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(ref x, in y, ref readonly z); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_RefReadonly",
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
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_RefReadonly",
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
            "ref_readonly_tests",
            "RefReadonlyTests",
            "ArgumentModifier_RefReadonly",
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

/// Roslyn: RefReadonlyTests.ArgumentModifier_ReadonlyRef (case 3)
#[test]
fn argument_modifier_readonly_ref() {
    let src = r#"M(readonly ref x)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(readonly ref x); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_ReadonlyRef",
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
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_ReadonlyRef",
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
            "ref_readonly_tests",
            "RefReadonlyTests",
            "ArgumentModifier_ReadonlyRef",
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

/// Roslyn: RefReadonlyTests.ArgumentModifier_Readonly (case 4)
#[test]
fn argument_modifier_readonly() {
    let src = r#"M(readonly x)"#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(readonly x); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_Readonly",
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
                    "ref_readonly_tests",
                    "RefReadonlyTests",
                    "ArgumentModifier_Readonly",
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
            "ref_readonly_tests",
            "RefReadonlyTests",
            "ArgumentModifier_Readonly",
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
