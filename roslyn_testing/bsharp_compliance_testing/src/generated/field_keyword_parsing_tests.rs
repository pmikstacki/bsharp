// Auto-generated from Roslyn: FieldKeywordParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
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
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "field_keyword_parsing_tests",
                "FieldKeywordParsingTests",
                "Incremental_ChangeBetweenMethodAndProperty",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
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
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "field_keyword_parsing_tests",
                "FieldKeywordParsingTests",
                "Incremental_ChangeBetweenMethodAndProperty",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
