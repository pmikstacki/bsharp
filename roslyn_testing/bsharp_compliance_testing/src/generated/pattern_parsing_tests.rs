// Auto-generated from Roslyn: PatternParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: PatternParsingTests.ThrowExpression (case 1)
#[test]
fn throw_expression() {
    let src = r#"
class C
{
    int x = y ?? throw null;
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ThrowExpression", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_1 (case 2)
#[test]
fn is_pattern_precedence_1() {
    let src = r#"A is B < C, D > [ ]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_1", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_2 (case 3)
#[test]
fn is_pattern_precedence_2() {
    let src = r#"A < B > C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B > C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_2", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 4)
#[test]
fn is_pattern_precedence_3() {
    let src = r#"e is A<B> && e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> && e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 5)
#[test]
fn is_pattern_precedence_3_case_2() {
    let src = r#"e is A<B> || e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> || e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 6)
#[test]
fn is_pattern_precedence_3_case_3() {
    let src = r#"e is A<B> ^ e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> ^ e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 7)
#[test]
fn is_pattern_precedence_3_case_4() {
    let src = r#"e is A<B> | e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> | e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 8)
#[test]
fn is_pattern_precedence_3_case_5() {
    let src = r#"e is A<B> & e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> & e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 9)
#[test]
fn is_pattern_precedence_3_case_6() {
    let src = r#"e is A<B>[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B>[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 10)
#[test]
fn is_pattern_precedence_3_case_7() {
    let src = r#"new { X = e is A<B> }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new { X = e is A<B> }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 11)
#[test]
fn is_pattern_precedence_3_case_8() {
    let src = r#"e is A<B>"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B>; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 12)
#[test]
fn is_pattern_precedence_3_case_9() {
    let src = r#"(item is Dictionary<string, object>[])"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (item is Dictionary<string, object>[]); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 13)
#[test]
fn is_pattern_precedence_3_case_10() {
    let src = r#"A is B < C, D > [ ]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 14)
#[test]
fn is_pattern_precedence_3_case_11() {
    let src = r#"A is B < C, D > [ ] E"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B < C, D > [ ] E; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsPatternPrecedence_3 (case 15)
#[test]
fn is_pattern_precedence_3_case_12() {
    let src = r#"A < B > C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B > C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsPatternPrecedence_3", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.TypeDisambiguation_02 (case 16)
#[test]
fn type_disambiguation_02() {
    let src = r#"
                var r = a is X<T> // should disambiguate as a type here
                        is bool;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "TypeDisambiguation_02", 16, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence03 (case 17)
#[test]
fn pattern_expression_precedence_03() {
    let src = r#"A is null == B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null == B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PatternExpressionPrecedence03", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence04 (case 18)
#[test]
fn pattern_expression_precedence_04() {
    let src = r#"A is null & B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null & B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PatternExpressionPrecedence04", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence05 (case 19)
#[test]
fn pattern_expression_precedence_05() {
    let src = r#"A is null && B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null && B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PatternExpressionPrecedence05", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence05b (case 20)
#[test]
fn pattern_expression_precedence_05_b() {
    let src = r#"A is null || B"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is null || B; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PatternExpressionPrecedence05b", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.PatternExpressionPrecedence07 (case 21)
#[test]
fn pattern_expression_precedence_07() {
    let src = r#"switch (array) {
case KeyValuePair<string, DateTime>[] pairs1:
case KeyValuePair<String, DateTime>[] pairs2:
    break;
}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PatternExpressionPrecedence07", 21, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_01 (case 22)
#[test]
fn array_of_pointer_01() {
    let src = r#"A is B***"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_01", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_01b (case 23)
#[test]
fn array_of_pointer_01_b() {
    let src = r#"A is B*** C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B*** C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_01b", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_02 (case 24)
#[test]
fn array_of_pointer_02() {
    let src = r#"A is B***[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_02", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_03 (case 25)
#[test]
fn array_of_pointer_03() {
    let src = r#"A is B***[] C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A is B***[] C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_03", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_04 (case 26)
#[test]
fn array_of_pointer_04() {
    let src = r#"(B*** C, D)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (B*** C, D); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_04", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_04b (case 27)
#[test]
fn array_of_pointer_04_b() {
    let src = r#"(B*** C)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (B*** C); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_04b", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_06 (case 28)
#[test]
fn array_of_pointer_06() {
    let src = r#"(D, B*** C)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (D, B*** C); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_06", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_08 (case 29)
#[test]
fn array_of_pointer_08() {
    let src = r#"switch (e) { case B*** C: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_08", 29, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ArrayOfPointer_09 (case 30)
#[test]
fn array_of_pointer_09() {
    let src = r#"switch (e) { case B***[] C: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ArrayOfPointer_09", 30, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NameofInPattern_02 (case 31)
#[test]
fn nameof_in_pattern_02() {
    let src = r#"switch (e) { case nameof(n): ; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NameofInPattern_02", 31, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpression_01 (case 32)
#[test]
fn parenthesized_expression_01() {
    let src = r#"switch (e) { case (((3))): ; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpression_01", 32, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpression_02 (case 33)
#[test]
fn parenthesized_expression_02() {
    let src = r#"switch (e) { case (((3))) when true: ; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpression_02", 33, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.SwitchExpression03 (case 34)
#[test]
fn switch_expression_03() {
    let src = r#"1 switch { (a, b, c) => d }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { (a, b, c) => d }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "SwitchExpression03", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.VarIsContextualKeywordForPatterns01 (case 35)
#[test]
fn var_is_contextual_keyword_for_patterns_01() {
    let src = r#"switch (e) { case var: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "VarIsContextualKeywordForPatterns01", 35, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.VarIsContextualKeywordForPatterns02 (case 36)
#[test]
fn var_is_contextual_keyword_for_patterns_02() {
    let src = r#"if (e is var) {}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "VarIsContextualKeywordForPatterns02", 36, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.RecursivePattern_07 (case 37)
#[test]
fn recursive_pattern_07() {
    let src = r#"var x = o is Type x;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "RecursivePattern_07", 37, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.RecursivePattern_10 (case 38)
#[test]
fn recursive_pattern_10() {
    let src = r#"var x = o is Type { Prop : 3 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "RecursivePattern_10", 38, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.RecursivePattern_11 (case 39)
#[test]
fn recursive_pattern_11() {
    let src = r#"var x = o is { Prop : 3 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "RecursivePattern_11", 39, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.DiscardInSwitchExpression (case 40)
#[test]
fn discard_in_switch_expression() {
    let src = r#"e switch { _ => 1 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { _ => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInSwitchExpression", 40, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_01a (case 41)
#[test]
fn discard_in_switch_statement_01_a() {
    let src = r#"switch(e) { case _: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInSwitchStatement_01a", 41, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_01b (case 42)
#[test]
fn discard_in_switch_statement_01_b() {
    let src = r#"switch(e) { case _: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInSwitchStatement_01b", 42, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.DiscardInSwitchStatement_02 (case 43)
#[test]
fn discard_in_switch_statement_02() {
    let src = r#"switch(e) { case _ when true: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInSwitchStatement_02", 43, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.DiscardInRecursivePattern_01 (case 44)
#[test]
fn discard_in_recursive_pattern_01() {
    let src = r#"e is (_, _)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is (_, _); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInRecursivePattern_01", 44, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.DiscardInRecursivePattern_02 (case 45)
#[test]
fn discard_in_recursive_pattern_02() {
    let src = r#"e is { P: _ }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is { P: _ }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "DiscardInRecursivePattern_02", 45, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.NotDiscardInIsTypeExpression (case 46)
#[test]
fn not_discard_in_is_type_expression() {
    let src = r#"e is _"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is _; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NotDiscardInIsTypeExpression", 46, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsNullableArray02 (case 47)
#[test]
fn is_nullable_array_02() {
    let src = r#"o is A[] ? b && c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A[] ? b && c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsNullableArray02", 47, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.IsNullableType02 (case 48)
#[test]
fn is_nullable_type_02() {
    let src = r#"o is A? ? b : c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { o is A? ? b : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "IsNullableType02", 48, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.TrailingCommaInSwitchExpression_01 (case 49)
#[test]
fn trailing_comma_in_switch_expression_01() {
    let src = r#"1 switch { 1 => 2, }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 1 switch { 1 => 2, }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "TrailingCommaInSwitchExpression_01", 49, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_01 (case 50)
#[test]
fn parenthesized_expression_in_pattern_01() {
    let src = r#"switch (e) {
    case (('C') << 24) + (('g') << 16) + (('B') << 8) + 'I': break;
}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpressionInPattern_01", 50, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_02 (case 51)
#[test]
fn parenthesized_expression_in_pattern_02() {
    let src = r#"switch (e) {
    case ((2) + (2)): break;
}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpressionInPattern_02", 51, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_03 (case 52)
#[test]
fn parenthesized_expression_in_pattern_03() {
    let src = r#"switch (e) {
    case ((2 + 2) - 2): break;
}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpressionInPattern_03", 52, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedExpressionInPattern_05 (case 53)
#[test]
fn parenthesized_expression_in_pattern_05() {
    let src = r#"switch (e) {
    case ((2 << 2) | 2): break;
}"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedExpressionInPattern_05", 53, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ChainedSwitchExpression_02 (case 54)
#[test]
fn chained_switch_expression_02() {
    let src = r#"a < b switch { 1 => 2 } < c switch { 2 => 3 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a < b switch { 1 => 2 } < c switch { 2 => 3 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ChainedSwitchExpression_02", 54, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_01 (case 55)
#[test]
fn switch_expression_precedence_01() {
    let src = r#"a < b switch { true => 1 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a < b switch { true => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "SwitchExpressionPrecedence_01", 55, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.SwitchExpressionPrecedence_02 (case 56)
#[test]
fn switch_expression_precedence_02() {
    let src = r#"a == b switch { true => 1 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a == b switch { true => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "SwitchExpressionPrecedence_02", 56, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.ParenthesizedNamedConstantPatternInSwitchExpression (case 57)
#[test]
fn parenthesized_named_constant_pattern_in_switch_expression() {
    let src = r#"e switch { (X) => 1 }"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e switch { (X) => 1 }; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedNamedConstantPatternInSwitchExpression", 57, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.GenericPropertyPattern (case 58)
#[test]
fn generic_property_pattern() {
    let src = r#"e is A<B> {}"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { e is A<B> {}; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "GenericPropertyPattern", 58, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.RelationalPatternPrecedence_03 (case 59)
#[test]
fn relational_pattern_precedence_03() {
    let src = r#"_ = e is < 4;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "RelationalPatternPrecedence_03", 59, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.TypePattern_05 (case 60)
#[test]
fn type_pattern_05() {
    let src = r#"_ = e switch { T(int) => 1, };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "TypePattern_05", 60, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.TypePattern_06 (case 61)
#[test]
fn type_pattern_06() {
    let src = r#"_ = e switch { int => 1, long => 2, };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "TypePattern_06", 61, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.TypePattern_07 (case 62)
#[test]
fn type_pattern_07() {
    let src = r#"_ = e is (int) or string;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "TypePattern_07", 62, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.CompoundPattern_01 (case 63)
#[test]
fn compound_pattern_01() {
    let src = r#"bool isLetter(char c) => c is >= 'a' and <= 'z' or >= 'A' and <= 'Z';"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "CompoundPattern_01", 63, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_01 (case 64)
#[test]
fn combinator_as_designator_01() {
    let src = r#"_ = e is int and;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "CombinatorAsDesignator_01", 64, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_03 (case 65)
#[test]
fn combinator_as_designator_03() {
    let src = r#"_ = e is int and && b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "CombinatorAsDesignator_03", 65, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_06 (case 66)
#[test]
fn combinator_as_designator_06() {
    let src = r#"_ = e is int and ?? Z;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "CombinatorAsDesignator_06", 66, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.CombinatorAsDesignator_07 (case 67)
#[test]
fn combinator_as_designator_07() {
    let src = r#"_ = e is int and ? a : b;"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "CombinatorAsDesignator_07", 67, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithBlockLambda (case 68)
#[test]
fn precedence_inversion_with_block_lambda() {
    let src = r#"() => {} + d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { () => {} + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PrecedenceInversionWithBlockLambda", 68, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.PrecedenceInversionWithAnonymousMethod (case 69)
#[test]
fn precedence_inversion_with_anonymous_method() {
    let src = r#"delegate {} + d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { delegate {} + d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "PrecedenceInversionWithAnonymousMethod", 69, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PatternParsingTests.OneElementPositional_01 (case 70)
#[test]
fn one_element_positional_01() {
    let src = r#"_ = e is (3);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "OneElementPositional_01", 70, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.OneElementPositional_02 (case 71)
#[test]
fn one_element_positional_02() {
    let src = r#"_ = e is (A);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "OneElementPositional_02", 71, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.OneElementPositional_03 (case 72)
#[test]
fn one_element_positional_03() {
    let src = r#"_ = e is (int);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "OneElementPositional_03", 72, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ConjunctiveFollowedByPropertyPattern_01 (case 73)
#[test]
fn conjunctive_followed_by_property_pattern_01() {
    let src = r#"switch (e) { case {} and {}: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ConjunctiveFollowedByPropertyPattern_01", 73, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_01 (case 74)
#[test]
fn parenthesized_relational_pattern_01() {
    let src = r#"_ = e is (>= 1);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedRelationalPattern_01", 74, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_02 (case 75)
#[test]
fn parenthesized_relational_pattern_02() {
    let src = r#"_ = e switch { (>= 1) => 1 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedRelationalPattern_02", 75, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_03 (case 76)
#[test]
fn parenthesized_relational_pattern_03() {
    let src = r#"bool isAsciiLetter(char c) => c is (>= 'A' and <= 'Z') or (>= 'a' and <= 'z');"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedRelationalPattern_03", 76, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ParenthesizedRelationalPattern_04 (case 77)
#[test]
fn parenthesized_relational_pattern_04() {
    let src = r#"_ = e is (<= 1, >= 2);"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ParenthesizedRelationalPattern_04", 77, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.GenericTypeAsTypePatternInSwitchExpression (case 78)
#[test]
fn generic_type_as_type_pattern_in_switch_expression() {
    let src = r#"_ = e switch { List<X> => 1, List<Y> => 2, };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "GenericTypeAsTypePatternInSwitchExpression", 78, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_PredefinedType (case 79)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_predefined_type() {
    let src = r#"_ = e switch { int? => 1 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchExpression_PredefinedType", 79, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_PredefinedType (case 80)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_predefined_type() {
    let src = r#"switch(a) { case int?: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchStatement_PredefinedType", 80, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized (case 81)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_predefined_type_parenthesized() {
    let src = r#"_ = e switch { (int?) => 1 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchExpression_PredefinedType_Parenthesized", 81, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized (case 82)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_predefined_type_parenthesized() {
    let src = r#"switch(a) { case (int?): break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchStatement_PredefinedType_Parenthesized", 82, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression (case 83)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression() {
    let src = r#"_ = e switch { a? => 1 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchExpression", 83, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement (case 84)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement() {
    let src = r#"switch(a) { case a?: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchStatement", 84, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchExpression_Parenthesized (case 85)
#[test]
fn nullable_type_as_type_pattern_in_switch_expression_parenthesized() {
    let src = r#"_ = e switch { (a?) => 1 };"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchExpression_Parenthesized", 85, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.NullableTypeAsTypePatternInSwitchStatement_Parenthesized (case 86)
#[test]
fn nullable_type_as_type_pattern_in_switch_statement_parenthesized() {
    let src = r#"switch(a) { case (a?): break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "NullableTypeAsTypePatternInSwitchStatement_Parenthesized", 86, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ConditionalAsConstantPatternInSwitchStatement (case 87)
#[test]
fn conditional_as_constant_pattern_in_switch_statement() {
    let src = r#"switch(a) { case a?x:y: break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ConditionalAsConstantPatternInSwitchStatement", 87, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: PatternParsingTests.ConditionalAsConstantPatternInSwitchStatement_Parenthesized (case 88)
#[test]
fn conditional_as_constant_pattern_in_switch_statement_parenthesized() {
    let src = r#"switch(a) { case (a?x:y): break; }"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("pattern_parsing_tests", "PatternParsingTests", "ConditionalAsConstantPatternInSwitchStatement_Parenthesized", 88, CaseData::Statement { ast: &ast, src });
}

