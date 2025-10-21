// Auto-generated from Roslyn: PatternParsingTests2
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_02 (case 1)
#[test]
fn extended_property_subpattern_02() {
    let src = r#"e is { {}: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { {}: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_02", 1, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_02", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_02", 1, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_03 (case 2)
#[test]
fn extended_property_subpattern_03() {
    let src = r#"e is { name<T>: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { name<T>: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_03", 2, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_03", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_03", 2, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_04 (case 3)
#[test]
fn extended_property_subpattern_04() {
    let src = r#"e is { name[0]: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { name[0]: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_04", 3, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_04", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_04", 3, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_05 (case 4)
#[test]
fn extended_property_subpattern_05() {
    let src = r#"e is { a?.b: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { a?.b: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_05", 4, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_05", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_05", 4, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_06 (case 5)
#[test]
fn extended_property_subpattern_06() {
    let src = r#"e is { a->c: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { a->c: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_06", 5, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_06", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_06", 5, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_07 (case 6)
#[test]
fn extended_property_subpattern_07() {
    let src = r#"e is { [0]: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { [0]: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_07", 6, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_07", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_07", 6, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_08 (case 7)
#[test]
fn extended_property_subpattern_08() {
    let src = r#"e is { not a: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { not a: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_08", 7, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_08", 7, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_08", 7, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_09 (case 8)
#[test]
fn extended_property_subpattern_09() {
    let src = r#"e is { x or y: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { x or y: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_09", 8, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_09", 8, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_09", 8, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_10 (case 9)
#[test]
fn extended_property_subpattern_10() {
    let src = r#"e is { 1: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { 1: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_10", 9, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_10", 9, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_10", 9, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_11 (case 10)
#[test]
fn extended_property_subpattern_11() {
    let src = r#"e is { >1: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { >1: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_11", 10, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_11", 10, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_11", 10, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_12 (case 11)
#[test]
fn extended_property_subpattern_12() {
    let src = r#"e is { a!.b: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { a!.b: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_12", 11, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_12", 11, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_12", 11, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_13 (case 12)
#[test]
fn extended_property_subpattern_13() {
    let src = r#"e is { a[0].b: p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { a[0].b: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_13", 12, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_13", 12, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_13", 12, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_14 (case 13)
#[test]
fn extended_property_subpattern_14() {
    let src = r#"e is { [0].b: p }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { [0].b: p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_14", 13, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_14", 13, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_14", 13, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_15 (case 14)
#[test]
fn extended_property_subpattern_15() {
    let src = r#"e is { (c?a:b): p }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { (c?a:b): p }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_15", 14, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_15", 14, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_15", 14, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_InPositionalPattern (case 15)
#[test]
fn extended_property_subpattern_in_positional_pattern() {
    let src = r#"e is ( a.b.c: p )"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is ( a.b.c: p ); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_InPositionalPattern", 15, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_InPositionalPattern", 15, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_InPositionalPattern", 15, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket01 (case 16)
#[test]
fn missing_closing_angle_bracket_01() {
    let src = r#"e is List<int"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is List<int; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket01", 16, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket01", 16, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket01", 16, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket02 (case 17)
#[test]
fn missing_closing_angle_bracket_02() {
    let src = r#"e is List<int or IEnumerable<int"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is List<int or IEnumerable<int; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket02", 17, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket02", 17, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket02", 17, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket03 (case 18)
#[test]
fn missing_closing_angle_bracket_03() {
    let src = r#"e is List<int { Count: 4 }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is List<int { Count: 4 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket03", 18, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket03", 18, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket03", 18, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket04 (case 19)
#[test]
fn missing_closing_angle_bracket_04() {
    let src = r#"e is not List<int and not IEnumerable<int"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is not List<int and not IEnumerable<int; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket04", 19, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket04", 19, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket04", 19, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket05 (case 20)
#[test]
fn missing_closing_angle_bracket_05() {
    let src = r#"e is (not List<int and not IEnumerable<int) or List<int or (not IEnumerable<int)"#;
    let expected = Some(ExpectedDiagnostics { count: 4, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is (not List<int and not IEnumerable<int) or List<int or (not IEnumerable<int); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket05", 20, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket05", 20, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket05", 20, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket06 (case 21)
#[test]
fn missing_closing_angle_bracket_06() {
    let src = r#"e is X<Y { Property: A<B a }"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is X<Y { Property: A<B a }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket06", 21, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket06", 21, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket06", 21, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.MissingClosingAngleBracket07 (case 22)
#[test]
fn missing_closing_angle_bracket_07() {
    let src = r#"e is A.B<X or C.D<Y"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A.B<X or C.D<Y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket07", 22, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket07", 22, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "MissingClosingAngleBracket07", 22, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType1 (case 23)
#[test]
fn extended_property_subpattern_nullable_type_1() {
    let src = r#"e is { Prop: Type? }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType1", 23, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType1", 23, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType1", 23, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType2 (case 24)
#[test]
fn extended_property_subpattern_nullable_type_2() {
    let src = r#"e is { Prop: Type? propVal }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? propVal }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType2", 24, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType2", 24, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType2", 24, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType3 (case 25)
#[test]
fn extended_property_subpattern_nullable_type_3() {
    let src = r#"e is { Prop: Type? propVal, Prop2: int? val2 }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? propVal, Prop2: int? val2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType3", 25, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType3", 25, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType3", 25, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType4 (case 26)
#[test]
fn extended_property_subpattern_nullable_type_4() {
    let src = r#"e is { Prop: Type? propVal Prop2: int? val2 }"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? propVal Prop2: int? val2 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType4", 26, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType4", 26, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType4", 26, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType5 (case 27)
#[test]
fn extended_property_subpattern_nullable_type_5() {
    let src = r#"e is { Prop: Type? or AnotherType? }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? or AnotherType? }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType5", 27, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType5", 27, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType5", 27, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

/// Roslyn: PatternParsingTests2.ExtendedPropertySubpattern_NullableType6 (case 28)
#[test]
fn extended_property_subpattern_nullable_type_6() {
    let src = r#"e is { Prop: Type? t or AnotherType? a }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { Prop: Type? t or AnotherType? a }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType6", 28, Some(expected.clone()), CaseData::File { unit: &unit, src: src2, original: Some(src) });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType6", 28, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("pattern_parsing_tests_2", "PatternParsingTests2", "ExtendedPropertySubpattern_NullableType6", 28, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
    }
}

