// Auto-generated from Roslyn: PatternParsingTests2
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType1 (case 1)
#[test]
fn extended_property_subpattern_nullable_type_1() {
    let src = r#"e is { Prop: Type? }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType1", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType2 (case 2)
#[test]
fn extended_property_subpattern_nullable_type_2() {
    let src = r#"e is { Prop: Type? propVal }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? propVal }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType2", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType3 (case 3)
#[test]
fn extended_property_subpattern_nullable_type_3() {
    let src = r#"e is { Prop: Type? propVal, Prop2: int? val2 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? propVal, Prop2: int? val2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType3", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType6 (case 4)
#[test]
fn extended_property_subpattern_nullable_type_6() {
    let src = r#"e is { Prop: Type? t or AnotherType? a }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? t or AnotherType? a }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType6", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

