// Auto-generated STRUCTURE tests from Roslyn: AllowsConstraintParsing
use bsharp_parser::syntax::span::Span;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::structure_assert;
#[test]
fn ref_struct_single() {
    let src = r#"
class C<T> where T : allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_missing_ref() {
    let src = r#"
class C<T> where T : allows struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_missing_struct() {
    let src = r#"
class C<T> where T : allows ref
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_missing_ref_and_struct() {
    let src = r#"
class C<T> where T : allows
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_escaped_allows() {
    let src = r#"
class C<T> where T : @allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_escaped_ref() {
    let src = r#"
class C<T> where T : allows @ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_single_escaped_struct() {
    let src = r#"
class C<T> where T : allows ref @struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_two_in_arow() {
    let src = r#"
class C<T> where T : allows ref struct, ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_two_in_arow_missing_ref() {
    let src = r#"
class C<T> where T : allows ref struct, struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_two_in_arow_missing_struct() {
    let src = r#"
class C<T> where T : allows ref struct, ref
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_two_allows_in_arow() {
    let src = r#"
class C<T> where T : allows ref struct, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_acomma_01() {
    let src = r#"
class C<T> where T : allows ref struct, 
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_acomma_02() {
    let src = r#"
class C<T> where T : struct, allows ref struct, 
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_acomma_and_where_01() {
    let src = r#"
class C<T, S> where T : allows ref struct, where S : class
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_acomma_and_where_02() {
    let src = r#"
class C<T, S> where T : struct, allows ref struct, where S : class
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_where_01() {
    let src = r#"
class C<T, S> where T : allows ref struct where S : class
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_followed_by_where_02() {
    let src = r#"
class C<T, S> where T : struct, allows ref struct where S : class
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_struct() {
    let src = r#"
class C<T> where T : struct, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_struct_and_missing_comma() {
    let src = r#"
class C<T> where T : struct allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_class() {
    let src = r#"
class C<T> where T : class, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_default() {
    let src = r#"
class C<T> where T : default, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_unmanaged() {
    let src = r#"
class C<T> where T : unmanaged, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_not_null() {
    let src = r#"
class C<T> where T : notnull, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_type_constraint() {
    let src = r#"
class C<T> where T : SomeType, allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_new() {
    let src = r#"
class C<T> where T : new(), allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_after_multiple() {
    let src = r#"
class C<T> where T : struct, SomeType, new(), allows ref struct
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_class() {
    let src = r#"
class C<T> where T : allows ref struct, class
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_default() {
    let src = r#"
class C<T> where T : allows ref struct, default
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_unmanaged() {
    let src = r#"
class C<T> where T : allows ref struct, unmanaged
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_not_null() {
    let src = r#"
class C<T> where T : allows ref struct, notnull
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_type_constraint() {
    let src = r#"
class C<T> where T : allows ref struct, SomeType
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

#[test]
fn ref_struct_before_new() {
    let src = r#"
class C<T> where T : allows ref struct, new()
{}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![] } };
    structure_assert::assert_tree(&expected, &unit);
}

