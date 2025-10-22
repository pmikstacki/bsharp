// Auto-generated STRUCTURE tests from Roslyn: ParserRegressionTests
use bsharp_parser::syntax::span::Span;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::structure_assert;
#[test]
fn qualified_name_01() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn qualified_name_02() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn qualified_name_03() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn qualified_name_04() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn qualified_name_05() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn qualified_name_07() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

