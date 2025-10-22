// Auto-generated from Roslyn: SyntaxNodeOrTokenListTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: SyntaxNodeOrTokenListTests.TestAddInsertRemove (case 1)
#[test]
fn add_insert_remove() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_or_token_list_tests",
                "SyntaxNodeOrTokenListTests",
                "TestAddInsertRemove",
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

/// Roslyn: SyntaxNodeOrTokenListTests.DoTestAddInsertRemoveReplaceOnEmptyList (case 2)
#[test]
fn do_test_add_insert_remove_replace_on_empty_list() {
    let src = r#"E "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { E ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_node_or_token_list_tests",
                "SyntaxNodeOrTokenListTests",
                "DoTestAddInsertRemoveReplaceOnEmptyList",
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
