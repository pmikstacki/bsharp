// Auto-generated from Roslyn: SyntaxNodeTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws_spanned;
use bsharp_syntax::span::Span;
/// Roslyn: SyntaxNodeTests.TestQualifiedNameSyntaxWith (case 1)
#[test]
fn qualified_name_syntax_with() {
    let src = r#"A.B"#;
    let span = Span::new(src);
    let src2 = r#"class C { A.B }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestQualifiedNameSyntaxWith",
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

/// Roslyn: SyntaxNodeTests.TestAddBaseListTypes (case 2)
#[test]
fn add_base_list_types() {
    let src = r#"class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestAddBaseListTypes",
                2,
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

/// Roslyn: SyntaxNodeTests.TestAddBaseListTypes (case 3)
#[test]
fn add_base_list_types_case_2() {
    let src = r#"B"#;
    let span = Span::new(src);
    let src2 = r#"class C { B }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestAddBaseListTypes",
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 4)
#[test]
fn contains_directive() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                4,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 5)
#[test]
fn contains_directive_case_2() {
    let src = r#"namespace N { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                5,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 6)
#[test]
fn contains_directive_case_3() {
    let src = r#"namespace N { } #if false"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                6,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 7)
#[test]
fn contains_directive_case_4() {
    let src = r#"#!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                7,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 8)
#[test]
fn contains_directive_case_5() {
    let src = r#" #!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                8,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 9)
#[test]
fn contains_directive_case_6() {
    let src = r#"#!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                9,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 10)
#[test]
fn contains_directive_case_7() {
    let src = r#"#:x"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective",
                10,
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

/// Roslyn: SyntaxNodeTests.TestContainsDirective_IfIf (case 11)
#[test]
fn contains_directive_if_if() {
    let src = r#"
                if (#if)
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestContainsDirective_IfIf",
                11,
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

/// Roslyn: SyntaxNodeTests.TestReplaceNode (case 12)
#[test]
fn replace_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNode",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNode (case 13)
#[test]
fn replace_node_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNode",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNodes (case 14)
#[test]
fn replace_nodes() {
    let src = r#"a + b + c + d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNodes",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 15)
#[test]
fn replace_node_in_list_with_multiple() {
    let src = r#"m(a, b)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNodeInListWithMultiple",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 16)
#[test]
fn replace_node_in_list_with_multiple_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNodeInListWithMultiple",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 17)
#[test]
fn replace_node_in_list_with_multiple_case_3() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNodeInListWithMultiple",
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

/// Roslyn: SyntaxNodeTests.TestReplaceNonListNodeWithMultiple (case 18)
#[test]
fn replace_non_list_node_with_multiple() {
    let src = r#"if (a < b) m(c)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNonListNodeWithMultiple",
                18,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceNonListNodeWithMultiple (case 19)
#[test]
fn replace_non_list_node_with_multiple_case_2() {
    let src = r#"m1(x)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNonListNodeWithMultiple",
                19,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceNonListNodeWithMultiple (case 20)
#[test]
fn replace_non_list_node_with_multiple_case_3() {
    let src = r#"m2(y)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNonListNodeWithMultiple",
                20,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 21)
#[test]
fn insert_nodes_in_list() {
    let src = r#"m(a, b)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesInList",
                21,
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

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 22)
#[test]
fn insert_nodes_in_list_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesInList",
                22,
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

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 23)
#[test]
fn insert_nodes_in_list_case_3() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesInList",
                23,
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

/// Roslyn: SyntaxNodeTests.TestInsertNodesRelativeToNonListNode (case 24)
#[test]
fn insert_nodes_relative_to_non_list_node() {
    let src = r#"if (a < b) m(c)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesRelativeToNonListNode",
                24,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesRelativeToNonListNode (case 25)
#[test]
fn insert_nodes_relative_to_non_list_node_case_2() {
    let src = r#"m1(x)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesRelativeToNonListNode",
                25,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesRelativeToNonListNode (case 26)
#[test]
fn insert_nodes_relative_to_non_list_node_case_3() {
    let src = r#"m2(y)"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertNodesRelativeToNonListNode",
                26,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 27)
#[test]
fn replace_statement_in_list_with_multiple() {
    let src = r#"{ var x = 10; var y = 20; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceStatementInListWithMultiple",
                27,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 28)
#[test]
fn replace_statement_in_list_with_multiple_case_2() {
    let src = r#"var z = 30; "#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceStatementInListWithMultiple",
                28,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 29)
#[test]
fn replace_statement_in_list_with_multiple_case_3() {
    let src = r#"var q = 40; "#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceStatementInListWithMultiple",
                29,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 30)
#[test]
fn insert_statements_in_list() {
    let src = r#"{ var x = 10; var y = 20; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertStatementsInList",
                30,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 31)
#[test]
fn insert_statements_in_list_case_2() {
    let src = r#"var z = 30; "#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertStatementsInList",
                31,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 32)
#[test]
fn insert_statements_in_list_case_3() {
    let src = r#"var q = 40; "#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertStatementsInList",
                32,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestReplaceSingleToken (case 33)
#[test]
fn replace_single_token() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceSingleToken",
                33,
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

/// Roslyn: SyntaxNodeTests.TestReplaceMultipleTokens (case 34)
#[test]
fn replace_multiple_tokens() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceMultipleTokens",
                34,
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

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTokenWithMultipleTokens (case 35)
#[test]
fn replace_single_token_with_multiple_tokens() {
    let src = r#"private class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceSingleTokenWithMultipleTokens",
                35,
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

/// Roslyn: SyntaxNodeTests.TestReplaceNonListTokenWithMultipleTokensFails (case 36)
#[test]
fn replace_non_list_token_with_multiple_tokens_fails() {
    let src = r#"private class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceNonListTokenWithMultipleTokensFails",
                36,
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

/// Roslyn: SyntaxNodeTests.TestInsertTokens (case 37)
#[test]
fn insert_tokens() {
    let src = r#"public class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertTokens",
                37,
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

/// Roslyn: SyntaxNodeTests.TestInsertTokensRelativeToNonListToken (case 38)
#[test]
fn insert_tokens_relative_to_non_list_token() {
    let src = r#"public class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertTokensRelativeToNonListToken",
                38,
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

/// Roslyn: SyntaxNodeTests.TestReplaceTriviaDeep (case 39)
#[test]
fn replace_trivia_deep() {
    let src = r#"#if true
a + 
#endif
 + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { #if true
a + 
#endif
 + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceTriviaDeep",
                39,
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

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTriviaInNode (case 40)
#[test]
fn replace_single_trivia_in_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceSingleTriviaInNode",
                40,
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

/// Roslyn: SyntaxNodeTests.TestReplaceMultipleTriviaInNode (case 41)
#[test]
fn replace_multiple_trivia_in_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceMultipleTriviaInNode",
                41,
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

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTriviaWithMultipleTriviaInNode (case 42)
#[test]
fn replace_single_trivia_with_multiple_trivia_in_node() {
    let src = r#"/* c */ identifier"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { /* c */ identifier; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestReplaceSingleTriviaWithMultipleTriviaInNode",
                42,
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

/// Roslyn: SyntaxNodeTests.TestInsertTriviaInNode (case 43)
#[test]
fn insert_trivia_in_node() {
    let src = r#"/* c */ identifier"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { /* c */ identifier; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestInsertTriviaInNode",
                43,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia (case 44)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(a, b, /* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepExteriorTrivia",
                44,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_2 (case 45)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_2() {
    let src = r#"m(a, b, /* trivia */
c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */
c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_2",
                45,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_3 (case 46)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_3() {
    let src = r#"m(a, b,
/* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b,
/* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_3",
                46,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_4 (case 47)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_4() {
    let src = r#"SomeMethod(/*arg1:*/ a,
    /*arg2:*/ b,
    /*arg3:*/ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { SomeMethod(/*arg1:*/ a,
    /*arg2:*/ b,
    /*arg3:*/ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_4",
                47,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_5 (case 48)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_5() {
    let src = r#"SomeMethod(// comment about a
           a,
           // some comment about b
           b,
           // some comment about c
           c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { SomeMethod(// comment about a
           a,
           // some comment about b
           b,
           // some comment about c
           c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_5",
                48,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia (case 49)
#[test]
fn remove_node_in_separated_list_keep_no_trivia() {
    let src = r#"m(a, b, /* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepNoTrivia",
                49,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_2 (case 50)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_2() {
    let src = r#"m(a, b, /* trivia */ 
c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ 
c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepNoTrivia_2",
                50,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_3 (case 51)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_3() {
    let src = r#"m(a, b,
/* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b,
/* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepNoTrivia_3",
                51,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_4 (case 52)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_4() {
    let src = r#"SomeMethod(/*arg1:*/ a,
    /*arg2:*/ b,
    /*arg3:*/ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { SomeMethod(/*arg1:*/ a,
    /*arg2:*/ b,
    /*arg3:*/ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepNoTrivia_4",
                52,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_5 (case 53)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_5() {
    let src = r#"SomeMethod(// comment about a
           a,
           // some comment about b
           b,
           // some comment about c
           c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { SomeMethod(// comment about a
           a,
           // some comment about b
           b,
           // some comment about c
           c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNodeInSeparatedList_KeepNoTrivia_5",
                53,
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

/// Roslyn: SyntaxNodeTests.TestRemoveOnlyNodeInSeparatedList_KeepExteriorTrivia (case 54)
#[test]
fn remove_only_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(/* before */ a /* after */)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(/* before */ a /* after */); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveOnlyNodeInSeparatedList_KeepExteriorTrivia",
                54,
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

/// Roslyn: SyntaxNodeTests.TestRemoveFirstNodeInSeparatedList_KeepExteriorTrivia (case 55)
#[test]
fn remove_first_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(/* before */ a /* after */, b, c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(/* before */ a /* after */, b, c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveFirstNodeInSeparatedList_KeepExteriorTrivia",
                55,
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

/// Roslyn: SyntaxNodeTests.TestRemoveLastNodeInSeparatedList_KeepExteriorTrivia (case 56)
#[test]
fn remove_last_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(a, b, /* before */ c /* after */)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* before */ c /* after */); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveLastNodeInSeparatedList_KeepExteriorTrivia",
                56,
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

/// Roslyn: SyntaxNodeTests.TestRemoveNode_KeepNoTrivia (case 57)
#[test]
fn remove_node_keep_no_trivia() {
    let src = r#"{ a; b; /* trivia */ c }"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNode_KeepNoTrivia",
                57,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestRemoveNode_KeepExteriorTrivia (case 58)
#[test]
fn remove_node_keep_exterior_trivia() {
    let src = r#"{ a; b; /* trivia */ c }"#;
    let span = Span::new(src);
    let r = parse_statement_ws_spanned(span).map(|(rest, s)| (rest, s.node));
    match r {
        Ok((rest, ast)) => {
            assert!(
                rest.fragment().trim().is_empty(),
                "Unconsumed input: {}",
                rest.fragment()
            );
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveNode_KeepExteriorTrivia",
                58,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxNodeTests.TestRemoveLastNode_KeepExteriorTrivia (case 59)
#[test]
fn remove_last_node_keep_exterior_trivia() {
    let src = r#"class C { void M() { } /* trivia */ }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveLastNode_KeepExteriorTrivia",
                59,
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

/// Roslyn: SyntaxNodeTests.TestRemove_KeepExteriorTrivia_KeepUnbalancedDirectives (case 60)
#[test]
fn remove_keep_exterior_trivia_keep_unbalanced_directives() {
    let src = r#"
class C
{
// before
void M()
{
#region Fred
} // after
#endregion
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemove_KeepExteriorTrivia_KeepUnbalancedDirectives",
                60,
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

/// Roslyn: SyntaxNodeTests.TestRemoveWithoutEOL_KeepEndOfLine (case 61)
#[test]
fn remove_without_eol_keep_end_of_line() {
    let src = r#"class A { } class B { } // test"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveWithoutEOL_KeepEndOfLine",
                61,
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

/// Roslyn: SyntaxNodeTests.TestRemoveBadDirectiveWithoutEOL_KeepEndOfLine_KeepDirectives (case 62)
#[test]
fn remove_bad_directive_without_eol_keep_end_of_line_keep_directives() {
    let src = r#"class A { } class B { } #endregion"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveBadDirectiveWithoutEOL_KeepEndOfLine_KeepDirectives",
                62,
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

/// Roslyn: SyntaxNodeTests.TestRemoveDocument_KeepEndOfLine (case 63)
#[test]
fn remove_document_keep_end_of_line() {
    let src = r#"
#region A
class A 
{ } 
#endregion"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveDocument_KeepEndOfLine",
                63,
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

/// Roslyn: SyntaxNodeTests.TestRemoveFirstParameter_KeepTrailingTrivia (case 64)
#[test]
fn remove_first_parameter_keep_trailing_trivia() {
    let src = r#"
class C
{
void M(
// before a
int a

// before b
, /* after comma */ int b
/* after b*/)
{
}
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveFirstParameter_KeepTrailingTrivia",
                64,
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

/// Roslyn: SyntaxNodeTests.TestRemoveLastParameter_KeepLeadingTrivia (case 65)
#[test]
fn remove_last_parameter_keep_leading_trivia() {
    let src = r#"
class C
{
void M(
// before a
int a, /* after comma */ 

// before b
int b /* after b*/)
{
}
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestRemoveLastParameter_KeepLeadingTrivia",
                65,
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

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 66)
#[test]
fn trivia_exists() {
    let src = r#"goo"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { goo; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestTriviaExists",
                66,
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

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 67)
#[test]
fn trivia_exists_case_2() {
    let src = r#" goo  "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  goo  ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestTriviaExists",
                67,
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

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 68)
#[test]
fn trivia_exists_case_3() {
    let src = r#"goo"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { goo; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestTriviaExists",
                68,
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

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 69)
#[test]
fn trivia_exists_case_4() {
    let src = r#" goo  "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  goo  ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_tests",
                "SyntaxNodeTests",
                "TestTriviaExists",
                69,
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
