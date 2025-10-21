// Auto-generated from Roslyn: FileModifierParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: FileModifierParsingTests.TestFileModifierAfterIncompleteBaseList2 (case 1)
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
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("file_modifier_parsing_tests", "FileModifierParsingTests", "TestFileModifierAfterIncompleteBaseList2", 1, CaseData::File { unit: &unit, src, original: None });
}

