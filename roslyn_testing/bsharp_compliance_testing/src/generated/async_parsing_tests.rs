// Auto-generated from Roslyn: AsyncParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: AsyncParsingTests.SimpleAsyncMethod (case 1)
#[test]
fn simple_async_method() {
    let src = r#"
class C
{
    async void M() { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "SimpleAsyncMethod", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.MethodReturningAsync (case 2)
#[test]
fn method_returning_async() {
    let src = r#"
class C
{
    async M() { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "MethodReturningAsync", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember01 (case 3)
#[test]
fn incomplete_async_member_01() {
    let src = r#"
class C
{
    async Task<
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember01", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember02 (case 4)
#[test]
fn incomplete_async_member_02() {
    let src = r#"
class C
{
    async Tasks.Task<
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember02", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember03 (case 5)
#[test]
fn incomplete_async_member_03() {
    let src = r#"
class C
{
    static async Tasks.Task<
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember03", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember04 (case 6)
#[test]
fn incomplete_async_member_04() {
    let src = r#"
class C
{
    async operator+
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember04", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember05 (case 7)
#[test]
fn incomplete_async_member_05() {
    let src = r#"
class C
{
    async Task<T>
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember05", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.IncompleteAsyncMember06 (case 8)
#[test]
fn incomplete_async_member_06() {
    let src = r#"
class C
{
    async Task<T> f
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "IncompleteAsyncMember06", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.EventAsyncAsync (case 9)
#[test]
fn event_async_async() {
    let src = r#"
class C
{
    event async async;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "EventAsyncAsync", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.EventAsyncAsyncAsync1 (case 10)
#[test]
fn event_async_async_async_1() {
    let src = r#"
class C
{
    event async async async;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "EventAsyncAsyncAsync1", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.EventAsyncAsyncAsync2 (case 11)
#[test]
fn event_async_async_async_2() {
    let src = r#"
class C
{
    async event async async;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "EventAsyncAsyncAsync2", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.AsyncModifierOnDelegateDeclaration (case 12)
#[test]
fn async_modifier_on_delegate_declaration() {
    let src = r#"
class C
{
    public async delegate void Goo();
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "AsyncModifierOnDelegateDeclaration", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.AsyncTypeCloseCurly (case 13)
#[test]
fn async_type_close_curly() {
    let src = r#"class C { async T }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "AsyncTypeCloseCurly", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.AsyncGenericType (case 14)
#[test]
fn async_generic_type() {
    let src = r#"class Program
{
    public async Task<IReadOnlyCollection<ProjectConfiguration>>
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "AsyncGenericType", 14, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern2 (case 15)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_2() {
    let src = r#"x is A a ? async b => 0 : null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A a ? async b => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "AsyncLambdaInConditionalExpressionAfterPattern2", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: AsyncParsingTests.AsyncLambdaInConditionalExpressionAfterPattern4 (case 16)
#[test]
fn async_lambda_in_conditional_expression_after_pattern_4() {
    let src = r#"x is A a ? async (b) => 0 : null"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x is A a ? async (b) => 0 : null; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("async_parsing_tests", "AsyncParsingTests", "AsyncLambdaInConditionalExpressionAfterPattern4", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

