// Auto-generated from Roslyn: ImplicitObjectCreationParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnAnonymousObjectCreation (case 1)
#[test]
fn no_regression_on_anonymous_object_creation() {
    let src = r#"new{}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new{}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnAnonymousObjectCreation", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnConditional (case 2)
#[test]
fn no_regression_on_conditional() {
    let src = r#"new (a, b) ? x : y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new (a, b) ? x : y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnConditional", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestNoRegressionOnTupleArrayCreation (case 3)
#[test]
fn no_regression_on_tuple_array_creation() {
    let src = r#"new(x,y)[0]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(x,y)[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestNoRegressionOnTupleArrayCreation", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestInvalidTupleArrayCreation (case 4)
#[test]
fn invalid_tuple_array_creation() {
    let src = r#"new()[0]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new()[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestInvalidTupleArrayCreation", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestEmptyArgList (case 5)
#[test]
fn empty_arg_list() {
    let src = r#"new()"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyArgList", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestEmptyObjectInitializer (case 6)
#[test]
fn empty_object_initializer() {
    let src = r#"new(){}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(){}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestEmptyObjectInitializer", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestObjectInitializer (case 7)
#[test]
fn object_initializer() {
    let src = r#"new(1,2){x=y}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(1,2){x=y}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestObjectInitializer", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ImplicitObjectCreationParsingTests.TestCollectionInitializer (case 8)
#[test]
fn collection_initializer() {
    let src = r#"new(1){2}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new(1){2}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("implicit_object_creation_parsing_tests", "ImplicitObjectCreationParsingTests", "TestCollectionInitializer", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

