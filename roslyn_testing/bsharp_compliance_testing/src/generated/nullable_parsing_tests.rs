// Auto-generated from Roslyn: NullableParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: NullableParsingTests.PartialAccessibilityAndNullableArray (case 1)
#[test]
fn partial_accessibility_and_nullable_array() {
    let src = r#"class C
{
    privat C[]? F;
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "PartialAccessibilityAndNullableArray", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: NullableParsingTests.NullableArray_Cast_01 (case 2)
#[test]
fn nullable_array_cast_01() {
    let src = r#"(object[]?)null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[]?)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_Cast_01", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullableArray_Cast_02 (case 3)
#[test]
fn nullable_array_cast_02() {
    let src = r#"(object[]??)null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[]??)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_Cast_02", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullableArray_Cast_04 (case 4)
#[test]
fn nullable_array_cast_04() {
    let src = r#"(object?[]?[])null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object?[]?[])null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_Cast_04", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullableArray_Cast_05 (case 5)
#[test]
fn nullable_array_cast_05() {
    let src = r#"(object[][]?[]?)null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (object[][]?[]?)null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_Cast_05", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.ConditionalOperator_NullableType (case 6)
#[test]
fn conditional_operator_nullable_type() {
    let src = r#"x is T ? ? y : z"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T ? ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "ConditionalOperator_NullableType", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.ConditionalOperator_NullableArray (case 7)
#[test]
fn conditional_operator_nullable_array() {
    let src = r#"x is T[] ? ? y : z"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is T[] ? ? y : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "ConditionalOperator_NullableArray", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NullableType (case 8)
#[test]
fn null_coalescing_operator_nullable_type() {
    let src = r#"x as T? ?? y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T? ?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullCoalescingOperator_NullableType", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullCoalescingOperator_NullableArray (case 9)
#[test]
fn null_coalescing_operator_nullable_array() {
    let src = r#"x as T[] ? ?? y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x as T[] ? ?? y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullCoalescingOperator_NullableArray", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.DeclarationPattern_NullableType (case 10)
#[test]
fn declaration_pattern_nullable_type() {
    let src = r#"switch (e) { case T? t: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "DeclarationPattern_NullableType", 10, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.DeclarationPattern_ArrayOfNullableType (case 11)
#[test]
fn declaration_pattern_array_of_nullable_type() {
    let src = r#"switch (e) { case T?[] t: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "DeclarationPattern_ArrayOfNullableType", 11, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.DeclarationPattern_NullableArrayOfArray (case 12)
#[test]
fn declaration_pattern_nullable_array_of_array() {
    let src = r#"switch (e) { case T[]?[] t: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "DeclarationPattern_NullableArrayOfArray", 12, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.NullableArray_Using (case 13)
#[test]
fn nullable_array_using() {
    let src = r#"using (A[]? a = b) { }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_Using", 13, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.NullableArray_ExplicitlyTypedLambda (case 14)
#[test]
fn nullable_array_explicitly_typed_lambda() {
    let src = r#"F((object[]? a) => a)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { F((object[]? a) => a); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_ExplicitlyTypedLambda", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.NullableArray_PartialMember (case 15)
#[test]
fn nullable_array_partial_member() {
    let src = r#"class C
{
    partial A[]? F();
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "NullableArray_PartialMember", 15, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: NullableParsingTests.CreateNullableArray_01 (case 16)
#[test]
fn create_nullable_array_01() {
    let src = r#"new object[,][]?"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,][]?; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "CreateNullableArray_01", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.CreateNullableArray_02 (case 17)
#[test]
fn create_nullable_array_02() {
    let src = r#"new object[,][]? { 1, 2 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,][]? { 1, 2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "CreateNullableArray_02", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.CreateNullableArray_04 (case 18)
#[test]
fn create_nullable_array_04() {
    let src = r#"new object[,]?[]?[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[,]?[]?[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "CreateNullableArray_04", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.CreateNullableArray_06 (case 19)
#[test]
fn create_nullable_array_06() {
    let src = r#"new object[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new object[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "CreateNullableArray_06", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.IsExpressionOfNullableTypeInStatement (case 20)
#[test]
fn is_expression_of_nullable_type_in_statement() {
    let src = r#"_ = x is Type?;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "IsExpressionOfNullableTypeInStatement", 20, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.DeclarationPatternOfNullableTypeInStatement (case 21)
#[test]
fn declaration_pattern_of_nullable_type_in_statement() {
    let src = r#"_ = x is Type? t;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "DeclarationPatternOfNullableTypeInStatement", 21, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: NullableParsingTests.DisjunctivePattern_NullableType2 (case 22)
#[test]
fn disjunctive_pattern_nullable_type_2() {
    let src = r#"x is int? i or string? s"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is int? i or string? s; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "DisjunctivePattern_NullableType2", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType2 (case 23)
#[test]
fn conjunctive_pattern_nullable_type_2() {
    let src = r#"x is Type? t and { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "ConjunctivePattern_NullableType2", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType4 (case 24)
#[test]
fn conjunctive_pattern_nullable_type_4() {
    let src = r#"x is Type? t and (1, 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and (1, 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "ConjunctivePattern_NullableType4", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: NullableParsingTests.ConjunctivePattern_NullableType6 (case 25)
#[test]
fn conjunctive_pattern_nullable_type_6() {
    let src = r#"x is Type? t and []"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is Type? t and []; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("nullable_parsing_tests", "NullableParsingTests", "ConjunctivePattern_NullableType6", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

