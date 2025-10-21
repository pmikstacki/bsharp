// Auto-generated from Roslyn: LambdaParameterParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: LambdaParameterParsingTests.TestLambdaWithNullValidation (case 1)
#[test]
fn lambda_with_null_validation() {
    let src = r#"Func<string, string> func1 = x!! => x + "1";"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func1 = x!! => x + "1"; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestLambdaWithNullValidation", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestLambdaWithNullValidationParams (case 2)
#[test]
fn lambda_with_null_validation_params() {
    let src = r#"Func<int, int, bool> func1 = (x!!, y) => x == y;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, bool> func1 = (x!!, y) => x == y; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestLambdaWithNullValidationParams", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSingleParamInParens (case 3)
#[test]
fn null_checked_single_param_in_parens() {
    let src = r#"Func<int, int> func1 = (x!!) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (x!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSingleParamInParens", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSingleParamNoSpaces (case 4)
#[test]
fn null_checked_single_param_no_spaces() {
    let src = r#"Func<int, int> func1 = x!!=>x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = x!!=>x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSingleParamNoSpaces", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedTypedSingleParamInParen (case 5)
#[test]
fn null_checked_typed_single_param_in_paren() {
    let src = r#"Func<int, int> func1 = (int x!!) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (int x!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedTypedSingleParamInParen", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedTypedManyParams (case 6)
#[test]
fn null_checked_typed_many_params() {
    let src = r#"Func<int, int, int> func1 = (int x!!, int y) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, int> func1 = (int x!!, int y) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedTypedManyParams", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestManyNullCheckedTypedParams (case 7)
#[test]
fn many_null_checked_typed_params() {
    let src = r#"Func<int, int, int> func1 = (int x!!, int y!!) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int, int> func1 = (int x!!, int y!!) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestManyNullCheckedTypedParams", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedNoParams (case 8)
#[test]
fn null_checked_no_params() {
    let src = r#"Func<int> func1 = (!!) => 42;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int> func1 = (!!) => 42; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedNoParams", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDiscard (case 9)
#[test]
fn null_checked_discard() {
    let src = r#"Func<int, int> func1 = (_!!) => 42;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<int, int> func1 = (_!!) => 42; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDiscard", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection0 (case 10)
#[test]
fn null_checked_syntax_correction_0() {
    let src = r#"Func<string, string> func0 = x!=> x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x!=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection0", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection1 (case 11)
#[test]
fn null_checked_syntax_correction_1() {
    let src = r#"Func<string, string> func1 = x !=> x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func1 = x !=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection1", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection2 (case 12)
#[test]
fn null_checked_syntax_correction_2() {
    let src = r#"Func<string, string> func2 = x != > x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func2 = x != > x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection2", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection3 (case 13)
#[test]
fn null_checked_syntax_correction_3() {
    let src = r#"Func<string, string> func3 = x! => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func3 = x! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection3", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection4 (case 14)
#[test]
fn null_checked_syntax_correction_4() {
    let src = r#"Func<string, string> func4 = x ! => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func4 = x ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection4", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection5 (case 15)
#[test]
fn null_checked_syntax_correction_5() {
    let src = r#"Func<string, string> func5 = x !!=> x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func5 = x !!=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection5", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection6 (case 16)
#[test]
fn null_checked_syntax_correction_6() {
    let src = r#"Func<string, string> func6 = x !!= > x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func6 = x !!= > x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection6", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection7 (case 17)
#[test]
fn null_checked_syntax_correction_7() {
    let src = r#"Func<string, string> func7 = x!! => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func7 = x!! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection7", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection8 (case 18)
#[test]
fn null_checked_syntax_correction_8() {
    let src = r#"Func<string, string> func8 = x! !=> x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func8 = x! !=> x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection8", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSyntaxCorrection9 (case 19)
#[test]
fn null_checked_syntax_correction_9() {
    let src = r#"Func<string, string> func9 = x! ! => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func9 = x! ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSyntaxCorrection9", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterSimpleLambdaName (case 20)
#[test]
fn braces_after_simple_lambda_name() {
    let src = r#"Func<string[], string> func0 = x[] => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = x[] => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestBracesAfterSimpleLambdaName", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterParenthesizedLambdaName (case 21)
#[test]
fn braces_after_parenthesized_lambda_name() {
    let src = r#"Func<string[], string> func0 = (x[]) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = (x[]) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestBracesAfterParenthesizedLambdaName", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestBracesAfterParenthesizedLambdaTypeAndName (case 22)
#[test]
fn braces_after_parenthesized_lambda_type_and_name() {
    let src = r#"Func<string[], string> func0 = (string x[]) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string[], string> func0 = (string x[]) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestBracesAfterParenthesizedLambdaTypeAndName", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueSimpleLambda (case 23)
#[test]
fn default_value_simple_lambda() {
    let src = r#"Func<string, string> func0 = x = null => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x = null => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValueSimpleLambda", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValue_TypedSimpleLambda (case 24)
#[test]
fn default_value_typed_simple_lambda() {
    let src = r#"var f = int x = 3 => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { var f = int x = 3 => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValue_TypedSimpleLambda", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambda1 (case 25)
#[test]
fn default_value_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValueParenthesizedLambda1", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambda2 (case 26)
#[test]
fn default_value_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValueParenthesizedLambda2", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambdaWithType1 (case 27)
#[test]
fn default_value_parenthesized_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValueParenthesizedLambdaWithType1", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestDefaultValueParenthesizedLambdaWithType2 (case 28)
#[test]
fn default_value_parenthesized_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestDefaultValueParenthesizedLambdaWithType2", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueSimpleLambda (case 29)
#[test]
fn null_checked_default_value_simple_lambda() {
    let src = r#"Func<string, string> func0 = x!! = null => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x!! = null => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDefaultValueSimpleLambda", 29, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambda1 (case 30)
#[test]
fn null_checked_default_value_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x!! = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDefaultValueParenthesizedLambda1", 30, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambda2 (case 31)
#[test]
fn null_checked_default_value_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x!! = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDefaultValueParenthesizedLambda2", 31, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambdaWithType1 (case 32)
#[test]
fn null_checked_default_value_parenthesized_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x!! = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDefaultValueParenthesizedLambdaWithType1", 32, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedDefaultValueParenthesizedLambdaWithType2 (case 33)
#[test]
fn null_checked_default_value_parenthesized_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x!! = null) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x!! = null) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedDefaultValueParenthesizedLambdaWithType2", 33, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenSimpleLambda (case 34)
#[test]
fn null_checked_space_between_simple_lambda() {
    let src = r#"Func<string, string> func0 = x! ! => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = x! ! => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSpaceBetweenSimpleLambda", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenParenthesizedLambda1 (case 35)
#[test]
fn null_checked_space_between_parenthesized_lambda_1() {
    let src = r#"Func<string, string> func0 = (x! !) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSpaceBetweenParenthesizedLambda1", 35, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenParenthesizedLambda2 (case 36)
#[test]
fn null_checked_space_between_parenthesized_lambda_2() {
    let src = r#"Func<string, string> func0 = (y, x! !) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (y, x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSpaceBetweenParenthesizedLambda2", 36, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenLambdaWithType1 (case 37)
#[test]
fn null_checked_space_between_lambda_with_type_1() {
    let src = r#"Func<string, string> func0 = (string x! !) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSpaceBetweenLambdaWithType1", 37, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.TestNullCheckedSpaceBetweenLambdaWithType2 (case 38)
#[test]
fn null_checked_space_between_lambda_with_type_2() {
    let src = r#"Func<string, string> func0 = (string y, string x! !) => x;"#;
    let span = Span::new(src);
    let src2 = r#"class C { Func<string, string> func0 = (string y, string x! !) => x; }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "TestNullCheckedSpaceBetweenLambdaWithType2", 38, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: LambdaParameterParsingTests.AsyncAwaitInLambda (case 39)
#[test]
fn async_await_in_lambda() {
    let src = r#"F(async () => await Task.FromResult(4));"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("lambda_parameter_parsing_tests", "LambdaParameterParsingTests", "AsyncAwaitInLambda", 39, CaseData::Statement { ast: &ast, src });
}

