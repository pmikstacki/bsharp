// Auto-generated from Roslyn: TrackNodeTests
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::statement_parser::parse_statement_ws;
use bsharp_parser::syntax::span::Span;
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterTrackNodesReturnsCurrentNode (case 1)
#[test]
fn get_current_node_after_track_nodes_returns_current_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterTrackNodesReturnsCurrentNode", 1, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesAfterTrackNodesReturnsSingletonSequence (case 2)
#[test]
fn get_current_nodes_after_track_nodes_returns_singleton_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodesAfterTrackNodesReturnsSingletonSequence", 2, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeWithoutTrackNodesReturnsNull (case 3)
#[test]
fn get_current_node_without_track_nodes_returns_null() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeWithoutTrackNodesReturnsNull", 3, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesWithoutTrackNodesReturnsEmptySequence (case 4)
#[test]
fn get_current_nodes_without_track_nodes_returns_empty_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodesWithoutTrackNodesReturnsEmptySequence", 4, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterEditReturnsCurrentNode (case 5)
#[test]
fn get_current_node_after_edit_returns_current_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterEditReturnsCurrentNode", 5, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterEditReturnsSingletonSequence (case 6)
#[test]
fn get_current_node_after_edit_returns_singleton_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterEditReturnsSingletonSequence", 6, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterRemovalReturnsNull (case 7)
#[test]
fn get_current_node_after_removal_returns_null() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterRemovalReturnsNull", 7, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesAfterRemovalEmptySequence (case 8)
#[test]
fn get_current_nodes_after_removal_empty_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodesAfterRemovalEmptySequence", 8, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterAddingMultipleThrows (case 9)
#[test]
fn get_current_node_after_adding_multiple_throws() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterAddingMultipleThrows", 9, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterAddingMultipleReturnsMultiple (case 10)
#[test]
fn get_current_node_after_adding_multiple_returns_multiple() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestGetCurrentNodeAfterAddingMultipleReturnsMultiple", 10, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestTrackNodesWithMultipleTracksAllNodes (case 11)
#[test]
fn track_nodes_with_multiple_tracks_all_nodes() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestTrackNodesWithMultipleTracksAllNodes", 11, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestTrackNodesWithNoNodesTracksNothing (case 12)
#[test]
fn track_nodes_with_no_nodes_tracks_nothing() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestTrackNodesWithNoNodesTracksNothing", 12, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

/// Roslyn: TrackNodeTests.TestTrackNodeThatIsNotInTheSubtreeThrows (case 13)
#[test]
fn track_node_that_is_not_in_the_subtree_throws() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    assert!(r.is_ok(), "parse failed: {:?}", r.err());
    let (_rest, unit) = r.unwrap();
    after_parse::after_parse("track_node_tests", "TrackNodeTests", "TestTrackNodeThatIsNotInTheSubtreeThrows", 13, CaseData::File { unit: &unit, src: src2, original: Some(src) });
}

