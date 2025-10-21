// Auto-generated from Roslyn: PartialEventsAndConstructorsParsingTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Tree (case 1)
#[test]
fn event_tree() {
    let src = r#"
            partial class C
            {
                partial event Action E;
                partial event Action E { add { } remove { } }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Tree", 1, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition (case 2)
#[test]
fn event_definition() {
    let src = r#"
            partial event Action E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple (case 3)
#[test]
fn event_definition_multiple() {
    let src = r#"
            partial event Action E, F;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E, F;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_Multiple", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Initializer (case 4)
#[test]
fn event_definition_initializer() {
    let src = r#"
            partial event Action E = null;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_Initializer", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple_Initializer (case 5)
#[test]
fn event_definition_multiple_initializer() {
    let src = r#"
            partial event Action E, F = null;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E, F = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_Multiple_Initializer", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple_Initializers (case 6)
#[test]
fn event_definition_multiple_initializers() {
    let src = r#"
            partial event Action E = null, F = null;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E = null, F = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_Multiple_Initializers", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterEvent (case 7)
#[test]
fn event_definition_partial_after_event() {
    let src = r#"
            event partial Action E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            event partial Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_PartialAfterEvent", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterType (case 8)
#[test]
fn event_definition_partial_after_type() {
    let src = r#"
            event Action partial E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            event Action partial E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_PartialAfterType", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterPublic (case 9)
#[test]
fn event_definition_partial_after_public() {
    let src = r#"
            public partial event Action E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            public partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_PartialAfterPublic", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialBeforePublic (case 10)
#[test]
fn event_definition_partial_before_public() {
    let src = r#"
            partial public event Action E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial public event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_PartialBeforePublic", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_DoublePartial (case 11)
#[test]
fn event_definition_double_partial() {
    let src = r#"
            partial partial event Action E;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_DoublePartial", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_MissingRest (case 12)
#[test]
fn event_definition_missing_rest() {
    let src = r#"
            partial event
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Definition_MissingRest", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation (case 13)
#[test]
fn event_implementation() {
    let src = r#"
            partial event Action E { add { } remove { } }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E { add { } remove { } }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Implementation", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation_SemicolonAccessors (case 14)
#[test]
fn event_implementation_semicolon_accessors() {
    let src = r#"
            partial event Action E { add; remove; }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E { add; remove; }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_Implementation_SemicolonAccessors", 14, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_InPlaceOfIdentifier (case 15)
#[test]
fn event_in_place_of_identifier() {
    let src = r#"
            partial class C
            {
                [Attr(
                partial event Action E;
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Event_InPlaceOfIdentifier", 15, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Tree (case 16)
#[test]
fn constructor_tree() {
    let src = r#"
            partial class C
            {
                partial C();
                partial C() { }
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_Tree", 16, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Declaration (case 17)
#[test]
fn constructor_declaration() {
    let src = r#"
            partial C() { }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() { }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_Declaration", 17, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Declaration_CSharp13 (case 18)
#[test]
fn constructor_declaration_csharp_13() {
    let src = r#"
            partial C() { }
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() { }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_Declaration_CSharp13", 18, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_ArrowBody (case 19)
#[test]
fn constructor_arrow_body() {
    let src = r#"
            partial C() => throw null;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() => throw null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_ArrowBody", 19, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_NoParens (case 20)
#[test]
fn constructor_no_parens() {
    let src = r#"
            partial C;
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_NoParens", 20, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_NoName (case 21)
#[test]
fn constructor_no_name() {
    let src = r#"
            partial ();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial ();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_NoName", 21, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAsName (case 22)
#[test]
fn constructor_partial_as_name() {
    let src = r#"
            partial partial();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial partial();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_PartialAsName", 22, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAfterName (case 23)
#[test]
fn constructor_partial_after_name() {
    let src = r#"
            C partial();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            C partial();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_PartialAfterName", 23, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAfterPublic (case 24)
#[test]
fn constructor_partial_after_public() {
    let src = r#"
            public partial C();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            public partial C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_PartialAfterPublic", 24, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialBeforePublic (case 25)
#[test]
fn constructor_partial_before_public() {
    let src = r#"
            partial public C();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial public C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_PartialBeforePublic", 25, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_TypeTwice (case 26)
#[test]
fn constructor_type_twice() {
    let src = r#"
            partial C C();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_TypeTwice", 26, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialEscaped (case 27)
#[test]
fn constructor_partial_escaped() {
    let src = r#"
            @partial C();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            @partial C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_PartialEscaped", 27, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_KeywordName (case 28)
#[test]
fn constructor_keyword_name() {
    let src = r#"
            partial const();
            "#;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial const();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_KeywordName", 28, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_InPlaceOfIdentifier (case 29)
#[test]
fn constructor_in_place_of_identifier() {
    let src = r#"
            partial class C
            {
                [Attr(
                partial C();
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "Constructor_InPlaceOfIdentifier", 29, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_Method (case 30)
#[test]
fn returning_partial_type_method() {
    let src = r#"
            class C
            {
                partial M() => null;
                @partial M() => null;
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "ReturningPartialType_Method", 30, CaseData::File { unit: &unit, src, original: None });
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_Method_CSharp13 (case 31)
#[test]
fn returning_partial_type_method_csharp_13() {
    let src = r#"
            class C
            {
                partial M() => null;
                @partial M() => null;
            }
            "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("partial_events_and_constructors_parsing_tests", "PartialEventsAndConstructorsParsingTests", "ReturningPartialType_Method_CSharp13", 31, CaseData::File { unit: &unit, src, original: None });
}

