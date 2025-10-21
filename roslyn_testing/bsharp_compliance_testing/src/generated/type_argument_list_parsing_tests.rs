// Auto-generated from Roslyn: TypeArgumentListParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: TypeArgumentListParsingTests.TestComparisonToTuple (case 1)
#[test]
fn comparison_to_tuple() {
    let src = r#"
public class C
{
    public static void Main()
    {
        XX X = new XX();
        int a = 1, b = 2;
        bool z = X < (a, b), w = false;
    }
}

struct XX
{
    public static bool operator <(XX x, (int a, int b) arg) => true;
    public static bool operator >(XX x, (int a, int b) arg) => false;
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("type_argument_list_parsing_tests", "TypeArgumentListParsingTests", "TestComparisonToTuple", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_01 (case 2)
#[test]
fn generic_arg_with_greater_than_01() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>>

        ProjectChange = projectChange;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("type_argument_list_parsing_tests", "TypeArgumentListParsingTests", "TestGenericArgWithGreaterThan_01", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_02 (case 3)
#[test]
fn generic_arg_with_greater_than_02() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<U<T<S>>>

        ProjectChange = projectChange;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("type_argument_list_parsing_tests", "TypeArgumentListParsingTests", "TestGenericArgWithGreaterThan_02", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_03 (case 4)
#[test]
fn generic_arg_with_greater_than_03() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<S>>>

        ProjectChange = projectChange;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("type_argument_list_parsing_tests", "TypeArgumentListParsingTests", "TestGenericArgWithGreaterThan_03", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: TypeArgumentListParsingTests.TestGenericArgWithGreaterThan_04 (case 5)
#[test]
fn generic_arg_with_greater_than_04() {
    let src = r#"
class C
{
    void M()
    {
        var added = ImmutableDictionary<T<(S, U)>>>

        ProjectChange = projectChange;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("type_argument_list_parsing_tests", "TypeArgumentListParsingTests", "TestGenericArgWithGreaterThan_04", 5, CaseData::File { unit: &unit, src, original: None });
}

