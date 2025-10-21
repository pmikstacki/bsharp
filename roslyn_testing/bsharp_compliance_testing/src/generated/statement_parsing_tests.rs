// Auto-generated from Roslyn: StatementParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: StatementParsingTests.TestUsingVarWithDeclarationTree (case 1)
#[test]
fn using_var_with_declaration_tree() {
    let src = r#"using T a = b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarWithDeclarationTree", 1, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarWithVarDeclarationTree (case 2)
#[test]
fn using_var_with_var_declaration_tree() {
    let src = r#"using var a = b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarWithVarDeclarationTree", 2, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestAwaitUsingVarWithDeclarationTree (case 3)
#[test]
fn await_using_var_with_declaration_tree() {
    let src = r#"await using T a = b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestAwaitUsingVarWithDeclarationTree", 3, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestAwaitUsingVarWithVarDeclarationTree (case 4)
#[test]
fn await_using_var_with_var_declaration_tree() {
    let src = r#"await using var a = b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestAwaitUsingVarWithVarDeclarationTree", 4, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarWithDeclarationMultipleVariablesTree (case 5)
#[test]
fn using_var_with_declaration_multiple_variables_tree() {
    let src = r#"using T a = b, c = d;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarWithDeclarationMultipleVariablesTree", 5, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase1Tree (case 6)
#[test]
fn using_var_special_case_1_tree() {
    let src = r#"using var x = f ? a : b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarSpecialCase1Tree", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase2Tree (case 7)
#[test]
fn using_var_special_case_2_tree() {
    let src = r#"using f ? x = a;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarSpecialCase2Tree", 7, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarSpecialCase3Tree (case 8)
#[test]
fn using_var_special_case_3_tree() {
    let src = r#"using f? x, y;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarSpecialCase3Tree", 8, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarRefTree (case 9)
#[test]
fn using_var_ref_tree() {
    let src = r#"using ref int x = ref y;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarRefTree", 9, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarRefReadonlyTree (case 10)
#[test]
fn using_var_ref_readonly_tree() {
    let src = r#"using ref readonly int x = ref y;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarRefReadonlyTree", 10, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarRefVarTree (case 11)
#[test]
fn using_var_ref_var_tree() {
    let src = r#"using ref var x = ref y;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarRefVarTree", 11, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestUsingVarRefVarIsYTree (case 12)
#[test]
fn using_var_ref_var_is_ytree() {
    let src = r#"using ref var x = y;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestUsingVarRefVarIsYTree", 12, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.ParsePointerToArray (case 13)
#[test]
fn parse_pointer_to_array() {
    let src = r#"int []* p;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "ParsePointerToArray", 13, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.ParsePointerToNullableType (case 14)
#[test]
fn parse_pointer_to_nullable_type() {
    let src = r#"int?* p;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "ParsePointerToNullableType", 14, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.ParseNewNullableWithInitializer (case 15)
#[test]
fn parse_new_nullable_with_initializer() {
    let src = r#"_ = new int? {};"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "ParseNewNullableWithInitializer", 15, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern1 (case 16)
#[test]
fn switch_statement_with_nullable_type_in_pattern_1() {
    let src = r#"
                switch (obj)
                {
                    case Type?:
                        break;
                }
                "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestSwitchStatementWithNullableTypeInPattern1", 16, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern2 (case 17)
#[test]
fn switch_statement_with_nullable_type_in_pattern_2() {
    let src = r#"
                switch (obj)
                {
                    case Type? varName:
                        break;
                }
                "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestSwitchStatementWithNullableTypeInPattern2", 17, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern3 (case 18)
#[test]
fn switch_statement_with_nullable_type_in_pattern_3() {
    let src = r#"
                switch (obj)
                {
                    case Type? when x > 0:
                        break;
                }
                "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestSwitchStatementWithNullableTypeInPattern3", 18, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: StatementParsingTests.TestSwitchStatementWithNullableTypeInPattern4 (case 19)
#[test]
fn switch_statement_with_nullable_type_in_pattern_4() {
    let src = r#"
                switch (obj)
                {
                    case Type? varName when x > 0:
                        break;
                }
                "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("statement_parsing_tests", "StatementParsingTests", "TestSwitchStatementWithNullableTypeInPattern4", 19, CaseData::Statement { ast: &ast, src });
}

