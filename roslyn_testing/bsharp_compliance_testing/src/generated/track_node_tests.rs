// Auto-generated from Roslyn: TrackNodeTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterTrackNodesReturnsCurrentNode (case 1)
#[test]
fn get_current_node_after_track_nodes_returns_current_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterTrackNodesReturnsCurrentNode",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesAfterTrackNodesReturnsSingletonSequence (case 2)
#[test]
fn get_current_nodes_after_track_nodes_returns_singleton_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodesAfterTrackNodesReturnsSingletonSequence",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeWithoutTrackNodesReturnsNull (case 3)
#[test]
fn get_current_node_without_track_nodes_returns_null() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeWithoutTrackNodesReturnsNull",
                3,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesWithoutTrackNodesReturnsEmptySequence (case 4)
#[test]
fn get_current_nodes_without_track_nodes_returns_empty_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodesWithoutTrackNodesReturnsEmptySequence",
                4,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterEditReturnsCurrentNode (case 5)
#[test]
fn get_current_node_after_edit_returns_current_node() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterEditReturnsCurrentNode",
                5,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterEditReturnsSingletonSequence (case 6)
#[test]
fn get_current_node_after_edit_returns_singleton_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterEditReturnsSingletonSequence",
                6,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterRemovalReturnsNull (case 7)
#[test]
fn get_current_node_after_removal_returns_null() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterRemovalReturnsNull",
                7,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodesAfterRemovalEmptySequence (case 8)
#[test]
fn get_current_nodes_after_removal_empty_sequence() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodesAfterRemovalEmptySequence",
                8,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterAddingMultipleThrows (case 9)
#[test]
fn get_current_node_after_adding_multiple_throws() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterAddingMultipleThrows",
                9,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestGetCurrentNodeAfterAddingMultipleReturnsMultiple (case 10)
#[test]
fn get_current_node_after_adding_multiple_returns_multiple() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestGetCurrentNodeAfterAddingMultipleReturnsMultiple",
                10,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestTrackNodesWithMultipleTracksAllNodes (case 11)
#[test]
fn track_nodes_with_multiple_tracks_all_nodes() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestTrackNodesWithMultipleTracksAllNodes",
                11,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestTrackNodesWithNoNodesTracksNothing (case 12)
#[test]
fn track_nodes_with_no_nodes_tracks_nothing() {
    let src = r#"a + b + c"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b + c; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestTrackNodesWithNoNodesTracksNothing",
                12,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: TrackNodeTests.TestTrackNodeThatIsNotInTheSubtreeThrows (case 13)
#[test]
fn track_node_that_is_not_in_the_subtree_throws() {
    let src = r#"a + b"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { a + b; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "track_node_tests",
                "TrackNodeTests",
                "TestTrackNodeThatIsNotInTheSubtreeThrows",
                13,
                None,
                CaseData::File {
                    unit: &unit,
                    src: src2,
                    original: Some(src),
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
