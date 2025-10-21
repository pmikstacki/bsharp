// Auto-generated from Roslyn: AsyncStreamsParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: AsyncStreamsParsingTests.AwaitUsingDeclaration (case 1)
#[test]
fn await_using_declaration() {
    let src = r#"
class C
{
    async void M()
    {
        await using (var x = this)
        {
        }
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingDeclaration", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncStreamsParsingTests.AwaitUsingWithExpression (case 2)
#[test]
fn await_using_with_expression() {
    let src = r#"
class C
{
    async void M()
    {
        await using (this)
        {
        }
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncStreamsParsingTests.AwaitForeach (case 3)
#[test]
fn await_foreach() {
    let src = r#"
class C
{
    async void M()
    {
        await foreach (var i in collection)
        {
        }
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach", 3, CaseData::File { unit: &unit, src, original: None });
}

