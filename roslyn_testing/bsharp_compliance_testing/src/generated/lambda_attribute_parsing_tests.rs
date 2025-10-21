// Auto-generated from Roslyn: LambdaAttributeParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: LambdaAttributeParsingTests.ConditionalExpression_01_A (case 1)
#[test]
fn conditional_expression_01_a() {
    let src = r#"x ? () => { } : z"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x ? () => { } : z; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_attribute_parsing_tests", "LambdaAttributeParsingTests", "ConditionalExpression_01_A", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaAttributeParsingTests.ParseAttributeWithLambda2 (case 2)
#[test]
fn parse_attribute_with_lambda_2() {
    let src = r#"
                // Lambda inside attribute without attributes of its own is fine for parsing.
                [A(() => {})]
                class C
                {
                }
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_attribute_parsing_tests", "LambdaAttributeParsingTests", "ParseAttributeWithLambda2", 2, CaseData::File { unit: &unit, src, original: None });
}

