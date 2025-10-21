// Auto-generated from Roslyn: NameParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
/// Roslyn: NameParsingTests.ParseGlobalAliasQualifiedNameAfterConditionalExpression (case 1)
#[test]
fn parse_global_alias_qualified_name_after_conditional_expression() {
    let src = r#"x is X ? global::X.Y.Z : default"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is X ? global::X.Y.Z : default; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected("name_parsing_tests", "NameParsingTests", "ParseGlobalAliasQualifiedNameAfterConditionalExpression", 1, None, CaseData::File { unit: &unit, src: src2, original: Some(src) });
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

