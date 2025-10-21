// Auto-generated from Roslyn: MemberDeclarationParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: MemberDeclarationParsingTests.ParsePrivate (case 1)
#[test]
fn parse_private() {
    let src = r#"private"#;
    let span = Span::new(src);
    let src2 = r#"class C { private }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ParsePrivate", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.Statement (case 2)
#[test]
fn statement() {
    let src = r#"x = x + 1;"#;
    let span = Span::new(src);
    let src2 = r#"class C { x = x + 1; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Statement", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.Namespace (case 3)
#[test]
fn namespace() {
    let src = r#"namespace ns {}"#;
    let span = Span::new(src);
    let src2 = r#"namespace ns {}"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Namespace", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.TypeDeclaration (case 4)
#[test]
fn type_declaration() {
    let src = r#"class C { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TypeDeclaration", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MethodDeclaration (case 5)
#[test]
fn method_declaration() {
    let src = r#"void M() { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MethodDeclaration", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.FieldDeclaration (case 6)
#[test]
fn field_declaration() {
    let src = r#"static int F1 = a, F2 = b;"#;
    let span = Span::new(src);
    let src2 = r#"class C { static int F1 = a, F2 = b; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "FieldDeclaration", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CtorDeclaration (case 7)
#[test]
fn ctor_declaration() {
    let src = r#"public ThisClassName(int x) : base(x) { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { public ThisClassName(int x) : base(x) { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CtorDeclaration", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.DtorDeclaration (case 8)
#[test]
fn dtor_declaration() {
    let src = r#"public ~ThisClassName() { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { public ~ThisClassName() { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DtorDeclaration", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration (case 9)
#[test]
fn conversion_declaration() {
    let src = r#"public implicit operator long(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public implicit operator long(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration (case 10)
#[test]
fn operator_declaration() {
    let src = r#"public int operator +(int x, int y) => x + y;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_01 (case 11)
#[test]
fn unsigned_right_shift_operator_01() {
    let src = r#"C operator >>>(C x, C y) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>>(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_01", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_02 (case 12)
#[test]
fn unsigned_right_shift_operator_02() {
    let src = r#"C operator > >>(C x, C y) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >>(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_02", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_03 (case 13)
#[test]
fn unsigned_right_shift_operator_03() {
    let src = r#"C operator >> >(C x, C y) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> >(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_03", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_04 (case 14)
#[test]
fn unsigned_right_shift_operator_04() {
    let src = r#"C operator >>>=(C x, C y) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>>=(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_04", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.TrashAfterDeclaration (case 15)
#[test]
fn trash_after_declaration() {
    let src = r#"public int x; public int y"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int x; public int y }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.TrashAfterDeclaration (case 16)
#[test]
fn trash_after_declaration_case_2() {
    let src = r#"public int x; public int y"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int x; public int y }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_01 (case 17)
#[test]
fn generic_async_task_01() {
    let src = r#"async Task<SomeNamespace.SomeType Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace.SomeType Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_01", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_01 (case 18)
#[test]
fn generic_public_task_01() {
    let src = r#"public Task<SomeNamespace.SomeType Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace.SomeType Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_01", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_02 (case 19)
#[test]
fn generic_async_task_02() {
    let src = r#"async Task<SomeNamespace. Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace. Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_02", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_02 (case 20)
#[test]
fn generic_public_task_02() {
    let src = r#"public Task<SomeNamespace. Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace. Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_02", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_03 (case 21)
#[test]
fn generic_async_task_03() {
    let src = r#"async Task<SomeNamespace.> Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace.> Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_03", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_03 (case 22)
#[test]
fn generic_public_task_03() {
    let src = r#"public Task<SomeNamespace.> Method();"#;
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace.> Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_03", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.InitAccessor (case 23)
#[test]
fn init_accessor() {
    let src = r#"string Property { get; init; }"#;
    let span = Span::new(src);
    let src2 = r#"class C { string Property { get; init; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAccessor", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.InitAndSetAccessor (case 24)
#[test]
fn init_and_set_accessor() {
    let src = r#"string Property { init; set; }"#;
    let span = Span::new(src);
    let src2 = r#"class C { string Property { init; set; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAndSetAccessor", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.SetAndInitAccessor (case 25)
#[test]
fn set_and_init_accessor() {
    let src = r#"string Property { set; init; }"#;
    let span = Span::new(src);
    let src2 = r#"class C { string Property { set; init; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "SetAndInitAccessor", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_01 (case 26)
#[test]
fn required_modifier_property_01() {
    let src = r#"required string Prop { get; }"#;
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_01", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_07 (case 27)
#[test]
fn required_modifier_property_07() {
    let src = r#"required Type required { get; }"#;
    let span = Span::new(src);
    let src2 = r#"class C { required Type required { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_07", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_01 (case 28)
#[test]
fn required_modifier_field_01() {
    let src = r#"required string Field;"#;
    let span = Span::new(src);
    let src2 = r#"class C { required string Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_01", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_02 (case 29)
#[test]
fn required_modifier_field_02() {
    let src = r#"required Field;"#;
    let span = Span::new(src);
    let src2 = r#"class C { required Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_02", 29, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_03 (case 30)
#[test]
fn required_modifier_field_03() {
    let src = r#"required Field;"#;
    let span = Span::new(src);
    let src2 = r#"class C { required Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_03", 30, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_04 (case 31)
#[test]
fn required_modifier_field_04() {
    let src = r#"required required;"#;
    let span = Span::new(src);
    let src2 = r#"class C { required required; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_04", 31, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_05 (case 32)
#[test]
fn required_modifier_field_05() {
    let src = r#"required required;"#;
    let span = Span::new(src);
    let src2 = r#"class C { required required; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_05", 32, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_01 (case 33)
#[test]
fn required_modifier_method_01() {
    let src = r#"required string M() {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { required string M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_01", 33, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_02 (case 34)
#[test]
fn required_modifier_method_02() {
    let src = r#"required M() {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { required M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_02", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_03 (case 35)
#[test]
fn required_modifier_method_03() {
    let src = r#"required M() {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { required M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_03", 35, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierOperator (case 36)
#[test]
fn required_modifier_operator() {
    let src = r#"static required C operator+(C c1, C c2) {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { static required C operator+(C c1, C c2) {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierOperator", 36, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierConversion_01 (case 37)
#[test]
fn required_modifier_conversion_01() {
    let src = r#"static required implicit operator C(S s) {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { static required implicit operator C(S s) {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierConversion_01", 37, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_01 (case 38)
#[test]
fn required_modifier_incomplete_member_01() {
    let src = r#"required string Prop"#;
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_01", 38, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_02 (case 39)
#[test]
fn required_modifier_incomplete_member_02() {
    let src = r#"required string"#;
    let span = Span::new(src);
    let src2 = r#"class C { required string }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_02", 39, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_03 (case 40)
#[test]
fn required_modifier_incomplete_member_03() {
    let src = r#"required C"#;
    let span = Span::new(src);
    let src2 = r#"class C { required C }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_03", 40, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_04 (case 41)
#[test]
fn required_modifier_incomplete_member_04() {
    let src = r#"required"#;
    let span = Span::new(src);
    let src2 = r#"class C { required }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_04", 41, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_05 (case 42)
#[test]
fn required_modifier_incomplete_member_05() {
    let src = r#"required"#;
    let span = Span::new(src);
    let src2 = r#"class C { required }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_05", 42, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifier_LocalNamedRequired_TopLevelStatements (case 43)
#[test]
fn required_modifier_local_named_required_top_level_statements() {
    let src = r#"
                bool required;
                required = true;
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifier_LocalNamedRequired_TopLevelStatements", 43, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_06 (case 44)
#[test]
fn operator_declaration_explicit_implementation_06() {
    let src = r#"public int N::I::operator +(int x, int y) => x + y;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int N::I::operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_06", 44, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_07 (case 45)
#[test]
fn operator_declaration_explicit_implementation_07() {
    let src = r#"public int I::operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int I::operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_07", 45, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_08 (case 46)
#[test]
fn operator_declaration_explicit_implementation_08() {
    let src = r#"public int I.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_08", 46, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_09 (case 47)
#[test]
fn operator_declaration_explicit_implementation_09() {
    let src = r#"public int I<T>.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int I<T>.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_09", 47, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_10 (case 48)
#[test]
fn operator_declaration_explicit_implementation_10() {
    let src = r#"public int N1::N2::I.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int N1::N2::I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_10", 48, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_21 (case 49)
#[test]
fn operator_declaration_explicit_implementation_21() {
    let src = r#"public int I..operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_21", 49, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_22 (case 50)
#[test]
fn operator_declaration_explicit_implementation_22() {
    let src = r#"public int I . . operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { public int I . . operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_22", 50, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_28 (case 51)
#[test]
fn operator_declaration_explicit_implementation_28() {
    let src = r#"int N::I::operator +(int x, int y) => x + y;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N::I::operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_28", 51, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_29 (case 52)
#[test]
fn operator_declaration_explicit_implementation_29() {
    let src = r#"int I::operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I::operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_29", 52, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_30 (case 53)
#[test]
fn operator_declaration_explicit_implementation_30() {
    let src = r#"int I.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_30", 53, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_31 (case 54)
#[test]
fn operator_declaration_explicit_implementation_31() {
    let src = r#"int I<T>.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I<T>.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_31", 54, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_32 (case 55)
#[test]
fn operator_declaration_explicit_implementation_32() {
    let src = r#"int N1::N2::I.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2::I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_32", 55, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_43 (case 56)
#[test]
fn operator_declaration_explicit_implementation_43() {
    let src = r#"int I..operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_43", 56, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_44 (case 57)
#[test]
fn operator_declaration_explicit_implementation_44() {
    let src = r#"int I . . operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I . . operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_44", 57, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_45 (case 58)
#[test]
fn operator_declaration_explicit_implementation_45() {
    let src = r#"int N.I..operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N.I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_45", 58, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_46 (case 59)
#[test]
fn operator_declaration_explicit_implementation_46() {
    let src = r#"N.I.operator +(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { N.I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_46", 59, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_47 (case 60)
#[test]
fn operator_declaration_explicit_implementation_47() {
    let src = r#"N.I. int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { N.I. int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_47", 60, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_03 (case 61)
#[test]
fn conversion_declaration_explicit_implementation_03() {
    let src = r#"operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_03", 61, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_06 (case 62)
#[test]
fn conversion_declaration_explicit_implementation_06() {
    let src = r#"implicit N::I::operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { implicit N::I::operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_06", 62, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_07 (case 63)
#[test]
fn conversion_declaration_explicit_implementation_07() {
    let src = r#"explicit I::operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I::operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_07", 63, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_08 (case 64)
#[test]
fn conversion_declaration_explicit_implementation_08() {
    let src = r#"implicit I.operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { implicit I.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_08", 64, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_09 (case 65)
#[test]
fn conversion_declaration_explicit_implementation_09() {
    let src = r#"explicit I<T>.operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I<T>.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_09", 65, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_10 (case 66)
#[test]
fn conversion_declaration_explicit_implementation_10() {
    let src = r#"implicit N1::N2::I.operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { implicit N1::N2::I.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_10", 66, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_21 (case 67)
#[test]
fn conversion_declaration_explicit_implementation_21() {
    let src = r#"explicit I..operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I..operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_21", 67, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_22 (case 68)
#[test]
fn conversion_declaration_explicit_implementation_22() {
    let src = r#"implicit I . . operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { implicit I . . operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_22", 68, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_23 (case 69)
#[test]
fn conversion_declaration_explicit_implementation_23() {
    let src = r#"explicit I T(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I T(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_23", 69, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_24 (case 70)
#[test]
fn conversion_declaration_explicit_implementation_24() {
    let src = r#"explicit I.T(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.T(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_24", 70, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_25 (case 71)
#[test]
fn conversion_declaration_explicit_implementation_25() {
    let src = r#"explicit I.operator (int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_25", 71, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_27 (case 72)
#[test]
fn conversion_declaration_explicit_implementation_27() {
    let src = r#"explicit I.operator (int x);"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_27", 72, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_28 (case 73)
#[test]
fn conversion_declaration_explicit_implementation_28() {
    let src = r#"explicit I.T1 T2(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.T1 T2(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_28", 73, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_29 (case 74)
#[test]
fn conversion_declaration_explicit_implementation_29() {
    let src = r#"explicit I.operator (int x)"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x) }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_29", 74, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_30 (case 75)
#[test]
fn conversion_declaration_explicit_implementation_30() {
    let src = r#"explicit I.operator (int x, );"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x, ); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_30", 75, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_31 (case 76)
#[test]
fn conversion_declaration_explicit_implementation_31() {
    let src = r#"explicit I.operator (int x, int y);"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x, int y); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_31", 76, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_32 (case 77)
#[test]
fn conversion_declaration_explicit_implementation_32() {
    let src = r#"explicit I.operator var(x);"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator var(x); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_32", 77, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_33 (case 78)
#[test]
fn conversion_declaration_explicit_implementation_33() {
    let src = r#"explicit I.operator (int x int y);"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x int y); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_33", 78, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_34 (case 79)
#[test]
fn conversion_declaration_explicit_implementation_34() {
    let src = r#"explicit N.I..operator int(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { explicit N.I..operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_34", 79, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_01 (case 80)
#[test]
fn dot_dot_recovery_01() {
    let src = r#"N1..N2 M(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { N1..N2 M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_01", 80, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_02 (case 81)
#[test]
fn dot_dot_recovery_02() {
    let src = r#"int N1..M(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1..M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_02", 81, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_03 (case 82)
#[test]
fn dot_dot_recovery_03() {
    let src = r#"int N1.N2..M(int x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1.N2..M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_03", 82, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_01 (case 83)
#[test]
fn misplaced_colon_colon_01() {
    let src = r#"int N::I::M1() => 0;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N::I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_01", 83, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_02 (case 84)
#[test]
fn misplaced_colon_colon_02() {
    let src = r#"int N1::N2::I.M1() => 0;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2::I.M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_02", 84, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_03 (case 85)
#[test]
fn misplaced_colon_colon_03() {
    let src = r#"int N1::N2.I::M1() => 0;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2.I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_03", 85, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_04 (case 86)
#[test]
fn misplaced_colon_colon_04() {
    let src = r#"int I::M1() => 0;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_04", 86, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_05 (case 87)
#[test]
fn misplaced_colon_colon_05() {
    let src = r#"int N1::I.M1() => 0;"#;
    let span = Span::new(src);
    let src2 = r#"class C { int N1::I.M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_05", 87, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_01 (case 88)
#[test]
fn checked_operator_declaration_01() {
    let src = r#"C operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_01", 88, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_02 (case 89)
#[test]
fn checked_operator_declaration_02() {
    let src = r#"C I.operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_02", 89, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_03 (case 90)
#[test]
fn checked_operator_declaration_03() {
    let src = r#"C operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_03", 90, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_04 (case 91)
#[test]
fn checked_operator_declaration_04() {
    let src = r#"C I.operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_04", 91, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UncheckedOperatorDeclaration_01 (case 92)
#[test]
fn unchecked_operator_declaration_01() {
    let src = r#"C operator unchecked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_01", 92, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.UncheckedOperatorDeclaration_04 (case 93)
#[test]
fn unchecked_operator_declaration_04() {
    let src = r#"C I.operator unchecked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_04", 93, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.ReadonlyParameter3 (case 94)
#[test]
fn readonly_parameter_3() {
    let src = r#"
(ref readonly int i) => { }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
(ref readonly int i) => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter3", 94, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_01 (case 95)
#[test]
fn compound_assignment_declaration_01() {
    let src = r#"C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_01", 95, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_02 (case 96)
#[test]
fn compound_assignment_declaration_02() {
    let src = r#"C operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_02", 96, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_03 (case 97)
#[test]
fn compound_assignment_declaration_03() {
    let src = r#"C I.operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_03", 97, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_04 (case 98)
#[test]
fn compound_assignment_declaration_04() {
    let src = r#"C I.operator checked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_04", 98, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_05 (case 99)
#[test]
fn compound_assignment_declaration_05() {
    let src = r#"C operator > >=(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_05", 99, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_06 (case 100)
#[test]
fn compound_assignment_declaration_06() {
    let src = r#"C operator >> =(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_06", 100, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_07 (case 101)
#[test]
fn compound_assignment_declaration_07() {
    let src = r#"C operator > > =(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_07", 101, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_08 (case 102)
#[test]
fn compound_assignment_declaration_08() {
    let src = r#"C operator > >>=(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >>=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_08", 102, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_09 (case 103)
#[test]
fn compound_assignment_declaration_09() {
    let src = r#"C operator > > >=(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_09", 103, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_10 (case 104)
#[test]
fn compound_assignment_declaration_10() {
    let src = r#"C operator > > > =(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_10", 104, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_11 (case 105)
#[test]
fn compound_assignment_declaration_11() {
    let src = r#"C operator >> >=(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_11", 105, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_12 (case 106)
#[test]
fn compound_assignment_declaration_12() {
    let src = r#"C operator >> > =(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_12", 106, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_13 (case 107)
#[test]
fn compound_assignment_declaration_13() {
    let src = r#"C operator >>> =(C x) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>> =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_13", 107, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_14 (case 108)
#[test]
fn compound_assignment_declaration_14() {
    let src = r#"C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_14", 108, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_15 (case 109)
#[test]
fn compound_assignment_declaration_15() {
    let src = r#"C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_15", 109, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_16 (case 110)
#[test]
fn compound_assignment_declaration_16() {
    let src = r#"C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_16", 110, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_17 (case 111)
#[test]
fn compound_assignment_declaration_17() {
    let src = r#"C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_17", 111, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_18 (case 112)
#[test]
fn compound_assignment_declaration_18() {
    let src = r#"C operator unchecked "#;
    let span = Span::new(src);
    let src2 = r#"class C { C operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_18", 112, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_19_Partial (case 113)
#[test]
fn compound_assignment_declaration_19_partial() {
    let src = r#"partial C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { partial C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_19_Partial", 113, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_20_Partial (case 114)
#[test]
fn compound_assignment_declaration_20_partial() {
    let src = r#"partial void operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { partial void operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_20_Partial", 114, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.IncrementDeclaration_01_Partial (case 115)
#[test]
fn increment_declaration_01_partial() {
    let src = r#"partial C operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { partial C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_01_Partial", 115, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: MemberDeclarationParsingTests.IncrementDeclaration_02_Partial (case 116)
#[test]
fn increment_declaration_02_partial() {
    let src = r#"partial void operator "#;
    let span = Span::new(src);
    let src2 = r#"class C { partial void operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_02_Partial", 116, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

