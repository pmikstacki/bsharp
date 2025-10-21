// Auto-generated from Roslyn: CollectionExpressionParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: CollectionExpressionParsingTests.CollectionExpressionParsingDoesNotProduceLangVersionError (case 1)
#[test]
fn collection_expression_parsing_does_not_produce_lang_version_error() {
    let src = r#"[A, B]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CollectionExpressionParsingDoesNotProduceLangVersionError", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionDotAccess (case 2)
#[test]
fn expression_dot_access() {
    let src = r#"_ = [A, B].C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ExpressionDotAccess", 2, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelDotAccess (case 3)
#[test]
fn top_level_dot_access() {
    let src = r#"[A, B].C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TopLevelDotAccess", 3, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionNullSafeAccess (case 4)
#[test]
fn expression_null_safe_access() {
    let src = r#"_ = [A, B]?.C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ExpressionNullSafeAccess", 4, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TopLevelNullSafeAccess (case 5)
#[test]
fn top_level_null_safe_access() {
    let src = r#"[A, B]?.C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TopLevelNullSafeAccess", 5, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.AttributeOnTopLevelDotAccessStatement (case 6)
#[test]
fn attribute_on_top_level_dot_access_statement() {
    let src = r#"[A] [B].C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "AttributeOnTopLevelDotAccessStatement", 6, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.AttemptToImmediatelyIndexInTopLevelStatement (case 7)
#[test]
fn attempt_to_immediately_index_in_top_level_statement() {
    let src = r#"["A", "B"][0].C();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "AttemptToImmediatelyIndexInTopLevelStatement", 7, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.AlwaysParsedAsAttributeInsideNamespace (case 8)
#[test]
fn always_parsed_as_attribute_inside_namespace() {
    let src = r#"
                namespace A;
                [B].C();
                "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "AlwaysParsedAsAttributeInsideNamespace", 8, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionIs (case 9)
#[test]
fn expression_is() {
    let src = r#"_ = [A, B] is [A, B];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ExpressionIs", 9, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.ExpressionWith (case 10)
#[test]
fn expression_with() {
    let src = r#"_ = [A, B] with { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ExpressionWith", 10, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.BinaryOperator (case 11)
#[test]
fn binary_operator() {
    let src = r#"_ = [A, B] + [C, D];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "BinaryOperator", 11, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.EmptyCollection (case 12)
#[test]
fn empty_collection() {
    let src = r#"_ = [];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "EmptyCollection", 12, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.CollectionOfEmptyCollection (case 13)
#[test]
fn collection_of_empty_collection() {
    let src = r#"_ = [[]];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CollectionOfEmptyCollection", 13, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity1 (case 14)
#[test]
fn conditional_ambiguity_1() {
    let src = r#"[a ? [b] : c]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [a ? [b] : c]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity1", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity1A (case 15)
#[test]
fn conditional_ambiguity_1_a() {
    let src = r#"[A] ? [B] : C"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] ? [B] : C; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity1A", 15, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity3 (case 16)
#[test]
fn conditional_ambiguity_3() {
    let src = r#"a ? [b] : c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? [b] : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity3", 16, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity3A (case 17)
#[test]
fn conditional_ambiguity_3_a() {
    let src = r#"a ? [b].M() : c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? [b].M() : c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity3A", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity4 (case 18)
#[test]
fn conditional_ambiguity_4() {
    let src = r#"a ? b?[c] : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity4", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity4A (case 19)
#[test]
fn conditional_ambiguity_4_a() {
    let src = r#"a ? b?[c].M() : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c].M() : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity4A", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity5 (case 20)
#[test]
fn conditional_ambiguity_5() {
    let src = r#"a ? b ? [c] : d : e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b ? [c] : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity5", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity5A (case 21)
#[test]
fn conditional_ambiguity_5_a() {
    let src = r#"a ? b ? [c].M() : d : e"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b ? [c].M() : d : e; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity5A", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity6 (case 22)
#[test]
fn conditional_ambiguity_6() {
    let src = r#"a?[c] ? b : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c] ? b : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity6", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity6A (case 23)
#[test]
fn conditional_ambiguity_6_a() {
    let src = r#"a?[c].M() ? b : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a?[c].M() ? b : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity6A", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity8 (case 24)
#[test]
fn conditional_ambiguity_8() {
    let src = r#"a ? b?[() => { var v = x ? [y] : z; }] : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[() => { var v = x ? [y] : z; }] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity8", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity9 (case 25)
#[test]
fn conditional_ambiguity_9() {
    let src = r#"a ? b?[delegate { var v = x ? [y] : z; }] : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[delegate { var v = x ? [y] : z; }] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity9", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity10 (case 26)
#[test]
fn conditional_ambiguity_10() {
    let src = r#"a ? b?[() => x ? [y] : z] : d"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[() => x ? [y] : z] : d; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity10", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity11 (case 27)
#[test]
fn conditional_ambiguity_11() {
    let src = r#"a ? b?[c] : d ? e?[f] : g"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e?[f] : g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity11", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.ConditionalAmbiguity12A (case 28)
#[test]
fn conditional_ambiguity_12_a() {
    let src = r#"a ? b?[c] : d ? e ? f?[g] : h : i"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a ? b?[c] : d ? e ? f?[g] : h : i; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ConditionalAmbiguity12A", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity1 (case 29)
#[test]
fn cast_versus_index_ambiguity_1() {
    let src = r#"(type)[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (type)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity1", 29, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity2 (case 30)
#[test]
fn cast_versus_index_ambiguity_2() {
    let src = r#"(ImmutableArray<int>)[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (ImmutableArray<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity2", 30, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity3 (case 31)
#[test]
fn cast_versus_index_ambiguity_3() {
    let src = r#"(Dotted.ImmutableArray<int>)[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (Dotted.ImmutableArray<int>)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity3", 31, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity7 (case 32)
#[test]
fn cast_versus_index_ambiguity_7() {
    let src = r#"(List<int>?)[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (List<int>?)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity7", 32, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity8 (case 33)
#[test]
fn cast_versus_index_ambiguity_8() {
    let src = r#"(int[])[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int[])[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity8", 33, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity12 (case 34)
#[test]
fn cast_versus_index_ambiguity_12() {
    let src = r#"(int[]?)[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int[]?)[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity12", 34, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity13 (case 35)
#[test]
fn cast_versus_index_ambiguity_13() {
    let src = r#"(int?[])[1, 2, 3]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (int?[])[1, 2, 3]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity13", 35, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity14 (case 36)
#[test]
fn cast_versus_index_ambiguity_14() {
    let src = r#"(type)([1, 2, 3])"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (type)([1, 2, 3]); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity14", 36, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity23 (case 37)
#[test]
fn cast_versus_index_ambiguity_23() {
    let src = r#"(A[])[0]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A[])[0]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity23", 37, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_A (case 38)
#[test]
fn cast_versus_index_ambiguity_24_a() {
    let src = r#"(A)[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity24_A", 38, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_B (case 39)
#[test]
fn cast_versus_index_ambiguity_24_b() {
    let src = r#"(A)[1]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity24_B", 39, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity24_D (case 40)
#[test]
fn cast_versus_index_ambiguity_24_d() {
    let src = r#"(A)[..B]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A)[..B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity24_D", 40, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity25 (case 41)
#[test]
fn cast_versus_index_ambiguity_25() {
    let src = r#"(A[])[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (A[])[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity25", 41, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity30 (case 42)
#[test]
fn cast_versus_index_ambiguity_30() {
    let src = r#"(ImmutableArray<List<Int32>>)[[1]]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (ImmutableArray<List<Int32>>)[[1]]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity30", 42, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity31 (case 43)
#[test]
fn cast_versus_index_ambiguity_31() {
    let src = r#"var x = (A<B>)[1];"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity31", 43, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: CollectionExpressionParsingTests.CastVersusIndexAmbiguity31_GlobalStatement (case 44)
#[test]
fn cast_versus_index_ambiguity_31_global_statement() {
    let src = r#"var x = (A<B>)[1];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CastVersusIndexAmbiguity31_GlobalStatement", 44, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.SpreadOfQuery (case 45)
#[test]
fn spread_of_query() {
    let src = r#"[.. from x in y select x]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. from x in y select x]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "SpreadOfQuery", 45, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpression1 (case 46)
#[test]
fn invoked_collection_expression_1() {
    let src = r#"[A, B]()"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A, B](); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpression1", 46, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpression2 (case 47)
#[test]
fn invoked_collection_expression_2() {
    let src = r#"++[A, B]()"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ++[A, B](); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpression2", 47, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNegatedLiteral (case 48)
#[test]
fn negated_literal() {
    let src = r#"-[A]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { -[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNegatedLiteral", 48, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNullCoalescing1 (case 49)
#[test]
fn null_coalescing_1() {
    let src = r#"[A] ?? [B]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A] ?? [B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNullCoalescing1", 49, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNullCoalescing2 (case 50)
#[test]
fn null_coalescing_2() {
    let src = r#"[..x ?? y]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x ?? y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNullCoalescing2", 50, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNullSuppression (case 51)
#[test]
fn null_suppression() {
    let src = r#"[A]!"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A]!; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNullSuppression", 51, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestPreIncrement (case 52)
#[test]
fn pre_increment() {
    let src = r#"++[A]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { ++[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestPreIncrement", 52, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestPostIncrement (case 53)
#[test]
fn post_increment() {
    let src = r#"[A]++"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A]++; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestPostIncrement", 53, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitParsedAsElementAccess (case 54)
#[test]
fn await_parsed_as_element_access() {
    let src = r#"await [A]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { await [A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestAwaitParsedAsElementAccess", 54, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitParsedAsElementAccessTopLevel (case 55)
#[test]
fn await_parsed_as_element_access_top_level() {
    let src = r#"await [A];"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestAwaitParsedAsElementAccessTopLevel", 55, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitInAsyncContext (case 56)
#[test]
fn await_in_async_context() {
    let src = r#"
class C
{
    async void F()
    {
        await [A];
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestAwaitInAsyncContext", 56, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TestAwaitInNonAsyncContext (case 57)
#[test]
fn await_in_non_async_context() {
    let src = r#"
class C
{
    void F()
    {
        await [A];
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestAwaitInNonAsyncContext", 57, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TestSimpleSpread (case 58)
#[test]
fn simple_spread() {
    let src = r#"[..e]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestSimpleSpread", 58, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange1 (case 59)
#[test]
fn spread_of_range_1() {
    let src = r#"[.. ..]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. ..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestSpreadOfRange1", 59, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange2 (case 60)
#[test]
fn spread_of_range_2() {
    let src = r#"[.. ..e]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. ..e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestSpreadOfRange2", 60, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange3 (case 61)
#[test]
fn spread_of_range_3() {
    let src = r#"[.. e..]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. e..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestSpreadOfRange3", 61, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestSpreadOfRange4 (case 62)
#[test]
fn spread_of_range_4() {
    let src = r#"[.. e1..e2]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [.. e1..e2]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestSpreadOfRange4", 62, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestThrowExpression (case 63)
#[test]
fn throw_expression() {
    let src = r#"[..throw e]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..throw e]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestThrowExpression", 63, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestMemberAccess (case 64)
#[test]
fn member_access() {
    let src = r#"[..x.y]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x.y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestMemberAccess", 64, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestAssignment (case 65)
#[test]
fn assignment() {
    let src = r#"[..x = y]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x = y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestAssignment", 65, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestLambda (case 66)
#[test]
fn lambda() {
    let src = r#"[..x => y]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x => y]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestLambda", 66, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestConditional (case 67)
#[test]
fn conditional() {
    let src = r#"[..x ? y : z]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..x ? y : z]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestConditional", 67, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestPartialRange (case 68)
#[test]
fn partial_range() {
    let src = r#"[..e..]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [..e..]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestPartialRange", 68, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNewArray1 (case 69)
#[test]
fn new_array_1() {
    let src = r#"new T?[1]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T?[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNewArray1", 69, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNewArray3 (case 70)
#[test]
fn new_array_3() {
    let src = r#"new T[]?[1]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T[]?[1]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNewArray3", 70, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestNewArray5 (case 71)
#[test]
fn new_array_5() {
    let src = r#"new T[]?[1].Length"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { new T[]?[1].Length; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestNewArray5", 71, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.TestError6 (case 72)
#[test]
fn error_6() {
    let src = r#"[....]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [....]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TestError6", 72, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.GenericNameWithBrackets1 (case 73)
#[test]
fn generic_name_with_brackets_1() {
    let src = r#"A < B?[] > D"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { A < B?[] > D; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "GenericNameWithBrackets1", 73, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Addressof1 (case 74)
#[test]
fn addressof_1() {
    let src = r#"&[A]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Addressof1", 74, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Addressof2 (case 75)
#[test]
fn addressof_2() {
    let src = r#"&[A, B]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Addressof2", 75, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Addressof3 (case 76)
#[test]
fn addressof_3() {
    let src = r#"&[A, B][C]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { &[A, B][C]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Addressof3", 76, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Deref1 (case 77)
#[test]
fn deref_1() {
    let src = r#"*[]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Deref1", 77, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Deref2 (case 78)
#[test]
fn deref_2() {
    let src = r#"*[A]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Deref2", 78, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Deref3 (case 79)
#[test]
fn deref_3() {
    let src = r#"*[A, B]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A, B]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Deref3", 79, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.Deref4 (case 80)
#[test]
fn deref_4() {
    let src = r#"*[A, B][C]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { *[A, B][C]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "Deref4", 80, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda1 (case 81)
#[test]
fn literal_containing_lambda_1() {
    let src = r#"_ = [Main, () => { }]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [Main, () => { }]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda1", 81, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda2 (case 82)
#[test]
fn literal_containing_lambda_2() {
    let src = r#"_ = [() => { }, () => { }]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [() => { }, () => { }]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda2", 82, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda3 (case 83)
#[test]
fn literal_containing_lambda_3() {
    let src = r#"_ = [() => { }, Main]"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { _ = [() => { }, Main]; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda3", 83, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda4 (case 84)
#[test]
fn literal_containing_lambda_4() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main, () => { }]);
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda4", 84, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda5 (case 85)
#[test]
fn literal_containing_lambda_5() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main, Main, () => { }]);
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda5", 85, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.LiteralContainingLambda6 (case 86)
#[test]
fn literal_containing_lambda_6() {
    let src = r#"
            using System;
            class Program
            {
                static void F(Action[] a) { }
                static void Main()
                {
                    F([Main(), () => { }]);
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LiteralContainingLambda6", 86, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess1 (case 87)
#[test]
fn member_access_1() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [1].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess1", 87, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess1A (case 88)
#[test]
fn member_access_1_a() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [Main].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess1A", 88, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess2 (case 89)
#[test]
fn member_access_2() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [1]?.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess2", 89, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess2A (case 90)
#[test]
fn member_access_2_a() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [Main]?.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess2A", 90, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess3 (case 91)
#[test]
fn member_access_3() {
    let src = r#"
            [1].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess3", 91, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess3A (case 92)
#[test]
fn member_access_3_a() {
    let src = r#"
            [Main].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess3A", 92, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess4 (case 93)
#[test]
fn member_access_4() {
    let src = r#"
            [1]?.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess4", 93, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess4A (case 94)
#[test]
fn member_access_4_a() {
    let src = r#"
            [Main]?.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess4A", 94, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess5 (case 95)
#[test]
fn member_access_5() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [1][0].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess5", 95, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess5A (case 96)
#[test]
fn member_access_5_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [1][0].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess5A", 96, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess6 (case 97)
#[test]
fn member_access_6() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [1][Main].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess6", 97, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess6A (case 98)
#[test]
fn member_access_6_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [1][Main].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess6A", 98, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess7 (case 99)
#[test]
fn member_access_7() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [Main][1].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess7", 99, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess7A (case 100)
#[test]
fn member_access_7_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [Main][1].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess7A", 100, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess8 (case 101)
#[test]
fn member_access_8() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    // Indexing into collection, then invoking member.
                    [Main][Main].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess8", 101, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess8A (case 102)
#[test]
fn member_access_8_a() {
    let src = r#"
            // Indexing into collection, then invoking member.
            [Main][Main].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess8A", 102, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess9 (case 103)
#[test]
fn member_access_9() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess9", 103, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess9A (case 104)
#[test]
fn member_access_9_a() {
    let src = r#"
            [].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess9A", 104, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess10 (case 105)
#[test]
fn member_access_10() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []?.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess10", 105, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess10A (case 106)
#[test]
fn member_access_10_a() {
    let src = r#"
            []?.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess10A", 106, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess11 (case 107)
#[test]
fn member_access_11() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [][0].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess11", 107, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess11A (case 108)
#[test]
fn member_access_11_a() {
    let src = r#"
            [][0].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess11A", 108, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess12 (case 109)
#[test]
fn member_access_12() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []!.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess12", 109, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess12A (case 110)
#[test]
fn member_access_12_a() {
    let src = r#"
            []!.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess12A", 110, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess13 (case 111)
#[test]
fn member_access_13() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A]!.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess13", 111, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess13A (case 112)
#[test]
fn member_access_13_a() {
    let src = r#"
            [A]!.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess13A", 112, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess15 (case 113)
#[test]
fn member_access_15() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A()]!.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess15", 113, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess15A (case 114)
#[test]
fn member_access_15_a() {
    let src = r#"
            [A()]!.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess15A", 114, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess16 (case 115)
#[test]
fn member_access_16() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A()][0]!.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess16", 115, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess16A (case 116)
#[test]
fn member_access_16_a() {
    let src = r#"
            [A()][0]!.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess16A", 116, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess17 (case 117)
#[test]
fn member_access_17() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [][0]!.GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess17", 117, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess17A (case 118)
#[test]
fn member_access_17_a() {
    let src = r#"
            [][0]!.GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess17A", 118, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess19 (case 119)
#[test]
fn member_access_19() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [..A][..B].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess19", 119, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess19A (case 120)
#[test]
fn member_access_19_a() {
    let src = r#"
            [..A][..B].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess19A", 120, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess20 (case 121)
#[test]
fn member_access_20() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [[A]].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess20", 121, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess20A (case 122)
#[test]
fn member_access_20_a() {
    let src = r#"
            [[A]].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess20A", 122, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess21 (case 123)
#[test]
fn member_access_21() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    [A([B])].GetHashCode();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess21", 123, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess21A (case 124)
#[test]
fn member_access_21_a() {
    let src = r#"
            [A([B])].GetHashCode();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess21A", 124, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess23 (case 125)
#[test]
fn member_access_23() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []++;
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess23", 125, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess23A (case 126)
#[test]
fn member_access_23_a() {
    let src = r#"
            []++;
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess23A", 126, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess24 (case 127)
#[test]
fn member_access_24() {
    let src = r#"
            class Program
            {
                static void Main()
                {
                    []--;
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess24", 127, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.MemberAccess24A (case 128)
#[test]
fn member_access_24_a() {
    let src = r#"
            []--;
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "MemberAccess24A", 128, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead2A (case 129)
#[test]
fn lambda_attribute_versus_collection_lookahead_2_a() {
    let src = r#"[A][B](C, D) ? e : f"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? e : f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LambdaAttributeVersusCollectionLookahead2A", 129, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead3A (case 130)
#[test]
fn lambda_attribute_versus_collection_lookahead_3_a() {
    let src = r#"[A][B](C, D) ? (e) : f"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? (e) : f; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LambdaAttributeVersusCollectionLookahead3A", 130, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.LambdaAttributeVersusCollectionLookahead4A (case 131)
#[test]
fn lambda_attribute_versus_collection_lookahead_4_a() {
    let src = r#"[A][B](C, D) ? (e, f) : g"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { [A][B](C, D) ? (e, f) : g; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "LambdaAttributeVersusCollectionLookahead4A", 131, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity1 (case 132)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_1() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()]();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1", 132, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A (case 133)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_1_a() {
    let src = r#"
            [() => {}][rand.Next()]();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity1A", 133, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity2 (case 134)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_2() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A);
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2", 134, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A (case 135)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_2_a() {
    let src = r#"
            [() => {}][rand.Next()](A);
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity2A", 135, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity3 (case 136)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_3() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)[0];
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3", 136, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A (case 137)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_3_a() {
    let src = r#"
            [() => {}][rand.Next()](A)[0];
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity3A", 137, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity4 (case 138)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_4() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)(B);
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4", 138, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A (case 139)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_4_a() {
    let src = r#"
            [() => {}][rand.Next()](A)(B);
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity4A", 139, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity5 (case 140)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_5() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A).B();
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5", 140, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A (case 141)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_5_a() {
    let src = r#"
            [() => {}][rand.Next()](A).B();
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity5A", 141, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity6 (case 142)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_6() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)++;
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6", 142, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A (case 143)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_6_a() {
    let src = r#"
            [() => {}][rand.Next()](A)++;
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity6A", 143, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity7 (case 144)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_7() {
    let src = r#"
            class C
            {
                void M()
                {
                    [() => {}][rand.Next()](A)[0] = 1;
                }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7", 144, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A (case 145)
#[test]
fn invoked_collection_expression_versus_local_function_ambiguity_7_a() {
    let src = r#"
            [() => {}][rand.Next()](A)[0] = 1;
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "InvokedCollectionExpressionVersusLocalFunctionAmbiguity7A", 145, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.ByteArrayAmbiguityWithAttributes (case 146)
#[test]
fn byte_array_ambiguity_with_attributes() {
    let src = r#"class C { public ReadOnlySpan<byte> B => [0, 1, 2, 3, 4, 5, 6, 7]; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "ByteArrayAmbiguityWithAttributes", 146, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TreatKeywordAsAttributeTarget (case 147)
#[test]
fn treat_keyword_as_attribute_target() {
    let src = r#"class C { public ReadOnlySpan<byte> B => [true: A] () => { }; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TreatKeywordAsAttributeTarget", 147, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.TreatKeywordAsCollectionExprElement (case 148)
#[test]
fn treat_keyword_as_collection_expr_element() {
    let src = r#"class C { public bool[] B => [true]; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "TreatKeywordAsCollectionExprElement", 148, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity3 (case 149)
#[test]
fn collection_expression_conditional_expression_ambiguity_3() {
    let src = r#"var v = x is Y ? [];"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CollectionExpression_ConditionalExpressionAmbiguity3", 149, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity4 (case 150)
#[test]
fn collection_expression_conditional_expression_ambiguity_4() {
    let src = r#"var v = x is Y ? [,];"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CollectionExpression_ConditionalExpressionAmbiguity4", 150, CaseData::Statement { ast: &ast, src });
}

/// Roslyn: CollectionExpressionParsingTests.CollectionExpression_ConditionalExpressionAmbiguity5 (case 151)
#[test]
fn collection_expression_conditional_expression_ambiguity_5() {
    let src = r#"var v = x is Y ? [][];"#;
    let span = Span::new(src);
    let r = parse_statement_ws(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (rest, ast) = r.unwrap();
    assert!(rest.fragment().trim().is_empty(), "Unconsumed input: {}", rest.fragment());
    after_parse::after_parse("collection_expression_parsing_tests", "CollectionExpressionParsingTests", "CollectionExpression_ConditionalExpressionAmbiguity5", 151, CaseData::Statement { ast: &ast, src });
}

