// Auto-generated from Roslyn: DeclarationExpressionTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: DeclarationExpressionTests.NullableTypeTest_01 (case 1)
#[test]
fn nullable_type_test_01() {
    let src = r#"if (e is int?) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "NullableTypeTest_01", 1, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_03 (case 2)
#[test]
fn nullable_type_test_03() {
    let src = r#"if (e is int? x) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "NullableTypeTest_03", 2, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.NullableTypeTest_04 (case 3)
#[test]
fn nullable_type_test_04() {
    let src = r#"if (e is int x ? true : false) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "NullableTypeTest_04", 3, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.UnderscoreInOldForeach_01 (case 4)
#[test]
fn underscore_in_old_foreach_01() {
    let src = r#"foreach (int _ in e) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "UnderscoreInOldForeach_01", 4, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.UnderscoreInOldForeach_02 (case 5)
#[test]
fn underscore_in_old_foreach_02() {
    let src = r#"foreach (var _ in e) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "UnderscoreInOldForeach_02", 5, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.TupleOnTheLeft (case 6)
#[test]
fn tuple_on_the_left() {
    let src = r#"(1, 2) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "TupleOnTheLeft", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.OutTuple_01 (case 7)
#[test]
fn out_tuple_01() {
    let src = r#"M(out (1, 2));"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "OutTuple_01", 7, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.OutTuple_02 (case 8)
#[test]
fn out_tuple_02() {
    let src = r#"M(out (x, y));"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "OutTuple_02", 8, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.OutTuple_03 (case 9)
#[test]
fn out_tuple_03() {
    let src = r#"M(out (1, 2).Field);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "OutTuple_03", 9, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.NamedTupleOnTheLeft (case 10)
#[test]
fn named_tuple_on_the_left() {
    let src = r#"(x: 1, y: 2) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "NamedTupleOnTheLeft", 10, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeclarationExpressionTests.InvokeMethodNamedVar (case 11)
#[test]
fn invoke_method_named_var() {
    let src = r#"var(1, 2) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("declaration_expression_tests", "DeclarationExpressionTests", "InvokeMethodNamedVar", 11, CaseData::Statement { ast: &ast, src });
}

