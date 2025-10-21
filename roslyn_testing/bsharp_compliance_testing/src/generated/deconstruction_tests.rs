// Auto-generated from Roslyn: DeconstructionTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: DeconstructionTests.ParenExpression (case 1)
#[test]
fn paren_expression() {
    let src = r#"
class C
{
    void Goo()
    {
        (x).ToString();
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "ParenExpression", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.TupleLiteral (case 2)
#[test]
fn tuple_literal() {
    let src = r#"
class C
{
    void Goo()
    {
        (Int32, Int64).Goo();
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "TupleLiteral", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DeconstructionAssignment (case 3)
#[test]
fn deconstruction_assignment() {
    let src = r#"
class C
{
    void Goo()
    {
        (x, y) = goo;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DeconstructionAssignment", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.SimpleDeclaration (case 4)
#[test]
fn simple_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        for(Int32 x = goo; ; ) { }
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "SimpleDeclaration", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.NestedDeconstructionAssignment (case 5)
#[test]
fn nested_deconstruction_assignment() {
    let src = r#"
class C
{
    void Goo()
    {
        (x, (y, z)) = goo;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "NestedDeconstructionAssignment", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.VarDeconstructionDeclaration (case 6)
#[test]
fn var_deconstruction_declaration() {
    let src = r#"
class C
{
    void Goo()
    {
        var (a, b) = goo;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "VarDeconstructionDeclaration", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.VarNestedDeconstructionDeclaration (case 7)
#[test]
fn var_nested_deconstruction_declaration() {
    let src = r#"
        class C
        {
            void Goo()
            {
                var ((a, b), c) = goo;
            }
        }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "VarNestedDeconstructionDeclaration", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.VarMethodCall (case 8)
#[test]
fn var_method_call() {
    let src = r#"
class C
{
    void Goo()
    {
        var(a, b);
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "VarMethodCall", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.VarDeconstructionFor (case 9)
#[test]
fn var_deconstruction_for() {
    let src = r#"
        class C
        {
            void Goo()
            {
                for (var (x, y) = goo; ; ) { }
            }
        }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "VarDeconstructionFor", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DiscardsInDeconstruction_01 (case 10)
#[test]
fn discards_in_deconstruction_01() {
    let src = r#"void M() { var (x, _) = e; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DiscardsInDeconstruction_01", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_01 (case 11)
#[test]
fn discards_in_pattern_01() {
    let src = r#"void M() { if (e is int _) {} }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DiscardsInPattern_01", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_02 (case 12)
#[test]
fn discards_in_pattern_02() {
    let src = r#"void M() { if (e is var _) {} }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DiscardsInPattern_02", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_03 (case 13)
#[test]
fn discards_in_pattern_03() {
    let src = r#"void M() { switch (e) { case int _: break; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DiscardsInPattern_03", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.DiscardsInPattern_04 (case 14)
#[test]
fn discards_in_pattern_04() {
    let src = r#"void M() { switch (e) { case var _: break; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "DiscardsInPattern_04", 14, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_00 (case 15)
#[test]
fn bad_type_for_deconstruct_00() {
    let src = r#"var (x, y) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "BadTypeForDeconstruct_00", 15, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_02 (case 16)
#[test]
fn bad_type_for_deconstruct_02() {
    let src = r#"var.var (x, y) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "BadTypeForDeconstruct_02", 16, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_03 (case 17)
#[test]
fn bad_type_for_deconstruct_03() {
    let src = r#"var<var> (x, y) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "BadTypeForDeconstruct_03", 17, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: DeconstructionTests.BadTypeForDeconstruct_07 (case 18)
#[test]
fn bad_type_for_deconstruct_07() {
    let src = r#"var?.var (x, y) = e;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("deconstruction_tests", "DeconstructionTests", "BadTypeForDeconstruct_07", 18, CaseData::Statement { ast: &ast, src });
}

