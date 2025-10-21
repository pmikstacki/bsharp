// Auto-generated from Roslyn: SwitchExpressionParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression5 (case 1)
#[test]
fn errant_case_in_switch_expression_5() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case,
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression5", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression7 (case 2)
#[test]
fn errant_case_in_switch_expression_7() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression7", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression8 (case 3)
#[test]
fn errant_case_in_switch_expression_8() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when true
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression8", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression9 (case 4)
#[test]
fn errant_case_in_switch_expression_9() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when true =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression9", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression10 (case 5)
#[test]
fn errant_case_in_switch_expression_10() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case true =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression10", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression11 (case 6)
#[test]
fn errant_case_in_switch_expression_11() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression11", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression12 (case 7)
#[test]
fn errant_case_in_switch_expression_12() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        case when =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression12", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression13 (case 8)
#[test]
fn errant_case_in_switch_expression_13() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when case
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression13", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantCaseInSwitchExpression14 (case 9)
#[test]
fn errant_case_in_switch_expression_14() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when case 0
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantCaseInSwitchExpression14", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression1 (case 10)
#[test]
fn errant_when_in_switch_expression_1() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression1", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression2 (case 11)
#[test]
fn errant_when_in_switch_expression_2() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when,
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression2", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression4 (case 12)
#[test]
fn errant_when_in_switch_expression_4() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression4", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression5 (case 13)
#[test]
fn errant_when_in_switch_expression_5() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when => true
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression5", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression6 (case 14)
#[test]
fn errant_when_in_switch_expression_6() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression6", 14, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression7 (case 15)
#[test]
fn errant_when_in_switch_expression_7() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true,
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression7", 15, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestErrantWhenInSwitchExpression9 (case 16)
#[test]
fn errant_when_in_switch_expression_9() {
    let src = r#"
            class C
            {
                public static int X()
                    => 5 switch
                    {
                        when true =>
                    };
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestErrantWhenInSwitchExpression9", 16, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern1 (case 17)
#[test]
fn nullable_type_in_pattern_1() {
    let src = r#"
            obj switch
            {
                Type? => 1
            }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestNullableTypeInPattern1", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern2 (case 18)
#[test]
fn nullable_type_in_pattern_2() {
    let src = r#"
            obj switch
            {
                Type? varName => 1
            }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestNullableTypeInPattern2", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern3 (case 19)
#[test]
fn nullable_type_in_pattern_3() {
    let src = r#"
            obj switch
            {
                Type? when x > 0 => 1
            }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? when x > 0 => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestNullableTypeInPattern3", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: SwitchExpressionParsingTests.TestNullableTypeInPattern4 (case 20)
#[test]
fn nullable_type_in_pattern_4() {
    let src = r#"
            obj switch
            {
                Type? varName when x > 0 => 1
            }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { 
            obj switch
            {
                Type? varName when x > 0 => 1
            }
            ; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("switch_expression_parsing_tests", "SwitchExpressionParsingTests", "TestNullableTypeInPattern4", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

