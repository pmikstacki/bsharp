// Auto-generated from Roslyn: SyntaxListTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 1)
#[test]
fn add_insert_remove_replace() {
    let src = r#"A "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 2)
#[test]
fn add_insert_remove_replace_case_2() {
    let src = r#"B "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 3)
#[test]
fn add_insert_remove_replace_case_3() {
    let src = r#"C "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                3,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 4)
#[test]
fn add_insert_remove_replace_case_4() {
    let src = r#"A "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                4,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 5)
#[test]
fn add_insert_remove_replace_case_5() {
    let src = r#"B "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { B ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                5,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 6)
#[test]
fn add_insert_remove_replace_case_6() {
    let src = r#"C "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { C ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                6,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 7)
#[test]
fn add_insert_remove_replace_case_7() {
    let src = r#"D "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                7,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.TestAddInsertRemoveReplace (case 8)
#[test]
fn add_insert_remove_replace_case_8() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "TestAddInsertRemoveReplace",
                8,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.DoTestAddInsertRemoveReplaceOnEmptyList (case 9)
#[test]
fn do_test_add_insert_remove_replace_on_empty_list() {
    let src = r#"D "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { D ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "DoTestAddInsertRemoveReplaceOnEmptyList",
                9,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.DoTestAddInsertRemoveReplaceOnEmptyList (case 10)
#[test]
fn do_test_add_insert_remove_replace_on_empty_list_case_2() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "DoTestAddInsertRemoveReplaceOnEmptyList",
                10,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.AddEmptySyntaxList (case 11)
#[test]
fn add_empty_syntax_list() {
    let src = r#"void"#;
    let span = Span::new(src);
    let src2 = r#"class C { void }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "AddEmptySyntaxList",
                11,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.AddNamespaceAttributeListsAndModifiers (case 12)
#[test]
fn add_namespace_attribute_lists_and_modifiers() {
    let src = r#"M"#;
    let span = Span::new(src);
    let src2 = r#"class C { M }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "AddNamespaceAttributeListsAndModifiers",
                12,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.AddNamespaceAttributeListsAndModifiers (case 13)
#[test]
fn add_namespace_attribute_lists_and_modifiers_case_2() {
    let src = r#"Attr"#;
    let span = Span::new(src);
    let src2 = r#"class C { Attr }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "AddNamespaceAttributeListsAndModifiers",
                13,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.AddNamespaceAttributeListsAndModifiers (case 14)
#[test]
fn add_namespace_attribute_lists_and_modifiers_case_3() {
    let src = r#"Attr"#;
    let span = Span::new(src);
    let src2 = r#"class C { Attr }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "AddNamespaceAttributeListsAndModifiers",
                14,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.Extensions (case 15)
#[test]
fn extensions() {
    let src = r#"A+B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A+B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "Extensions",
                15,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxListTests.Extensions (case 16)
#[test]
fn extensions_case_2() {
    let src = r#"1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_list_tests",
                "SyntaxListTests",
                "Extensions",
                16,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
