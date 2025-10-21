// Auto-generated from Roslyn: ExpressionParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: ExpressionParsingTests.TestInterpolatedSingleLineRawString1 (case 1)
#[test]
fn interpolated_single_line_raw_string_1() {
    let src = r#"$""""""{1 + 1}"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""{1 + 1}""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "TestInterpolatedSingleLineRawString1", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.TestInterpolatedSingleLineRawString2 (case 2)
#[test]
fn interpolated_single_line_raw_string_2() {
    let src = r#"$$""""""{{{1 + 1}}}"""""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $$""""""{{{1 + 1}}}""""""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "TestInterpolatedSingleLineRawString2", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.TestInterpolatedMultiLineRawString1 (case 3)
#[test]
fn interpolated_multi_line_raw_string_1() {
    let src = r#"$""""""
    {1 + 1}
    """""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $""""""
    {1 + 1}
    """"""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "TestInterpolatedMultiLineRawString1", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.TestInterpolatedMultiLineRawString2 (case 4)
#[test]
fn interpolated_multi_line_raw_string_2() {
    let src = r#"$$""""""
    {{{1 + 1}}}
    """""""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $$""""""
    {{{1 + 1}}}
    """"""; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "TestInterpolatedMultiLineRawString2", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.ShiftOperator (case 5)
#[test]
fn shift_operator() {
    let src = r#"
class C
{
    int x = 1 << 2 << 3;
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "ShiftOperator", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: ExpressionParsingTests.TypeArgumentIndexerInitializer (case 6)
#[test]
fn type_argument_indexer_initializer() {
    let src = r#"new C { [0] = op1 < op2, [1] = true }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new C { [0] = op1 < op2, [1] = true }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "TypeArgumentIndexerInitializer", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.ConditionalExpressionInInterpolation (case 7)
#[test]
fn conditional_expression_in_interpolation() {
    let src = r#"$"{a ? b : d}""#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { $"{a ? b : d}"; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "ConditionalExpressionInInterpolation", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpression (case 8)
#[test]
fn null_coalescing_assignment_expression() {
    let src = r#"a ??= b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentExpression", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionParenthesized (case 9)
#[test]
fn null_coalescing_assignment_expression_parenthesized() {
    let src = r#"(a) ??= b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a) ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentExpressionParenthesized", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionInvocation (case 10)
#[test]
fn null_coalescing_assignment_expression_invocation() {
    let src = r#"M(a) ??= b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { M(a) ??= b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentExpressionInvocation", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionAndCoalescingOperator (case 11)
#[test]
fn null_coalescing_assignment_expression_and_coalescing_operator() {
    let src = r#"a ?? b ??= c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ?? b ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentExpressionAndCoalescingOperator", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentExpressionNested (case 12)
#[test]
fn null_coalescing_assignment_expression_nested() {
    let src = r#"a ??= b ??= c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ??= b ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentExpressionNested", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.NullCoalescingAssignmentParenthesizedNested (case 13)
#[test]
fn null_coalescing_assignment_parenthesized_nested() {
    let src = r#"(a ??= b) ??= c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (a ??= b) ??= c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "NullCoalescingAssignmentParenthesizedNested", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.IndexExpression (case 14)
#[test]
fn index_expression() {
    let src = r#"^1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "IndexExpression", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_ThreeDots (case 15)
#[test]
fn range_expression_three_dots() {
    let src = r#"1...2"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1...2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_ThreeDots", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary (case 16)
#[test]
fn range_expression_binary() {
    let src = r#"1..1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Binary", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithIndexes (case 17)
#[test]
fn range_expression_binary_with_indexes() {
    let src = r#"^5..^3"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^5..^3; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Binary_WithIndexes", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithALowerPrecedenceOperator_01 (case 18)
#[test]
fn range_expression_binary_with_alower_precedence_operator_01() {
    let src = r#"1<<2..3>>4"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1<<2..3>>4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Binary_WithALowerPrecedenceOperator_01", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithALowerPrecedenceOperator_02 (case 19)
#[test]
fn range_expression_binary_with_alower_precedence_operator_02() {
    let src = r#"1<<2..3>>>4"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1<<2..3>>>4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Binary_WithALowerPrecedenceOperator_02", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Binary_WithAHigherPrecedenceOperator (case 20)
#[test]
fn range_expression_binary_with_ahigher_precedence_operator() {
    let src = r#"1+2..3-4"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1+2..3-4; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Binary_WithAHigherPrecedenceOperator", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryBadLeft (case 21)
#[test]
fn range_expression_unary_bad_left() {
    let src = r#"a*..b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a*..b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_UnaryBadLeft", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_BinaryLeftPlus (case 22)
#[test]
fn range_expression_binary_left_plus() {
    let src = r#"a + b..c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b..c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_BinaryLeftPlus", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryLeftPlus (case 23)
#[test]
fn range_expression_unary_left_plus() {
    let src = r#"a + b.."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_UnaryLeftPlus", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryRightMult (case 24)
#[test]
fn range_expression_unary_right_mult() {
    let src = r#"a.. && b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a.. && b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_UnaryRightMult", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_UnaryRightMult2 (case 25)
#[test]
fn range_expression_unary_right_mult_2() {
    let src = r#"..a && b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..a && b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_UnaryRightMult2", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_NotCast (case 26)
#[test]
fn range_expression_not_cast() {
    let src = r#"(Offset)..(Offset + Count)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Offset)..(Offset + Count); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_NotCast", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Right (case 27)
#[test]
fn range_expression_right() {
    let src = r#"..1"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..1; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Right", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Right_WithIndexes (case 28)
#[test]
fn range_expression_right_with_indexes() {
    let src = r#"..^3"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..^3; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Right_WithIndexes", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Left (case 29)
#[test]
fn range_expression_left() {
    let src = r#"1.."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Left", 29, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_Left_WithIndexes (case 30)
#[test]
fn range_expression_left_with_indexes() {
    let src = r#"^5.."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ^5..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_Left_WithIndexes", 30, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_NoOperands (case 31)
#[test]
fn range_expression_no_operands() {
    let src = r#".."#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_NoOperands", 31, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_NoOperands_WithOtherOperators (case 32)
#[test]
fn range_expression_no_operands_with_other_operators() {
    let src = r#"1+..<<2"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1+..<<2; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_NoOperands_WithOtherOperators", 32, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_RightOperand (case 33)
#[test]
fn range_expression_method_invocation_right_operand() {
    let src = r#"..2 .ToString()"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ..2 .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_MethodInvocation_RightOperand", 33, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_MethodInvocation_TwoOperands (case 34)
#[test]
fn range_expression_method_invocation_two_operands() {
    let src = r#"1..2 .ToString()"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1..2 .ToString(); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_MethodInvocation_TwoOperands", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.RangeExpression_ConditionalAccessExpression_02 (case 35)
#[test]
fn range_expression_conditional_access_expression_02() {
    let src = r#"c?.b..a"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { c?.b..a; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "RangeExpression_ConditionalAccessExpression_02", 35, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.BaseExpression_01 (case 36)
#[test]
fn base_expression_01() {
    let src = r#"base"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { base; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "BaseExpression_01", 36, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: ExpressionParsingTests.UnsignedRightShift_01 (case 37)
#[test]
fn unsigned_right_shift_01() {
    let src = r#"x >>> y"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x >>> y; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("expression_parsing_tests", "ExpressionParsingTests", "UnsignedRightShift_01", 37, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

