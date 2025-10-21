// Auto-generated from Roslyn: StatementAttributeParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: StatementAttributeParsingTests.AttributeOnEmptyStatement (case 1)
#[test]
fn attribute_on_empty_statement() {
    let src = r#"
class C
{
    void Goo()
    {
        [A];
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("statement_attribute_parsing_tests", "StatementAttributeParsingTests", "AttributeOnEmptyStatement", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Parenthesized (case 2)
#[test]
fn attribute_on_expression_statement_parenthesized() {
    let src = r#"
class C
{
    void Goo()
    {
        [A](1);
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("statement_attribute_parsing_tests", "StatementAttributeParsingTests", "AttributeOnExpressionStatement_Parenthesized", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range3 (case 3)
#[test]
fn attribute_on_expression_statement_range_3() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]..b;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("statement_attribute_parsing_tests", "StatementAttributeParsingTests", "AttributeOnExpressionStatement_Range3", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: StatementAttributeParsingTests.AttributeOnExpressionStatement_Range4 (case 4)
#[test]
fn attribute_on_expression_statement_range_4() {
    let src = r#"
class C
{
    void Goo(int a, int b)
    {
        [A]..;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("statement_attribute_parsing_tests", "StatementAttributeParsingTests", "AttributeOnExpressionStatement_Range4", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: StatementAttributeParsingTests.AttrDeclOnStatementWhereMemberExpected (case 5)
#[test]
fn attr_decl_on_statement_where_member_expected() {
    let src = r#"
class C
{
    [Attr] x.y();
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("statement_attribute_parsing_tests", "StatementAttributeParsingTests", "AttrDeclOnStatementWhereMemberExpected", 5, CaseData::File { unit: &unit, src, original: None });
}

