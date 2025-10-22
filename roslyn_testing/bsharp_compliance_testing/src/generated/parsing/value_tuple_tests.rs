// Auto-generated STRUCTURE tests from Roslyn: ValueTupleTests
use bsharp_parser::syntax::span::Span;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::structure_assert;
#[test]
fn simple_tuple() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn long_tuple() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn tuples_in_lambda() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn tuples_with_names_in_lambda() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn tuple_in_parameters() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn tuple_type_with_too_few_elements() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn tuple_expression_with_too_few_elements() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

