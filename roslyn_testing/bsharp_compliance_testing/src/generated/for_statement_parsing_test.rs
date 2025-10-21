// Auto-generated from Roslyn: ForStatementParsingTest
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ForStatementParsingTest.TestCommaSeparators1 (case 1)
#[test]
fn comma_separators_1() {
    let src = r#"for (int i = 0, j = 0; i < 10; i++) ;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestCommaSeparators1", 1, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestCommaSeparators6 (case 2)
#[test]
fn comma_separators_6() {
    let src = r#"for (int i = 0, j; i < 10; i++) ;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestCommaSeparators6", 2, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers1 (case 3)
#[test]
fn multiple_declarators_with_initializers_1() {
    let src = r#"
            for (int offset = 0, c1, c2; offset < length;)
            {
            }
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestMultipleDeclaratorsWithInitializers1", 3, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers2 (case 4)
#[test]
fn multiple_declarators_with_initializers_2() {
    let src = r#"
            for (int offset = 0, c1 = 1, c2; offset < length;)
            {
            }
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestMultipleDeclaratorsWithInitializers2", 4, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithInitializers3 (case 5)
#[test]
fn multiple_declarators_with_initializers_3() {
    let src = r#"
            for (int offset = 0, c1, c2 = 1; offset < length;)
            {
            }
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestMultipleDeclaratorsWithInitializers3", 5, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestMultipleDeclaratorsWithExpression1 (case 6)
#[test]
fn multiple_declarators_with_expression_1() {
    let src = r#"
            for (Console.WriteLine("Blah"); true;)
            {
            }
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestMultipleDeclaratorsWithExpression1", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_AnonymousFunction (case 7)
#[test]
fn various_expressions_anonymous_function() {
    let src = r#"
            for (delegate() {};delegate() {};delegate() {});
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_AnonymousFunction", 7, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_AnonymousObjectCreation (case 8)
#[test]
fn various_expressions_anonymous_object_creation() {
    let src = r#"
            for (new();new();new());
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_AnonymousObjectCreation", 8, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ArrayCreation (case 9)
#[test]
fn various_expressions_array_creation() {
    let src = r#"
            for (new int[] { };new int[] { };new int[] { });
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ArrayCreation", 9, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Assignment1 (case 10)
#[test]
fn various_expressions_assignment_1() {
    let src = r#"
            for (a=1;a=1;a=1);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Assignment1", 10, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Assignment2 (case 11)
#[test]
fn various_expressions_assignment_2() {
    let src = r#"
            for (a+=1;a+=1;a+=1);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Assignment2", 11, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Cast (case 12)
#[test]
fn various_expressions_cast() {
    let src = r#"
            for ((int)0;(int)0;(int)0);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Cast", 12, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Checked (case 13)
#[test]
fn various_expressions_checked() {
    let src = r#"
            for (checked(0);checked(0);checked(0));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Checked", 13, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Collection (case 14)
#[test]
fn various_expressions_collection() {
    let src = r#"
            for ([];[];[]);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Collection", 14, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ConditionalAccess (case 15)
#[test]
fn various_expressions_conditional_access() {
    let src = r#"
            for (a?.b;a?.b;a?.b);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ConditionalAccess", 15, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_DefaultExpression1 (case 16)
#[test]
fn various_expressions_default_expression_1() {
    let src = r#"
            for (default(int);default(int);default(int));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_DefaultExpression1", 16, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_DefaultExpression2 (case 17)
#[test]
fn various_expressions_default_expression_2() {
    let src = r#"
            for (default;default;default);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_DefaultExpression2", 17, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ElementAccess (case 18)
#[test]
fn various_expressions_element_access() {
    let src = r#"
            for (a[0];a[0];a[0]);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ElementAccess", 18, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_InterpolatedString (case 19)
#[test]
fn various_expressions_interpolated_string() {
    let src = r#"
            for ($"";$"";$"");
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_InterpolatedString", 19, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Invocation (case 20)
#[test]
fn various_expressions_invocation() {
    let src = r#"
            for (a();a();a());
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Invocation", 20, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_IsPattern (case 21)
#[test]
fn various_expressions_is_pattern() {
    let src = r#"
            for (a is B b;a is B b;a is B b);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_IsPattern", 21, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Literal (case 22)
#[test]
fn various_expressions_literal() {
    let src = r#"
            for (true;true;true);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Literal", 22, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_MemberAccess (case 23)
#[test]
fn various_expressions_member_access() {
    let src = r#"
            for (a.b;a.b;a.b);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_MemberAccess", 23, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Parenthesized (case 24)
#[test]
fn various_expressions_parenthesized() {
    let src = r#"
            for ((a);(a);(a));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Parenthesized", 24, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Postfix (case 25)
#[test]
fn various_expressions_postfix() {
    let src = r#"
            for (a++;a++;a++);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Postfix", 25, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation1 (case 26)
#[test]
fn various_expressions_object_creation_1() {
    let src = r#"
            for (new A();new A();new A());
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ObjectCreation1", 26, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation2 (case 27)
#[test]
fn various_expressions_object_creation_2() {
    let src = r#"
            for (new A() { };new A() { };new A() { });
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ObjectCreation2", 27, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_ObjectCreation3 (case 28)
#[test]
fn various_expressions_object_creation_3() {
    let src = r#"
            for (new A { };new A { };new A { });
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_ObjectCreation3", 28, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Prefix (case 29)
#[test]
fn various_expressions_prefix() {
    let src = r#"
            for (++a;++a;++a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Prefix", 29, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Query (case 30)
#[test]
fn various_expressions_query() {
    let src = r#"
            for (from a in b select c;from a in b select c;from a in b select c);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Query", 30, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range1 (case 31)
#[test]
fn various_expressions_range_1() {
    let src = r#"
            for (..;..;..);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Range1", 31, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range2 (case 32)
#[test]
fn various_expressions_range_2() {
    let src = r#"
            for (a..;a..;a..);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Range2", 32, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range3 (case 33)
#[test]
fn various_expressions_range_3() {
    let src = r#"
            for (..a;..a;..a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Range3", 33, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Range4 (case 34)
#[test]
fn various_expressions_range_4() {
    let src = r#"
            for (a..a;a..a;a..a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Range4", 34, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Ref1 (case 35)
#[test]
fn various_expressions_ref_1() {
    let src = r#"
            for (ref a; ref a; ref a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Ref1", 35, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Ref2 (case 36)
#[test]
fn various_expressions_ref_2() {
    let src = r#"
            for (ref int a; ref a; ref a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Ref2", 36, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Sizeof (case 37)
#[test]
fn various_expressions_sizeof() {
    let src = r#"
            for (sizeof(a);sizeof(a);sizeof(a));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Sizeof", 37, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Throw (case 38)
#[test]
fn various_expressions_throw() {
    let src = r#"
            for (throw a;throw a;throw a);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Throw", 38, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Tuple (case 39)
#[test]
fn various_expressions_tuple() {
    let src = r#"
            for ((a, b);(a, b);(a, b));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Tuple", 39, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_Typeof (case 40)
#[test]
fn various_expressions_typeof() {
    let src = r#"
            for (typeof(int);typeof(int);typeof(int));
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_Typeof", 40, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestVariousExpressions_With2 (case 41)
#[test]
fn various_expressions_with_2() {
    let src = r#"
            for (; a with { }; a with { })
            {
            }
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestVariousExpressions_With2", 41, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer1 (case 42)
#[test]
fn complex_initializer_1() {
    let src = r#"
            for (;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer1", 42, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer2 (case 43)
#[test]
fn complex_initializer_2() {
    let src = r#"
            for (int i;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer2", 43, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer3 (case 44)
#[test]
fn complex_initializer_3() {
    let src = r#"
            for (int i, j, k;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer3", 44, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer4 (case 45)
#[test]
fn complex_initializer_4() {
    let src = r#"
            for (int i = 0;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer4", 45, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer5 (case 46)
#[test]
fn complex_initializer_5() {
    let src = r#"
            for (A b;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer5", 46, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer6 (case 47)
#[test]
fn complex_initializer_6() {
    let src = r#"
            for (A b, c, d;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer6", 47, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer7 (case 48)
#[test]
fn complex_initializer_7() {
    let src = r#"
            for (A b = null, c, d = null;;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer7", 48, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: ForStatementParsingTest.TestComplexInitializer8 (case 49)
#[test]
fn complex_initializer_8() {
    let src = r#"
            for (A b = c switch { A => x, _ => y };;);
            "#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("for_statement_parsing_test", "ForStatementParsingTest", "TestComplexInitializer8", 49, CaseData::Statement { ast: &ast, src });
}

