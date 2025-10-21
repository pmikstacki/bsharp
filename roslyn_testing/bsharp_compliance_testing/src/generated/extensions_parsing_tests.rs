// Auto-generated from Roslyn: ExtensionsParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ExtensionsParsingTests.MultipleConstraints (case 1)
#[test]
fn multiple_constraints() {
    let src = r#"
class C
{
    extension<T1, T2>(object o) where T1 : struct where T2 : class { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "MultipleConstraints", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.TypeNamedExtension (case 2)
#[test]
fn type_named_extension() {
    let src = r#"
class extension
{
    extension(Type constructorParameter) { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "TypeNamedExtension", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.TypeNamedExtension (case 3)
#[test]
fn type_named_extension_case_2() {
    let src = r#"
class extension
{
    extension(Type constructorParameter) { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "TypeNamedExtension", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithModifierOnParameter_ScopedRef (case 4)
#[test]
fn with_modifier_on_parameter_scoped_ref() {
    let src = r#"
class C
{
    extension(scoped ref Type x) { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithModifierOnParameter_ScopedRef", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon (case 5)
#[test]
fn with_terminator_semi_colon() {
    let src = r#"
class C
{
    extension(Type) { ;
    class D { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithTerminator_SemiColon", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon_02 (case 6)
#[test]
fn with_terminator_semi_colon_02() {
    let src = r#"
class C
{
    extension(Type) { ;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithTerminator_SemiColon_02", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithTerminator_SemiColon_03 (case 7)
#[test]
fn with_terminator_semi_colon_03() {
    let src = r#"
class C
{
    extension<T ;
    class D { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithTerminator_SemiColon_03", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.MissingParameterList (case 8)
#[test]
fn missing_parameter_list() {
    let src = r#"
class C
{
    extension ;
    class D { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "MissingParameterList", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.SemiColonBody (case 9)
#[test]
fn semi_colon_body() {
    let src = r#"
class C
{
    extension<T>(Type) where T : struct;
    class D { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "SemiColonBody", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithTerminator_OpenBrace (case 10)
#[test]
fn with_terminator_open_brace() {
    let src = r#"
class C
{
    extension(Type) { {
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithTerminator_OpenBrace", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.MissingTypeAndIdentifier (case 11)
#[test]
fn missing_type_and_identifier() {
    let src = r#"
class C
{
    extension() { }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "MissingTypeAndIdentifier", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.ExtensionInExpression (case 12)
#[test]
fn extension_in_expression() {
    let src = r#"
class C
{
    void extension() { extension(); }
    void M()
    {
        extension extension = null;
    }
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "ExtensionInExpression", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.ParameterNameIsWhereOfConstraint (case 13)
#[test]
fn parameter_name_is_where_of_constraint() {
    let src = r#"
class C
{
    extension(object where T :
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "ParameterNameIsWhereOfConstraint", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExtensionsParsingTests.WithBodyAndSemiColon (case 14)
#[test]
fn with_body_and_semi_colon() {
    let src = r#"
class C
{
    extension(object) { };
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("extensions_parsing_tests", "ExtensionsParsingTests", "WithBodyAndSemiColon", 14, CaseData::File { unit: &unit, src, original: None });
}

