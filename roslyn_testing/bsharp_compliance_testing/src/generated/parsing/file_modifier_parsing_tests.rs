// Auto-generated STRUCTURE tests from Roslyn: FileModifierParsingTests
use bsharp_parser::syntax::span::Span;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::structure_assert;
#[test]
fn file_modifier_after_incomplete_base_list_1() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn file_modifier_after_incomplete_base_list_2() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

