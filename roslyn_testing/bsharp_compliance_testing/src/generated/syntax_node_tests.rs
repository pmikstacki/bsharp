// Auto-generated from Roslyn: SyntaxNodeTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SyntaxNodeTests.TestAddBaseListTypes (case 1)
#[test]
fn add_base_list_types() {
    let src = r#"class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestAddBaseListTypes", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 2)
#[test]
fn contains_directive() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 3)
#[test]
fn contains_directive_case_2() {
    let src = r#"namespace N { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 4)
#[test]
fn contains_directive_case_3() {
    let src = r#"namespace N { } #if false"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 5)
#[test]
fn contains_directive_case_4() {
    let src = r#"#!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 6)
#[test]
fn contains_directive_case_5() {
    let src = r#" #!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 7)
#[test]
fn contains_directive_case_6() {
    let src = r#"#!command"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestContainsDirective (case 8)
#[test]
fn contains_directive_case_7() {
    let src = r#"#:x"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestContainsDirective", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNode (case 9)
#[test]
fn replace_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNode", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNode (case 10)
#[test]
fn replace_node_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNode", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNodes (case 11)
#[test]
fn replace_nodes() {
    let src = r#"a + b + c + d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNodes", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 12)
#[test]
fn replace_node_in_list_with_multiple() {
    let src = r#"m(a, b)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNodeInListWithMultiple", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 13)
#[test]
fn replace_node_in_list_with_multiple_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNodeInListWithMultiple", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNodeInListWithMultiple (case 14)
#[test]
fn replace_node_in_list_with_multiple_case_3() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNodeInListWithMultiple", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 15)
#[test]
fn insert_nodes_in_list() {
    let src = r#"m(a, b)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertNodesInList", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 16)
#[test]
fn insert_nodes_in_list_case_2() {
    let src = r#"c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertNodesInList", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestInsertNodesInList (case 17)
#[test]
fn insert_nodes_in_list_case_3() {
    let src = r#"d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertNodesInList", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 18)
#[test]
fn replace_statement_in_list_with_multiple() {
    let src = r#"{ var x = 10; var y = 20; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceStatementInListWithMultiple", 18, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 19)
#[test]
fn replace_statement_in_list_with_multiple_case_2() {
    let src = r#"var z = 30; "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceStatementInListWithMultiple", 19, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestReplaceStatementInListWithMultiple (case 20)
#[test]
fn replace_statement_in_list_with_multiple_case_3() {
    let src = r#"var q = 40; "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceStatementInListWithMultiple", 20, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 21)
#[test]
fn insert_statements_in_list() {
    let src = r#"{ var x = 10; var y = 20; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertStatementsInList", 21, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 22)
#[test]
fn insert_statements_in_list_case_2() {
    let src = r#"var z = 30; "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertStatementsInList", 22, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestInsertStatementsInList (case 23)
#[test]
fn insert_statements_in_list_case_3() {
    let src = r#"var q = 40; "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertStatementsInList", 23, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: SyntaxNodeTests.TestReplaceSingleToken (case 24)
#[test]
fn replace_single_token() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceSingleToken", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceMultipleTokens (case 25)
#[test]
fn replace_multiple_tokens() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceMultipleTokens", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTokenWithMultipleTokens (case 26)
#[test]
fn replace_single_token_with_multiple_tokens() {
    let src = r#"private class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceSingleTokenWithMultipleTokens", 26, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestReplaceNonListTokenWithMultipleTokensFails (case 27)
#[test]
fn replace_non_list_token_with_multiple_tokens_fails() {
    let src = r#"private class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceNonListTokenWithMultipleTokensFails", 27, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestInsertTokens (case 28)
#[test]
fn insert_tokens() {
    let src = r#"public class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertTokens", 28, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestInsertTokensRelativeToNonListToken (case 29)
#[test]
fn insert_tokens_relative_to_non_list_token() {
    let src = r#"public class C { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertTokensRelativeToNonListToken", 29, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTriviaInNode (case 30)
#[test]
fn replace_single_trivia_in_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceSingleTriviaInNode", 30, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceMultipleTriviaInNode (case 31)
#[test]
fn replace_multiple_trivia_in_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceMultipleTriviaInNode", 31, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestReplaceSingleTriviaWithMultipleTriviaInNode (case 32)
#[test]
fn replace_single_trivia_with_multiple_trivia_in_node() {
    let src = r#"/* c */ identifier"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { /* c */ identifier; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestReplaceSingleTriviaWithMultipleTriviaInNode", 32, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestInsertTriviaInNode (case 33)
#[test]
fn insert_trivia_in_node() {
    let src = r#"/* c */ identifier"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { /* c */ identifier; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestInsertTriviaInNode", 33, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia (case 34)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(a, b, /* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepExteriorTrivia", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_2 (case 35)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_2() {
    let src = r#"m(a, b, /* trivia */
c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */
c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_2", 35, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_3 (case 36)
#[test]
fn remove_node_in_separated_list_keep_exterior_trivia_3() {
    let src = r#"m(a, b,
/* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b,
/* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_3", 36, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_4 (case 37)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_4", 37, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepExteriorTrivia_5 (case 38)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepExteriorTrivia_5", 38, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia (case 39)
#[test]
fn remove_node_in_separated_list_keep_no_trivia() {
    let src = r#"m(a, b, /* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepNoTrivia", 39, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_2 (case 40)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_2() {
    let src = r#"m(a, b, /* trivia */ 
c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* trivia */ 
c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepNoTrivia_2", 40, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_3 (case 41)
#[test]
fn remove_node_in_separated_list_keep_no_trivia_3() {
    let src = r#"m(a, b,
/* trivia */ c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b,
/* trivia */ c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepNoTrivia_3", 41, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_4 (case 42)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepNoTrivia_4", 42, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveNodeInSeparatedList_KeepNoTrivia_5 (case 43)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveNodeInSeparatedList_KeepNoTrivia_5", 43, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveOnlyNodeInSeparatedList_KeepExteriorTrivia (case 44)
#[test]
fn remove_only_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(/* before */ a /* after */)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(/* before */ a /* after */); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveOnlyNodeInSeparatedList_KeepExteriorTrivia", 44, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveFirstNodeInSeparatedList_KeepExteriorTrivia (case 45)
#[test]
fn remove_first_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(/* before */ a /* after */, b, c)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(/* before */ a /* after */, b, c); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveFirstNodeInSeparatedList_KeepExteriorTrivia", 45, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveLastNodeInSeparatedList_KeepExteriorTrivia (case 46)
#[test]
fn remove_last_node_in_separated_list_keep_exterior_trivia() {
    let src = r#"m(a, b, /* before */ c /* after */)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { m(a, b, /* before */ c /* after */); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveLastNodeInSeparatedList_KeepExteriorTrivia", 46, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestRemoveLastNode_KeepExteriorTrivia (case 47)
#[test]
fn remove_last_node_keep_exterior_trivia() {
    let src = r#"class C { void M() { } /* trivia */ }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveLastNode_KeepExteriorTrivia", 47, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestRemoveBadDirectiveWithoutEOL_KeepEndOfLine_KeepDirectives (case 48)
#[test]
fn remove_bad_directive_without_eol_keep_end_of_line_keep_directives() {
    let src = r#"class A { } class B { } #endregion"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveBadDirectiveWithoutEOL_KeepEndOfLine_KeepDirectives", 48, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestRemoveDocument_KeepEndOfLine (case 49)
#[test]
fn remove_document_keep_end_of_line() {
    let src = r#"
#region A
class A 
{ } 
#endregion"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveDocument_KeepEndOfLine", 49, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestRemoveFirstParameter_KeepTrailingTrivia (case 50)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveFirstParameter_KeepTrailingTrivia", 50, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestRemoveLastParameter_KeepLeadingTrivia (case 51)
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
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestRemoveLastParameter_KeepLeadingTrivia", 51, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 52)
#[test]
fn trivia_exists() {
    let src = r#"goo"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { goo; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestTriviaExists", 52, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 53)
#[test]
fn trivia_exists_case_2() {
    let src = r#" goo  "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  goo  ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestTriviaExists", 53, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 54)
#[test]
fn trivia_exists_case_3() {
    let src = r#"goo"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { goo; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestTriviaExists", 54, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SyntaxNodeTests.TestTriviaExists (case 55)
#[test]
fn trivia_exists_case_4() {
    let src = r#" goo  "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() {  goo  ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("syntax_node_tests", "SyntaxNodeTests", "TestTriviaExists", 55, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

