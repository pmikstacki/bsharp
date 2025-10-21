// Auto-generated from Roslyn: FieldKeywordParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: FieldKeywordParsingTests.Incremental_ChangeBetweenMethodAndProperty (case 1)
#[test]
fn incremental_change_between_method_and_property() {
    let src = r#"
                class C
                {
                    object F() => field;
                }
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("field_keyword_parsing_tests", "FieldKeywordParsingTests", "Incremental_ChangeBetweenMethodAndProperty", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: FieldKeywordParsingTests.Incremental_ChangeBetweenMethodAndProperty (case 2)
#[test]
fn incremental_change_between_method_and_property_case_2() {
    let src = r#"
                class C
                {
                    object F => field;
                }
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("field_keyword_parsing_tests", "FieldKeywordParsingTests", "Incremental_ChangeBetweenMethodAndProperty", 2, CaseData::File { unit: &unit, src, original: None });
}

