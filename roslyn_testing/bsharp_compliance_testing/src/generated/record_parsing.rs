// Auto-generated from Roslyn: RecordParsing
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: RecordParsing.AbstractMethod_ConstraintsAndSemiColon (case 1)
#[test]
fn abstract_method_constraints_and_semi_colon() {
    let src = r#"abstract record R { abstract void M<T>() where T : class; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("record_parsing", "RecordParsing", "AbstractMethod_ConstraintsAndSemiColon", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: RecordParsing.WithParsing11 (case 2)
#[test]
fn with_parsing_11() {
    let src = r#"await with;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("record_parsing", "RecordParsing", "WithParsing11", 2, CaseData::Statement { ast: &ast, src });
}

