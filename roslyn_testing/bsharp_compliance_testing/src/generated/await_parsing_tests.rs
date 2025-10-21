// Auto-generated from Roslyn: AwaitParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: AwaitParsingTests.AwaitOnIdentifierInAsynchronousContext (case 1)
#[test]
fn await_on_identifier_in_asynchronous_context() {
    let src = r#"
class C
{
    async void f()
    {
        await goo();
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitOnIdentifierInAsynchronousContext", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AwaitParsingTests.AwaitOnIdentifierInSynchronousContext (case 2)
#[test]
fn await_on_identifier_in_synchronous_context() {
    let src = r#"
class C
{
    void f()
    {
        await goo();
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitOnIdentifierInSynchronousContext", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AwaitParsingTests.AwaitStatement (case 3)
#[test]
fn await_statement() {
    let src = r#"
class C
{
    async void f()
    {
        await 1;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitStatement", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AwaitParsingTests.NestedLambdaAwait (case 4)
#[test]
fn nested_lambda_await() {
    let src = r#"
class C
{
    void f()
    {
        async () => {
            await 1;
            () => {
                int await;
            };
        };
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "NestedLambdaAwait", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AwaitParsingTests.AwaitExpr (case 5)
#[test]
fn await_expr() {
    let src = r#"
class C
{
    async void f()
    {
        int c = await g() || await g();
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitExpr", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional1 (case 6)
#[test]
fn await_as_start_of_expression_in_conditional_1() {
    let src = r#"f(x is int? await)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { f(x is int? await); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitAsStartOfExpressionInConditional1", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional2 (case 7)
#[test]
fn await_as_start_of_expression_in_conditional_2() {
    let src = r#"dict[x is int? await]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { dict[x is int? await]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitAsStartOfExpressionInConditional2", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: AwaitParsingTests.AwaitAsStartOfExpressionInConditional3 (case 8)
#[test]
fn await_as_start_of_expression_in_conditional_3() {
    let src = r#"x is { Prop: int? await }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is { Prop: int? await }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("await_parsing_tests", "AwaitParsingTests", "AwaitAsStartOfExpressionInConditional3", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

