// Auto-generated from Roslyn: RecordParsing
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
/// Roslyn: RecordParsing.RecordParsing06 (case 1)
#[test]
fn record_parsing_06() {
    let src = r#"interface P;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing06", 1, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing06", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing06", 1, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintAndSemiColon (case 2)
#[test]
fn record_parsing_constraint_and_semi_colon() {
    let src = r#"record R<T> where T : class;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon", 2, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon", 2, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintAndSemiColon_MissingColon (case 3)
#[test]
fn record_parsing_constraint_and_semi_colon_missing_colon() {
    let src = r#"record R<T> where T   class;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_MissingColon", 3, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_MissingColon", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_MissingColon", 3, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_TwoConstraintsAndSemiColon (case 4)
#[test]
fn record_parsing_two_constraints_and_semi_colon() {
    let src = r#"record R<T1, T2> where T1 : class where T2 : class;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon", 4, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon", 4, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintAndSemiColon_Class (case 5)
#[test]
fn record_parsing_constraint_and_semi_colon_class() {
    let src = r#"abstract class C<T> where T : class;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_Class", 5, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_Class", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndSemiColon_Class", 5, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_TwoConstraintsAndSemiColon_Class (case 6)
#[test]
fn record_parsing_two_constraints_and_semi_colon_class() {
    let src = r#"abstract class C<T1, T2> where T1 : class where T2 : class;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon_Class", 6, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon_Class", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_TwoConstraintsAndSemiColon_Class", 6, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.AbstractMethod_ConstraintsAndSemiColon (case 7)
#[test]
fn abstract_method_constraints_and_semi_colon() {
    let src = r#"abstract record R { abstract void M<T>() where T : class; }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "AbstractMethod_ConstraintsAndSemiColon", 7, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "AbstractMethod_ConstraintsAndSemiColon", 7, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "AbstractMethod_ConstraintsAndSemiColon", 7, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.TestClassWithMultipleConstraints001 (case 8)
#[test]
fn class_with_multiple_constraints_001() {
    let src = r#"class a<b> where b : c where b { }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints001", 8, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints001", 8, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints001", 8, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.TestClassWithMultipleConstraints002 (case 9)
#[test]
fn class_with_multiple_constraints_002() {
    let src = r#"class a<b> where b : c where { }"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints002", 9, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints002", 9, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestClassWithMultipleConstraints002", 9, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintsAndCurlyBraces (case 10)
#[test]
fn record_parsing_constraints_and_curly_braces() {
    let src = r#"record R<T> where T : class { }"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintsAndCurlyBraces", 10, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintsAndCurlyBraces", 10, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintsAndCurlyBraces", 10, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintAndCommaAndSemiColon (case 11)
#[test]
fn record_parsing_constraint_and_comma_and_semi_colon() {
    let src = r#"record R<T> where T : class, ;"#;
    let expected = Some(ExpectedDiagnostics { count: 1, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndSemiColon", 11, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndSemiColon", 11, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndSemiColon", 11, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.RecordParsing_ConstraintAndCommaAndNewAndSemiColon (case 12)
#[test]
fn record_parsing_constraint_and_comma_and_new_and_semi_colon() {
    let src = r#"record R<T> where T : class, new();"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndNewAndSemiColon", 12, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndNewAndSemiColon", 12, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "RecordParsing_ConstraintAndCommaAndNewAndSemiColon", 12, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.TestWhereWhere (case 13)
#[test]
fn where_where() {
    let src = r#"public class Goo<T> : System.Object where where { }"#;
    let expected = Some(ExpectedDiagnostics { count: 2, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhere", 13, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhere", 13, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhere", 13, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.TestWhereWhereWhere (case 14)
#[test]
fn where_where_where() {
    let src = r#"public class Goo<T> : System.Object where where where { }"#;
    let expected = Some(ExpectedDiagnostics { count: 3, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhereWhere", 14, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhereWhere", 14, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "TestWhereWhereWhere", 14, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: RecordParsing.WithParsing10 (case 15)
#[test]
fn with_parsing_10() {
    let src = r#"int x = await with { };"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing10", 15, Some(expected.clone()), CaseData::Statement { ast: &ast, src });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing10", 15, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing10", 15, None, CaseData::Statement { ast: &ast, src });
    }
}

/// Roslyn: RecordParsing.WithParsing11 (case 16)
#[test]
fn with_parsing_11() {
    let src = r#"await with;"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    if let Some(expected) = expected {
        match r {
            Ok((rest, ast)) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing11", 16, Some(expected.clone()), CaseData::Statement { ast: &ast, src });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing11", 16, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (rest, ast) = r.unwrap();
        assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
        after_parse::after_parse_with_expected("record_parsing", "RecordParsing", "WithParsing11", 16, None, CaseData::Statement { ast: &ast, src });
    }
}

