// Auto-generated from Roslyn: DeclarationParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: DeclarationParsingTests.CS0071_01 (case 1)
#[test]
fn cs_0071_01() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.P10;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "CS0071_01", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeclarationParsingTests.CS0071_02 (case 2)
#[test]
fn cs_0071_02() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.
P10;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "CS0071_02", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeclarationParsingTests.CS0071_03 (case 3)
#[test]
fn cs_0071_03() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.
P10
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "CS0071_03", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeclarationParsingTests.CS0071_04 (case 4)
#[test]
fn cs_0071_04() {
    let src = r#"
public interface I2 { }
public interface I1
{
    event System.Action I2.P10
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "CS0071_04", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeclarationParsingTests.NonAccessorAfterIncompleteProperty (case 5)
#[test]
fn non_accessor_after_incomplete_property() {
    let src = r#"
class C
{
    int A { get { return this.
    public int B;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "NonAccessorAfterIncompleteProperty", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeclarationParsingTests.ExpressionBodiedCtorDtorProp (case 6)
#[test]
fn expression_bodied_ctor_dtor_prop() {
    let src = r#"
class C
{
    C() : base() => M();
    C() => M();
    ~C() => M();
    int P { set => M(); }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("declaration_parsing_tests", "DeclarationParsingTests", "ExpressionBodiedCtorDtorProp", 6, CaseData::File { unit: &unit, src, original: None });
}

