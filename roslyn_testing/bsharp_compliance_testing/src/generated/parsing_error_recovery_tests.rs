// Auto-generated from Roslyn: ParsingErrorRecoveryTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_ErrorWarning (case 1)
#[test]
fn preprocessor_directive_trailing_error_warning() {
    let src = r#"
                /* comment */ #error E1
                /* comment */ #warning W1
                #error E2
                #warning W2
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("parsing_error_recovery_tests", "ParsingErrorRecoveryTests", "PreprocessorDirective_Trailing_ErrorWarning", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ParsingErrorRecoveryTests.PreprocessorDirective_Trailing_Line (case 2)
#[test]
fn preprocessor_directive_trailing_line() {
    let src = r#"
                #line 200
                /* comment */ #line 100
                #error E1
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("parsing_error_recovery_tests", "ParsingErrorRecoveryTests", "PreprocessorDirective_Trailing_Line", 2, CaseData::File { unit: &unit, src, original: None });
}

