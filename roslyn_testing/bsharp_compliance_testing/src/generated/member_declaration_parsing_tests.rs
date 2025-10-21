// Auto-generated from Roslyn: MemberDeclarationParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: MemberDeclarationParsingTests.ParsePrivate (case 1)
#[test]
fn parse_private() {
    let src = r#"private"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { private }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ParsePrivate", 1, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ParsePrivate", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ParsePrivate", 1, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.Statement (case 2)
#[test]
fn statement() {
    let src = r#"x = x + 1;"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { x = x + 1; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Statement", 2, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Statement", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Statement", 2, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.Namespace (case 3)
#[test]
fn namespace() {
    let src = r#"namespace ns {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"namespace ns {}"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Namespace", 3, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Namespace", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "Namespace", 3, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.TypeDeclaration (case 4)
#[test]
fn type_declaration() {
    let src = r#"class C { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TypeDeclaration", 4, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TypeDeclaration", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TypeDeclaration", 4, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MethodDeclaration (case 5)
#[test]
fn method_declaration() {
    let src = r#"void M() { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MethodDeclaration", 5, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MethodDeclaration", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MethodDeclaration", 5, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.FieldDeclaration (case 6)
#[test]
fn field_declaration() {
    let src = r#"static int F1 = a, F2 = b;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { static int F1 = a, F2 = b; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "FieldDeclaration", 6, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "FieldDeclaration", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "FieldDeclaration", 6, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CtorDeclaration (case 7)
#[test]
fn ctor_declaration() {
    let src = r#"public ThisClassName(int x) : base(x) { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public ThisClassName(int x) : base(x) { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CtorDeclaration", 7, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CtorDeclaration", 7, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CtorDeclaration", 7, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.DtorDeclaration (case 8)
#[test]
fn dtor_declaration() {
    let src = r#"public ~ThisClassName() { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public ~ThisClassName() { } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DtorDeclaration", 8, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DtorDeclaration", 8, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DtorDeclaration", 8, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration (case 9)
#[test]
fn conversion_declaration() {
    let src = r#"public implicit operator long(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public implicit operator long(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration", 9, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration", 9, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration", 9, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration (case 10)
#[test]
fn operator_declaration() {
    let src = r#"public int operator +(int x, int y) => x + y;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public int operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration", 10, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration", 10, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration", 10, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_01 (case 11)
#[test]
fn unsigned_right_shift_operator_01() {
    let src = r#"C operator >>>(C x, C y) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>>(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_01", 11, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_01", 11, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_01", 11, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_02 (case 12)
#[test]
fn unsigned_right_shift_operator_02() {
    let src = r#"C operator > >>(C x, C y) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 7, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >>(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_02", 12, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_02", 12, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_02", 12, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_03 (case 13)
#[test]
fn unsigned_right_shift_operator_03() {
    let src = r#"C operator >> >(C x, C y) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 7, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> >(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_03", 13, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_03", 13, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_03", 13, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UnsignedRightShiftOperator_04 (case 14)
#[test]
fn unsigned_right_shift_operator_04() {
    let src = r#"C operator >>>=(C x, C y) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>>=(C x, C y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_04", 14, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_04", 14, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UnsignedRightShiftOperator_04", 14, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.TrashAfterDeclaration (case 15)
#[test]
fn trash_after_declaration() {
    let src = r#"public int x; public int y"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int x; public int y }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 15, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 15, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 15, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.TrashAfterDeclaration (case 16)
#[test]
fn trash_after_declaration_case_2() {
    let src = r#"public int x; public int y"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public int x; public int y }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 16, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 16, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "TrashAfterDeclaration", 16, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_01 (case 17)
#[test]
fn generic_async_task_01() {
    let src = r#"async Task<SomeNamespace.SomeType Method();"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace.SomeType Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_01", 17, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_01", 17, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_01", 17, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_01 (case 18)
#[test]
fn generic_public_task_01() {
    let src = r#"public Task<SomeNamespace.SomeType Method();"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace.SomeType Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_01", 18, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_01", 18, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_01", 18, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_02 (case 19)
#[test]
fn generic_async_task_02() {
    let src = r#"async Task<SomeNamespace. Method();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace. Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_02", 19, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_02", 19, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_02", 19, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_02 (case 20)
#[test]
fn generic_public_task_02() {
    let src = r#"public Task<SomeNamespace. Method();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace. Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_02", 20, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_02", 20, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_02", 20, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericAsyncTask_03 (case 21)
#[test]
fn generic_async_task_03() {
    let src = r#"async Task<SomeNamespace.> Method();"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { async Task<SomeNamespace.> Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_03", 21, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_03", 21, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericAsyncTask_03", 21, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.GenericPublicTask_03 (case 22)
#[test]
fn generic_public_task_03() {
    let src = r#"public Task<SomeNamespace.> Method();"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public Task<SomeNamespace.> Method(); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_03", 22, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_03", 22, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "GenericPublicTask_03", 22, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.InitAccessor (case 23)
#[test]
fn init_accessor() {
    let src = r#"string Property { get; init; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { string Property { get; init; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAccessor", 23, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAccessor", 23, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAccessor", 23, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.InitSetAccessor (case 24)
#[test]
fn init_set_accessor() {
    let src = r#"string Property { init set; }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { string Property { init set; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitSetAccessor", 24, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitSetAccessor", 24, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitSetAccessor", 24, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.InitAndSetAccessor (case 25)
#[test]
fn init_and_set_accessor() {
    let src = r#"string Property { init; set; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { string Property { init; set; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAndSetAccessor", 25, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAndSetAccessor", 25, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "InitAndSetAccessor", 25, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.SetAndInitAccessor (case 26)
#[test]
fn set_and_init_accessor() {
    let src = r#"string Property { set; init; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { string Property { set; init; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "SetAndInitAccessor", 26, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "SetAndInitAccessor", 26, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "SetAndInitAccessor", 26, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_01 (case 27)
#[test]
fn required_modifier_property_01() {
    let src = r#"required string Prop { get; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_01", 27, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_01", 27, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_01", 27, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_02 (case 28)
#[test]
fn required_modifier_property_02() {
    let src = r#"required Prop { get; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required Prop { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_02", 28, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_02", 28, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_02", 28, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_03 (case 29)
#[test]
fn required_modifier_property_03() {
    let src = r#"required Prop { get; }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required Prop { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_03", 29, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_03", 29, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_03", 29, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_04 (case 30)
#[test]
fn required_modifier_property_04() {
    let src = r#"required required { get; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required required { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_04", 30, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_04", 30, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_04", 30, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_05 (case 31)
#[test]
fn required_modifier_property_05() {
    let src = r#"required required { get; }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required required { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_05", 31, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_05", 31, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_05", 31, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_06 (case 32)
#[test]
fn required_modifier_property_06() {
    let src = r#"required required Prop { get; }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required required Prop { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_06", 32, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_06", 32, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_06", 32, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierProperty_07 (case 33)
#[test]
fn required_modifier_property_07() {
    let src = r#"required Type required { get; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required Type required { get; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_07", 33, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_07", 33, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierProperty_07", 33, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_01 (case 34)
#[test]
fn required_modifier_field_01() {
    let src = r#"required string Field;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required string Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_01", 34, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_01", 34, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_01", 34, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_02 (case 35)
#[test]
fn required_modifier_field_02() {
    let src = r#"required Field;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_02", 35, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_02", 35, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_02", 35, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_03 (case 36)
#[test]
fn required_modifier_field_03() {
    let src = r#"required Field;"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required Field; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_03", 36, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_03", 36, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_03", 36, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_04 (case 37)
#[test]
fn required_modifier_field_04() {
    let src = r#"required required;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required required; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_04", 37, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_04", 37, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_04", 37, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierField_05 (case 38)
#[test]
fn required_modifier_field_05() {
    let src = r#"required required;"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required required; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_05", 38, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_05", 38, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierField_05", 38, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_01 (case 39)
#[test]
fn required_modifier_method_01() {
    let src = r#"required string M() {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required string M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_01", 39, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_01", 39, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_01", 39, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_02 (case 40)
#[test]
fn required_modifier_method_02() {
    let src = r#"required M() {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_02", 40, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_02", 40, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_02", 40, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierMethod_03 (case 41)
#[test]
fn required_modifier_method_03() {
    let src = r#"required M() {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { required M() {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_03", 41, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_03", 41, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierMethod_03", 41, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierOperator (case 42)
#[test]
fn required_modifier_operator() {
    let src = r#"static required C operator+(C c1, C c2) {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { static required C operator+(C c1, C c2) {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierOperator", 42, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierOperator", 42, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierOperator", 42, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierConversion_01 (case 43)
#[test]
fn required_modifier_conversion_01() {
    let src = r#"static required implicit operator C(S s) {}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { static required implicit operator C(S s) {} }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierConversion_01", 43, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierConversion_01", 43, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierConversion_01", 43, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteProperty_01 (case 44)
#[test]
fn required_modifier_incomplete_property_01() {
    let src = r#"required string Prop { get;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop { get; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_01", 44, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_01", 44, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_01", 44, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteProperty_02 (case 45)
#[test]
fn required_modifier_incomplete_property_02() {
    let src = r#"required string Prop {"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop { }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_02", 45, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_02", 45, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteProperty_02", 45, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_01 (case 46)
#[test]
fn required_modifier_incomplete_member_01() {
    let src = r#"required string Prop"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required string Prop }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_01", 46, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_01", 46, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_01", 46, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_02 (case 47)
#[test]
fn required_modifier_incomplete_member_02() {
    let src = r#"required string"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required string }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_02", 47, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_02", 47, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_02", 47, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_03 (case 48)
#[test]
fn required_modifier_incomplete_member_03() {
    let src = r#"required C"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required C }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_03", 48, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_03", 48, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_03", 48, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_04 (case 49)
#[test]
fn required_modifier_incomplete_member_04() {
    let src = r#"required"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_04", 49, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_04", 49, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_04", 49, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifierIncompleteMember_05 (case 50)
#[test]
fn required_modifier_incomplete_member_05() {
    let src = r#"required"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { required }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_05", 50, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_05", 50, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifierIncompleteMember_05", 50, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.RequiredModifier_LocalNamedRequired_TopLevelStatements (case 51)
#[test]
fn required_modifier_local_named_required_top_level_statements() {
    let src = r#"
                bool required;
                required = true;
                "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifier_LocalNamedRequired_TopLevelStatements", 51, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifier_LocalNamedRequired_TopLevelStatements", 51, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "RequiredModifier_LocalNamedRequired_TopLevelStatements", 51, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_06 (case 52)
#[test]
fn operator_declaration_explicit_implementation_06() {
    let src = r#"public int N::I::operator +(int x, int y) => x + y;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int N::I::operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_06", 52, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_06", 52, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_06", 52, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_07 (case 53)
#[test]
fn operator_declaration_explicit_implementation_07() {
    let src = r#"public int I::operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int I::operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_07", 53, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_07", 53, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_07", 53, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_08 (case 54)
#[test]
fn operator_declaration_explicit_implementation_08() {
    let src = r#"public int I.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public int I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_08", 54, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_08", 54, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_08", 54, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_09 (case 55)
#[test]
fn operator_declaration_explicit_implementation_09() {
    let src = r#"public int I<T>.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { public int I<T>.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_09", 55, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_09", 55, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_09", 55, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_10 (case 56)
#[test]
fn operator_declaration_explicit_implementation_10() {
    let src = r#"public int N1::N2::I.operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int N1::N2::I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_10", 56, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_10", 56, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_10", 56, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_16 (case 57)
#[test]
fn operator_declaration_explicit_implementation_16() {
    let src = r#"public int N::I::operator +(int x, int y) => x + y;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_16", 57, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_16", 57, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_16", 57, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_17 (case 58)
#[test]
fn operator_declaration_explicit_implementation_17() {
    let src = r#"public int I::operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_17", 58, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_17", 58, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_17", 58, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_18 (case 59)
#[test]
fn operator_declaration_explicit_implementation_18() {
    let src = r#"public int I.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_18", 59, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_18", 59, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_18", 59, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_19 (case 60)
#[test]
fn operator_declaration_explicit_implementation_19() {
    let src = r#"public int I<T>.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_19", 60, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_19", 60, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_19", 60, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_20 (case 61)
#[test]
fn operator_declaration_explicit_implementation_20() {
    let src = r#"public int N1::N2::I.operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_20", 61, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_20", 61, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_20", 61, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_21 (case 62)
#[test]
fn operator_declaration_explicit_implementation_21() {
    let src = r#"public int I..operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_21", 62, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_21", 62, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_21", 62, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_22 (case 63)
#[test]
fn operator_declaration_explicit_implementation_22() {
    let src = r#"public int I . . operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { public int I . . operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_22", 63, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_22", 63, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_22", 63, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_28 (case 64)
#[test]
fn operator_declaration_explicit_implementation_28() {
    let src = r#"int N::I::operator +(int x, int y) => x + y;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N::I::operator +(int x, int y) => x + y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_28", 64, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_28", 64, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_28", 64, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_29 (case 65)
#[test]
fn operator_declaration_explicit_implementation_29() {
    let src = r#"int I::operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int I::operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_29", 65, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_29", 65, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_29", 65, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_30 (case 66)
#[test]
fn operator_declaration_explicit_implementation_30() {
    let src = r#"int I.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { int I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_30", 66, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_30", 66, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_30", 66, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_31 (case 67)
#[test]
fn operator_declaration_explicit_implementation_31() {
    let src = r#"int I<T>.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { int I<T>.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_31", 67, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_31", 67, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_31", 67, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_32 (case 68)
#[test]
fn operator_declaration_explicit_implementation_32() {
    let src = r#"int N1::N2::I.operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2::I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_32", 68, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_32", 68, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_32", 68, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_38 (case 69)
#[test]
fn operator_declaration_explicit_implementation_38() {
    let src = r#"int N::I::operator +(int x, int y) => x + y;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_38", 69, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_38", 69, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_38", 69, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_39 (case 70)
#[test]
fn operator_declaration_explicit_implementation_39() {
    let src = r#"int I::operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_39", 70, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_39", 70, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_39", 70, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_40 (case 71)
#[test]
fn operator_declaration_explicit_implementation_40() {
    let src = r#"int I.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_40", 71, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_40", 71, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_40", 71, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_41 (case 72)
#[test]
fn operator_declaration_explicit_implementation_41() {
    let src = r#"int I<T>.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_41", 72, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_41", 72, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_41", 72, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_42 (case 73)
#[test]
fn operator_declaration_explicit_implementation_42() {
    let src = r#"int N1::N2::I.operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_42", 73, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_42", 73, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_42", 73, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_43 (case 74)
#[test]
fn operator_declaration_explicit_implementation_43() {
    let src = r#"int I..operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_43", 74, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_43", 74, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_43", 74, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_44 (case 75)
#[test]
fn operator_declaration_explicit_implementation_44() {
    let src = r#"int I . . operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int I . . operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_44", 75, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_44", 75, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_44", 75, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_45 (case 76)
#[test]
fn operator_declaration_explicit_implementation_45() {
    let src = r#"int N.I..operator +(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N.I..operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_45", 76, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_45", 76, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_45", 76, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_46 (case 77)
#[test]
fn operator_declaration_explicit_implementation_46() {
    let src = r#"N.I.operator +(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { N.I.operator +(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_46", 77, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_46", 77, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_46", 77, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.OperatorDeclaration_ExplicitImplementation_47 (case 78)
#[test]
fn operator_declaration_explicit_implementation_47() {
    let src = r#"N.I. int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { N.I. int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_47", 78, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_47", 78, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "OperatorDeclaration_ExplicitImplementation_47", 78, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_03 (case 79)
#[test]
fn conversion_declaration_explicit_implementation_03() {
    let src = r#"operator int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_03", 79, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_03", 79, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_03", 79, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_06 (case 80)
#[test]
fn conversion_declaration_explicit_implementation_06() {
    let src = r#"implicit N::I::operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { implicit N::I::operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_06", 80, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_06", 80, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_06", 80, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_07 (case 81)
#[test]
fn conversion_declaration_explicit_implementation_07() {
    let src = r#"explicit I::operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I::operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_07", 81, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_07", 81, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_07", 81, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_08 (case 82)
#[test]
fn conversion_declaration_explicit_implementation_08() {
    let src = r#"implicit I.operator int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { implicit I.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_08", 82, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_08", 82, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_08", 82, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_09 (case 83)
#[test]
fn conversion_declaration_explicit_implementation_09() {
    let src = r#"explicit I<T>.operator int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { explicit I<T>.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_09", 83, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_09", 83, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_09", 83, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_10 (case 84)
#[test]
fn conversion_declaration_explicit_implementation_10() {
    let src = r#"implicit N1::N2::I.operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { implicit N1::N2::I.operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_10", 84, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_10", 84, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_10", 84, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_16 (case 85)
#[test]
fn conversion_declaration_explicit_implementation_16() {
    let src = r#"implicit N::I::operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_16", 85, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_16", 85, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_16", 85, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_17 (case 86)
#[test]
fn conversion_declaration_explicit_implementation_17() {
    let src = r#"explicit I::operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_17", 86, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_17", 86, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_17", 86, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_18 (case 87)
#[test]
fn conversion_declaration_explicit_implementation_18() {
    let src = r#"implicit I.operator int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_18", 87, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_18", 87, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_18", 87, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_19 (case 88)
#[test]
fn conversion_declaration_explicit_implementation_19() {
    let src = r#"explicit I<T>.operator int(int x) => x;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_19", 88, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_19", 88, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_19", 88, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_20 (case 89)
#[test]
fn conversion_declaration_explicit_implementation_20() {
    let src = r#"implicit N1::N2::I.operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_20", 89, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_20", 89, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_20", 89, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_21 (case 90)
#[test]
fn conversion_declaration_explicit_implementation_21() {
    let src = r#"explicit I..operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I..operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_21", 90, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_21", 90, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_21", 90, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_22 (case 91)
#[test]
fn conversion_declaration_explicit_implementation_22() {
    let src = r#"implicit I . . operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { implicit I . . operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_22", 91, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_22", 91, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_22", 91, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_23 (case 92)
#[test]
fn conversion_declaration_explicit_implementation_23() {
    let src = r#"explicit I T(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I T(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_23", 92, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_23", 92, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_23", 92, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_24 (case 93)
#[test]
fn conversion_declaration_explicit_implementation_24() {
    let src = r#"explicit I.T(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.T(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_24", 93, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_24", 93, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_24", 93, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_25 (case 94)
#[test]
fn conversion_declaration_explicit_implementation_25() {
    let src = r#"explicit I.operator (int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_25", 94, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_25", 94, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_25", 94, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_26 (case 95)
#[test]
fn conversion_declaration_explicit_implementation_26() {
    let src = r#"explicit I.operator (int x) { return x; }"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x) { return x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_26", 95, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_26", 95, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_26", 95, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_27 (case 96)
#[test]
fn conversion_declaration_explicit_implementation_27() {
    let src = r#"explicit I.operator (int x);"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_27", 96, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_27", 96, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_27", 96, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_28 (case 97)
#[test]
fn conversion_declaration_explicit_implementation_28() {
    let src = r#"explicit I.T1 T2(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.T1 T2(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_28", 97, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_28", 97, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_28", 97, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_29 (case 98)
#[test]
fn conversion_declaration_explicit_implementation_29() {
    let src = r#"explicit I.operator (int x)"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x) }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_29", 98, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_29", 98, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_29", 98, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_30 (case 99)
#[test]
fn conversion_declaration_explicit_implementation_30() {
    let src = r#"explicit I.operator (int x, );"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x, ); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_30", 99, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_30", 99, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_30", 99, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_31 (case 100)
#[test]
fn conversion_declaration_explicit_implementation_31() {
    let src = r#"explicit I.operator (int x, int y);"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x, int y); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_31", 100, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_31", 100, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_31", 100, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_32 (case 101)
#[test]
fn conversion_declaration_explicit_implementation_32() {
    let src = r#"explicit I.operator var(x);"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator var(x); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_32", 101, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_32", 101, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_32", 101, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_33 (case 102)
#[test]
fn conversion_declaration_explicit_implementation_33() {
    let src = r#"explicit I.operator (int x int y);"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit I.operator (int x int y); }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_33", 102, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_33", 102, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_33", 102, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_34 (case 103)
#[test]
fn conversion_declaration_explicit_implementation_34() {
    let src = r#"explicit N.I..operator int(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { explicit N.I..operator int(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_34", 103, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_34", 103, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_34", 103, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ConversionDeclaration_ExplicitImplementation_35 (case 104)
#[test]
fn conversion_declaration_explicit_implementation_35() {
    let src = r#"
explicit
Func<int, int> f1 = (param1) => 10;
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_35", 104, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_35", 104, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ConversionDeclaration_ExplicitImplementation_35", 104, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_01 (case 105)
#[test]
fn dot_dot_recovery_01() {
    let src = r#"N1..N2 M(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { N1..N2 M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_01", 105, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_01", 105, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_01", 105, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_02 (case 106)
#[test]
fn dot_dot_recovery_02() {
    let src = r#"int N1..M(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N1..M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_02", 106, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_02", 106, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_02", 106, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.DotDotRecovery_03 (case 107)
#[test]
fn dot_dot_recovery_03() {
    let src = r#"int N1.N2..M(int x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N1.N2..M(int x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_03", 107, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_03", 107, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "DotDotRecovery_03", 107, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_01 (case 108)
#[test]
fn misplaced_colon_colon_01() {
    let src = r#"int N::I::M1() => 0;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N::I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_01", 108, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_01", 108, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_01", 108, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_02 (case 109)
#[test]
fn misplaced_colon_colon_02() {
    let src = r#"int N1::N2::I.M1() => 0;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2::I.M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_02", 109, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_02", 109, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_02", 109, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_03 (case 110)
#[test]
fn misplaced_colon_colon_03() {
    let src = r#"int N1::N2.I::M1() => 0;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int N1::N2.I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_03", 110, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_03", 110, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_03", 110, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_04 (case 111)
#[test]
fn misplaced_colon_colon_04() {
    let src = r#"int I::M1() => 0;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { int I::M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_04", 111, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_04", 111, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_04", 111, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.MisplacedColonColon_05 (case 112)
#[test]
fn misplaced_colon_colon_05() {
    let src = r#"int N1::I.M1() => 0;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { int N1::I.M1() => 0; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_05", 112, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_05", 112, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "MisplacedColonColon_05", 112, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_01 (case 113)
#[test]
fn checked_operator_declaration_01() {
    let src = r#"C operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_01", 113, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_01", 113, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_01", 113, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_02 (case 114)
#[test]
fn checked_operator_declaration_02() {
    let src = r#"C I.operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_02", 114, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_02", 114, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_02", 114, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_03 (case 115)
#[test]
fn checked_operator_declaration_03() {
    let src = r#"C operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_03", 115, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_03", 115, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_03", 115, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CheckedOperatorDeclaration_04 (case 116)
#[test]
fn checked_operator_declaration_04() {
    let src = r#"C I.operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_04", 116, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_04", 116, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CheckedOperatorDeclaration_04", 116, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UncheckedOperatorDeclaration_01 (case 117)
#[test]
fn unchecked_operator_declaration_01() {
    let src = r#"C operator unchecked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_01", 117, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_01", 117, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_01", 117, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.UncheckedOperatorDeclaration_04 (case 118)
#[test]
fn unchecked_operator_declaration_04() {
    let src = r#"C I.operator unchecked "#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_04", 118, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_04", 118, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "UncheckedOperatorDeclaration_04", 118, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ReadonlyParameter1 (case 119)
#[test]
fn readonly_parameter_1() {
    let src = r#"
public class Base {
    public virtual void M(ref int X) {
    }
}
public class Derived : Base {
    public override void M(ref readonly int X) {
    }
}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter1", 119, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter1", 119, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter1", 119, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ReadonlyParameter2 (case 120)
#[test]
fn readonly_parameter_2() {
    let src = r#"
(readonly int i) => { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
(readonly int i) => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter2", 120, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter2", 120, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter2", 120, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ReadonlyParameter3 (case 121)
#[test]
fn readonly_parameter_3() {
    let src = r#"
(ref readonly int i) => { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
(ref readonly int i) => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter3", 121, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter3", 121, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter3", 121, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.ReadonlyParameter4 (case 122)
#[test]
fn readonly_parameter_4() {
    let src = r#"
(readonly ref int i) => { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
(readonly ref int i) => { }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter4", 122, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter4", 122, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "ReadonlyParameter4", 122, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_01 (case 123)
#[test]
fn compound_assignment_declaration_01() {
    let src = r#"C operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_01", 123, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_01", 123, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_01", 123, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_02 (case 124)
#[test]
fn compound_assignment_declaration_02() {
    let src = r#"C operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_02", 124, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_02", 124, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_02", 124, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_03 (case 125)
#[test]
fn compound_assignment_declaration_03() {
    let src = r#"C I.operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_03", 125, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_03", 125, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_03", 125, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_04 (case 126)
#[test]
fn compound_assignment_declaration_04() {
    let src = r#"C I.operator checked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C I.operator checked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_04", 126, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_04", 126, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_04", 126, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_05 (case 127)
#[test]
fn compound_assignment_declaration_05() {
    let src = r#"C operator > >=(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_05", 127, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_05", 127, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_05", 127, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_06 (case 128)
#[test]
fn compound_assignment_declaration_06() {
    let src = r#"C operator >> =(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_06", 128, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_06", 128, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_06", 128, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_07 (case 129)
#[test]
fn compound_assignment_declaration_07() {
    let src = r#"C operator > > =(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_07", 129, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_07", 129, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_07", 129, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_08 (case 130)
#[test]
fn compound_assignment_declaration_08() {
    let src = r#"C operator > >>=(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > >>=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_08", 130, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_08", 130, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_08", 130, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_09 (case 131)
#[test]
fn compound_assignment_declaration_09() {
    let src = r#"C operator > > >=(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_09", 131, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_09", 131, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_09", 131, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_10 (case 132)
#[test]
fn compound_assignment_declaration_10() {
    let src = r#"C operator > > > =(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator > > > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_10", 132, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_10", 132, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_10", 132, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_11 (case 133)
#[test]
fn compound_assignment_declaration_11() {
    let src = r#"C operator >> >=(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> >=(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_11", 133, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_11", 133, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_11", 133, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_12 (case 134)
#[test]
fn compound_assignment_declaration_12() {
    let src = r#"C operator >> > =(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >> > =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_12", 134, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_12", 134, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_12", 134, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_13 (case 135)
#[test]
fn compound_assignment_declaration_13() {
    let src = r#"C operator >>> =(C x) => x;"#;
    let expected = Some(ExpectedDiagnostics { count: 8, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator >>> =(C x) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_13", 135, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_13", 135, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_13", 135, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_14 (case 136)
#[test]
fn compound_assignment_declaration_14() {
    let src = r#"C operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_14", 136, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_14", 136, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_14", 136, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_15 (case 137)
#[test]
fn compound_assignment_declaration_15() {
    let src = r#"C operator "#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_15", 137, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_15", 137, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_15", 137, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_16 (case 138)
#[test]
fn compound_assignment_declaration_16() {
    let src = r#"C operator "#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_16", 138, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_16", 138, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_16", 138, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_17 (case 139)
#[test]
fn compound_assignment_declaration_17() {
    let src = r#"C operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_17", 139, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_17", 139, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_17", 139, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_18 (case 140)
#[test]
fn compound_assignment_declaration_18() {
    let src = r#"C operator unchecked "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { C operator unchecked  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_18", 140, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_18", 140, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_18", 140, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_19_Partial (case 141)
#[test]
fn compound_assignment_declaration_19_partial() {
    let src = r#"partial C operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { partial C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_19_Partial", 141, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_19_Partial", 141, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_19_Partial", 141, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.CompoundAssignmentDeclaration_20_Partial (case 142)
#[test]
fn compound_assignment_declaration_20_partial() {
    let src = r#"partial void operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { partial void operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_20_Partial", 142, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_20_Partial", 142, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "CompoundAssignmentDeclaration_20_Partial", 142, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.IncrementDeclaration_01_Partial (case 143)
#[test]
fn increment_declaration_01_partial() {
    let src = r#"partial C operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { partial C operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_01_Partial", 143, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_01_Partial", 143, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_01_Partial", 143, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: MemberDeclarationParsingTests.IncrementDeclaration_02_Partial (case 144)
#[test]
fn increment_declaration_02_partial() {
    let src = r#"partial void operator "#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { partial void operator  }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_02_Partial", 144, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_02_Partial", 144, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("member_declaration_parsing_tests", "MemberDeclarationParsingTests", "IncrementDeclaration_02_Partial", 144, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

