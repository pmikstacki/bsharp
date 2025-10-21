// Auto-generated from Roslyn: IgnoredDirectiveParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: IgnoredDirectiveParsingTests.Api_Shebang (case 1)
#[test]
fn api_shebang() {
    let src = r#"#!abc"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("ignored_directive_parsing_tests", "IgnoredDirectiveParsingTests", "Api_Shebang", 1, CaseData::File { unit: &unit, src, original: None });
}

