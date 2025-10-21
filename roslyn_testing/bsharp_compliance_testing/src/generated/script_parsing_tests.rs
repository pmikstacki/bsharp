// Auto-generated from Roslyn: ScriptParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ScriptParsingTests.MethodDeclarationAndMethodCall (case 1)
#[test]
fn method_declaration_and_method_call() {
    let src = r#"
void bar() { }
bar();
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "MethodDeclarationAndMethodCall", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewAnonymousTypeExpressionStatement (case 2)
#[test]
fn new_anonymous_type_expression_statement() {
    let src = r#"new { a = 1 };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewAnonymousTypeExpressionStatement", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewArrayExpressionStatement (case 3)
#[test]
fn new_array_expression_statement() {
    let src = r#"new T[5];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionStatement", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewArrayExpressionWithInitializerAndPostFixExpressionStatement (case 4)
#[test]
fn new_array_expression_with_initializer_and_post_fix_expression_statement() {
    let src = r#"new int[] { }.Clone();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewArrayExpressionWithInitializerAndPostFixExpressionStatement", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewModifier_Method_WithBody (case 5)
#[test]
fn new_modifier_method_with_body() {
    let src = r#"new void Goo() { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewModifier_Method_WithBody", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewModifier_Class (case 6)
#[test]
fn new_modifier_class() {
    let src = r#"
new class C { }
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewModifier_Class", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.NewModifier_PartialClass (case 7)
#[test]
fn new_modifier_partial_class() {
    let src = r#"
new partial class C { }
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "NewModifier_PartialClass", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Unsafe_Block (case 8)
#[test]
fn unsafe_block() {
    let src = r#"
unsafe { }
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Unsafe_Block", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Unsafe_Field (case 9)
#[test]
fn unsafe_field() {
    let src = r#"
unsafe int Goo;
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Unsafe_Field", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Unsafe_Method (case 10)
#[test]
fn unsafe_method() {
    let src = r#"
unsafe void Goo() { }
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Unsafe_Method", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Delegate1 (case 11)
#[test]
fn delegate_1() {
    let src = r#"
delegate { }();
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Delegate1", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Delegate2 (case 12)
#[test]
fn delegate_2() {
    let src = r#"
delegate(){ }();
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Delegate2", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Delegate3 (case 13)
#[test]
fn delegate_3() {
    let src = r#"
delegate void Goo();
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Delegate3", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Multiplication_Interactive_Semicolon (case 14)
#[test]
fn multiplication_interactive_semicolon() {
    let src = r#"a * b;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Multiplication_Interactive_Semicolon", 14, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon1 (case 15)
#[test]
fn ternary_field_decl_semicolon_1() {
    let src = r#"T ? a;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon1", 15, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon2 (case 16)
#[test]
fn ternary_field_decl_semicolon_2() {
    let src = r#"T ? b, c = 1;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon2", 16, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon3 (case 17)
#[test]
fn ternary_field_decl_semicolon_3() {
    let src = r#"T ? b = d => { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon3", 17, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ScriptParsingTests.Ternary_FieldDecl_Semicolon4 (case 18)
#[test]
fn ternary_field_decl_semicolon_4() {
    let src = r#"T ? b = x ? y : z;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("script_parsing_tests", "ScriptParsingTests", "Ternary_FieldDecl_Semicolon4", 18, CaseData::File { unit: &unit, src, original: None });
}

