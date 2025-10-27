// Auto-generated STRUCTURE tests from Roslyn: PatternParsingTests
use crate::custom_asserts::structure_assert;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
#[test]
fn throw_expression() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}
