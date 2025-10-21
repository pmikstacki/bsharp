// Auto-generated from Roslyn: SeparatedSyntaxListParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SeparatedSyntaxListParsingTests.TypeArguments (case 1)
#[test]
fn type_arguments() {
    let src = r#"
class C
{
    A<> a1;
    A<T> a2;
    A<,> a3;
    A<T U> a4;
    A<,,> a5;
    A<T,> a6;
    A<,T> a7;
    A<T U,,> a8;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_parsing_tests", "SeparatedSyntaxListParsingTests", "TypeArguments", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SeparatedSyntaxListParsingTests.ArrayRankSpecifiers (case 2)
#[test]
fn array_rank_specifiers() {
    let src = r#"
class C
{
    object a1 = new int[];
    object a1 = new int[1];
    object a1 = new int[,];
    object a1 = new int[1 2];
    object a1 = new int[,,];
    object a1 = new int[1,];
    object a1 = new int[,1];
    object a1 = new int[1 1 ,,];
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("separated_syntax_list_parsing_tests", "SeparatedSyntaxListParsingTests", "ArrayRankSpecifiers", 2, CaseData::File { unit: &unit, src, original: None });
}

