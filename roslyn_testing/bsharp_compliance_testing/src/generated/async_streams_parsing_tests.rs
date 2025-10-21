// Auto-generated from Roslyn: AsyncStreamsParsingTests
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use crate::custom_asserts::roslyn_asserts::ExpectedDiagnostics;
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
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingDeclaration", 1, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingDeclaration", 1, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingDeclaration", 1, None, CaseData::File { unit: &unit, src, original: None });
    }
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
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression", 2, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression", 2, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression", 2, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: AsyncStreamsParsingTests.AwaitUsingWithExpression_Reversed (case 3)
#[test]
fn await_using_with_expression_reversed() {
    let src = r#"
class C
{
    async void M()
    {
        using await (this)
        {
        }
    }
}
"#;
    let expected = Some(ExpectedDiagnostics { count: 6, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression_Reversed", 3, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression_Reversed", 3, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitUsingWithExpression_Reversed", 3, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: AsyncStreamsParsingTests.AwaitForeach (case 4)
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
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach", 4, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach", 4, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach", 4, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: AsyncStreamsParsingTests.AwaitForeach_Reversed (case 5)
#[test]
fn await_foreach_reversed() {
    let src = r#"
class C
{
    async void M()
    {
        foreach await (var i in collection)
        {
        }
    }
}
"#;
    let expected = Some(ExpectedDiagnostics { count: 4, items: vec![] });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach_Reversed", 5, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach_Reversed", 5, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "AwaitForeach_Reversed", 5, None, CaseData::File { unit: &unit, src, original: None });
    }
}

/// Roslyn: AsyncStreamsParsingTests.DeconstructionAwaitForeach (case 6)
#[test]
fn deconstruction_await_foreach() {
    let src = r#"
class C
{
    async void M()
    {
        await foreach (var (i, j) in collection)
        {
        }
    }
}
"#;
    let expected: Option<ExpectedDiagnostics> = None; 
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "DeconstructionAwaitForeach", 6, Some(expected.clone()), CaseData::File { unit: &unit, src, original: None });
            }
            Err(_) => {
                after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "DeconstructionAwaitForeach", 6, Some(expected.clone()), CaseData::Empty);
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected("async_streams_parsing_tests", "AsyncStreamsParsingTests", "DeconstructionAwaitForeach", 6, None, CaseData::File { unit: &unit, src, original: None });
    }
}

