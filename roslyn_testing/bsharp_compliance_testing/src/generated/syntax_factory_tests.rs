// Auto-generated from Roslyn: SyntaxFactoryTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: SyntaxFactoryTests.UsingDirective (case 1)
#[test]
fn using_directive() {
    let src = r#"System.String"#;
    let span = Span::new(src);
    let src2 = r#"class C { System.String }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "UsingDirective",
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

/// Roslyn: SyntaxFactoryTests.GetTokenDiagnosticsWithSyntaxTree_WithDiagnostics (case 2)
#[test]
fn get_token_diagnostics_with_syntax_tree_with_diagnostics() {
    let src = r#"1l"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1l; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "GetTokenDiagnosticsWithSyntaxTree_WithDiagnostics",
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

/// Roslyn: SyntaxFactoryTests.GetTokenDiagnosticsWithSyntaxTree_WithoutDiagnostics (case 3)
#[test]
fn get_token_diagnostics_with_syntax_tree_without_diagnostics() {
    let src = r#"1L"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1L; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "GetTokenDiagnosticsWithSyntaxTree_WithoutDiagnostics",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnNullableDatetimeType (case 4)
#[test]
fn spacing_on_nullable_datetime_type() {
    let src = r#"DateTime"#;
    let span = Span::new(src);
    let src2 = r#"class C { DateTime }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnNullableDatetimeType",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnTernary (case 5)
#[test]
fn spacing_on_ternary() {
    let src = r#"x is int? y: z"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int? y: z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnTernary",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnTernary (case 6)
#[test]
fn spacing_on_ternary_case_2() {
    let src = r#"x is DateTime? y: z"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is DateTime? y: z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnTernary",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 7)
#[test]
fn spacing_on_coalescing() {
    let src = r#"x is int??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnCoalescing",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 8)
#[test]
fn spacing_on_coalescing_case_2() {
    let src = r#"x is DateTime??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is DateTime??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnCoalescing",
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

/// Roslyn: SyntaxFactoryTests.TestSpacingOnCoalescing (case 9)
#[test]
fn spacing_on_coalescing_case_3() {
    let src = r#"x is object??y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is object??y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestSpacingOnCoalescing",
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

/// Roslyn: SyntaxFactoryTests.TestUnnecessarySemicolon (case 10)
#[test]
fn unnecessary_semicolon() {
    let src = r#"int[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { int[] }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestUnnecessarySemicolon",
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

/// Roslyn: SyntaxFactoryTests.TestUnnecessarySemicolon (case 11)
#[test]
fn unnecessary_semicolon_case_2() {
    let src = r#"()"#;
    let span = Span::new(src);
    let src2 = r#"class C { () }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestUnnecessarySemicolon",
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

/// Roslyn: SyntaxFactoryTests.TestUnnecessarySemicolon (case 12)
#[test]
fn unnecessary_semicolon_case_3() {
    let src = r#"{}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestUnnecessarySemicolon",
                12,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxFactoryTests.TestCreateRecordWithMembers (case 13)
#[test]
fn create_record_with_members() {
    let src = r#"private int i;"#;
    let span = Span::new(src);
    let src2 = r#"class C { private int i; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestCreateRecordWithMembers",
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

/// Roslyn: SyntaxFactoryTests.TestCreateRecordWithMembers (case 14)
#[test]
fn create_record_with_members_case_2() {
    let src = r#"private int i;"#;
    let span = Span::new(src);
    let src2 = r#"class C { private int i; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestCreateRecordWithMembers",
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

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 15)
#[test]
fn parse_methods_keep_parse_options_in_the_tree() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
                15,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 16)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_2() {
    let src = r#""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
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

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 17)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_3() {
    let src = r#"public"#;
    let span = Span::new(src);
    let src2 = r#"class C { public }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
                17,
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

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 18)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_4() {
    let src = r#""#;
    let span = Span::new(src);
    let src2 = r#"class C {  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
                18,
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

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 19)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_5() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
                19,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxFactoryTests.TestParseMethodsKeepParseOptionsInTheTree (case 20)
#[test]
fn parse_methods_keep_parse_options_in_the_tree_case_6() {
    let src = r#""#;
    let span = Span::new(src);
    let src2 = r#"class C {  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_factory_tests",
                "SyntaxFactoryTests",
                "TestParseMethodsKeepParseOptionsInTheTree",
                20,
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
