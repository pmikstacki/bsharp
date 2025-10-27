// Auto-generated from Roslyn: SyntaxRewriterTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_syntax::span::Span;
/// Roslyn: SyntaxRewriterTests.TestSyntaxTreeForParsedSyntaxNode (case 1)
#[test]
fn syntax_tree_for_parsed_syntax_node() {
    let src = r#"class Class1<T> { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestSyntaxTreeForParsedSyntaxNode",
                1,
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

/// Roslyn: SyntaxRewriterTests.TestSyntaxTreeForParsedSyntaxNode (case 2)
#[test]
fn syntax_tree_for_parsed_syntax_node_case_2() {
    let src = r#"2 + 2"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 2 + 2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestSyntaxTreeForParsedSyntaxNode",
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

/// Roslyn: SyntaxRewriterTests.TestSyntaxTreeForSyntaxTreeWithReplacedNode (case 3)
#[test]
fn syntax_tree_for_syntax_tree_with_replaced_node() {
    let src = r#"Class2<U>"#;
    let span = Span::new(src);
    let src2 = r#"class C { Class2<U> }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestSyntaxTreeForSyntaxTreeWithReplacedNode",
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

/// Roslyn: SyntaxRewriterTests.TestReplaceNodeInListShouldNotLoseParseOptions (case 4)
#[test]
fn replace_node_in_list_should_not_lose_parse_options() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestReplaceNodeInListShouldNotLoseParseOptions",
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

/// Roslyn: SyntaxRewriterTests.TestReplaceNodeInListShouldNotLoseParseOptions (case 5)
#[test]
fn replace_node_in_list_should_not_lose_parse_options_case_2() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestReplaceNodeInListShouldNotLoseParseOptions",
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

/// Roslyn: SyntaxRewriterTests.TestInsertNodeShouldNotLoseParseOptions (case 6)
#[test]
fn insert_node_should_not_lose_parse_options() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestInsertNodeShouldNotLoseParseOptions",
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

/// Roslyn: SyntaxRewriterTests.TestInsertNodeShouldNotLoseParseOptions (case 7)
#[test]
fn insert_node_should_not_lose_parse_options_case_2() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "TestInsertNodeShouldNotLoseParseOptions",
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

/// Roslyn: SyntaxRewriterTests.RewriteMissingIdentifierInExpressionStatement_ImplicitlyCreatedSyntaxTree (case 8)
#[test]
fn rewrite_missing_identifier_in_expression_statement_implicitly_created_syntax_tree() {
    let src = r#"if (true)"#;
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
                "syntax_rewriter_tests",
                "SyntaxRewriterTests",
                "RewriteMissingIdentifierInExpressionStatement_ImplicitlyCreatedSyntaxTree",
                8,
                None,
                CaseData::Statement { ast: &ast, src },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
