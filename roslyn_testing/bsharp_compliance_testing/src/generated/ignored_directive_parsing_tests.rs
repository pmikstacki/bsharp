// Auto-generated from Roslyn: IgnoredDirectiveParsingTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: IgnoredDirectiveParsingTests.Api_Shebang (case 1)
#[test]
fn api_shebang() {
    let src = r#"#!abc"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "ignored_directive_parsing_tests",
                "IgnoredDirectiveParsingTests",
                "Api_Shebang",
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
