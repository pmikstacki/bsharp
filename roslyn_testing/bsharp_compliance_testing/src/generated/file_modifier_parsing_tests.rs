// Auto-generated from Roslyn: FileModifierParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: FileModifierParsingTests.TestFileModifierAfterIncompleteBaseList1 (case 1)
#[test]
fn file_modifier_after_incomplete_base_list_1() {
    let src = r#"
            class C : B
            file class D
            {
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
                    "file_modifier_parsing_tests",
                    "FileModifierParsingTests",
                    "TestFileModifierAfterIncompleteBaseList1",
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
                    "file_modifier_parsing_tests",
                    "FileModifierParsingTests",
                    "TestFileModifierAfterIncompleteBaseList1",
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
            "file_modifier_parsing_tests",
            "FileModifierParsingTests",
            "TestFileModifierAfterIncompleteBaseList1",
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

/// Roslyn: FileModifierParsingTests.TestFileModifierAfterIncompleteBaseList2 (case 2)
#[test]
fn file_modifier_after_incomplete_base_list_2() {
    let src = r#"
            class C : B, file
            {
            }

            class file
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
                    "file_modifier_parsing_tests",
                    "FileModifierParsingTests",
                    "TestFileModifierAfterIncompleteBaseList2",
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
                    "file_modifier_parsing_tests",
                    "FileModifierParsingTests",
                    "TestFileModifierAfterIncompleteBaseList2",
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
            "file_modifier_parsing_tests",
            "FileModifierParsingTests",
            "TestFileModifierAfterIncompleteBaseList2",
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
