// Auto-generated from Roslyn: ImplicitObjectCreationParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnNew (case 1)
#[test]
fn no_regression_on_new() {
    let src = r#"new"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNew", 1, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNew", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNew", 1, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnNullableTuple (case 2)
#[test]
fn no_regression_on_nullable_tuple() {
    let src = r#"new(Int32,Int32)?()"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(Int32,Int32)?(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNullableTuple", 2, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNullableTuple", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnNullableTuple", 2, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnImplicitArrayCreation (case 3)
#[test]
fn no_regression_on_implicit_array_creation() {
    let src = r#"new[]"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnImplicitArrayCreation", 3, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnImplicitArrayCreation", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnImplicitArrayCreation", 3, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnAnonymousObjectCreation (case 4)
#[test]
fn no_regression_on_anonymous_object_creation() {
    let src = r#"new{}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new{}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnAnonymousObjectCreation", 4, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnAnonymousObjectCreation", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnAnonymousObjectCreation", 4, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnConditional (case 5)
#[test]
fn no_regression_on_conditional() {
    let src = r#"new (a, b) ? x : y"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new (a, b) ? x : y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnConditional", 5, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnConditional", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnConditional", 5, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnTupleArrayCreation (case 6)
#[test]
fn no_regression_on_tuple_array_creation() {
    let src = r#"new(x,y)[0]"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(x,y)[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnTupleArrayCreation", 6, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnTupleArrayCreation", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnTupleArrayCreation", 6, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestInvalidTupleCreation (case 7)
#[test]
fn invalid_tuple_creation() {
    let src = r#"new(int,int)()"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(int,int)(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleCreation", 7, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleCreation", 7, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleCreation", 7, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestInvalidTupleArrayCreation (case 8)
#[test]
fn invalid_tuple_array_creation() {
    let src = r#"new()[0]"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new()[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleArrayCreation", 8, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleArrayCreation", 8, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleArrayCreation", 8, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestEmptyArgList (case 9)
#[test]
fn empty_arg_list() {
    let src = r#"new()"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyArgList", 9, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyArgList", 9, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyArgList", 9, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestEmptyObjectInitializer (case 10)
#[test]
fn empty_object_initializer() {
    let src = r#"new(){}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(){}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyObjectInitializer", 10, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyObjectInitializer", 10, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyObjectInitializer", 10, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestObjectInitializer (case 11)
#[test]
fn object_initializer() {
    let src = r#"new(1,2){x=y}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(1,2){x=y}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestObjectInitializer", 11, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestObjectInitializer", 11, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestObjectInitializer", 11, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestCollectionInitializer (case 12)
#[test]
fn collection_initializer() {
    let src = r#"new(1){2}"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(1){2}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestCollectionInitializer", 12, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestCollectionInitializer", 12, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestCollectionInitializer", 12, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

