// Auto-generated from Roslyn: FunctionPointerTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: FunctionPointerTests.SimpleFunctionPointerTest (case 1)
#[test]
fn simple_function_pointer_test() {
    let src = r#"delegate*<string, Goo, int> ptr;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "SimpleFunctionPointerTest", 1, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.VoidsAsType (case 2)
#[test]
fn voids_as_type() {
    let src = r#"delegate*<void, void, void> ptr;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "VoidsAsType", 2, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.PointerToAFunctionPointer (case 3)
#[test]
fn pointer_to_afunction_pointer() {
    let src = r#"delegate*<Goo, void>* ptr;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "PointerToAFunctionPointer", 3, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.Unterminated_05 (case 4)
#[test]
fn unterminated_05() {
    let src = r#"delegate* unmanaged[ptr];"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "Unterminated_05", 4, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.Unterminated_06 (case 5)
#[test]
fn unterminated_06() {
    let src = r#"delegate* unmanaged[cdecl] ;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "Unterminated_06", 5, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.ArrayType (case 6)
#[test]
fn array_type() {
    let src = r#"delegate*<ref C>[] ptr;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "ArrayType", 6, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: FunctionPointerTests.LambdaParameterType (case 7)
#[test]
fn lambda_parameter_type() {
    let src = r#"(delegate*<void> p1) => {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (delegate*<void> p1) => {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "LambdaParameterType", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: FunctionPointerTests.IsExpression (case 8)
#[test]
fn is_expression() {
    let src = r#"o is delegate*<void>"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is delegate*<void>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "IsExpression", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: FunctionPointerTests.IsNamedExpression (case 9)
#[test]
fn is_named_expression() {
    let src = r#"o is delegate*<void> ptr"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is delegate*<void> ptr; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "IsNamedExpression", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: FunctionPointerTests.SizeOf (case 10)
#[test]
fn size_of() {
    let src = r#"sizeof(delegate*<void>)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { sizeof(delegate*<void>); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "SizeOf", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: FunctionPointerTests.FunctionPointerArrayInTypeArgument (case 11)
#[test]
fn function_pointer_array_in_type_argument() {
    let src = r#"I<delegate*<void>[]> i;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("function_pointer_tests", "FunctionPointerTests", "FunctionPointerArrayInTypeArgument", 11, CaseData::Statement { ast: &ast, src });
}

