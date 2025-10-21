// Auto-generated from Roslyn: ValueTupleTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ValueTupleTests.TuplesInLambda (case 1)
#[test]
fn tuples_in_lambda() {
    let src = r#"
class C
{
    var x = ((string, string) a, (int, int) b) => { };
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TuplesInLambda", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ValueTupleTests.TuplesWithNamesInLambda (case 2)
#[test]
fn tuples_with_names_in_lambda() {
    let src = r#"
class C
{
    var x = ((string a, string) a, (int, int b) b) => { };
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TuplesWithNamesInLambda", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ValueTupleTests.TupleExpressionWithTooFewElements (case 3)
#[test]
fn tuple_expression_with_too_few_elements() {
    let src = r#"
class C
{
    object x = ((Alice: 1), ());
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TupleExpressionWithTooFewElements", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_01 (case 4)
#[test]
fn ternary_versus_declaration_01() {
    let src = r#"return (i, isValid ? Errors.IsValid : Errors.HasErrors);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TernaryVersusDeclaration_01", 4, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_02 (case 5)
#[test]
fn ternary_versus_declaration_02() {
    let src = r#"return (isValid ? Errors.IsValid : Errors.HasErrors, i);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TernaryVersusDeclaration_02", 5, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_03 (case 6)
#[test]
fn ternary_versus_declaration_03() {
    let src = r#"return (i, a < b, c > d);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TernaryVersusDeclaration_03", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_04 (case 7)
#[test]
fn ternary_versus_declaration_04() {
    let src = r#"return (i, a < b, c > d.x);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TernaryVersusDeclaration_04", 7, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ValueTupleTests.TernaryVersusDeclaration_05 (case 8)
#[test]
fn ternary_versus_declaration_05() {
    let src = r#"return (i, a < b, c > d && x);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("value_tuple_tests", "ValueTupleTests", "TernaryVersusDeclaration_05", 8, CaseData::Statement { ast: &ast, src });
}

