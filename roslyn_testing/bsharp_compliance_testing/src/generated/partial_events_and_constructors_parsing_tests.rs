// Auto-generated from Roslyn: PartialEventsAndConstructorsParsingTests
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
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Tree",
                    1,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Tree",
                    1,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Tree",
            1,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition (case 2)
#[test]
fn event_definition() {
    let src = r#"
            partial event Action E;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition",
                    2,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition",
                    2,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition",
            2,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple (case 3)
#[test]
fn event_definition_multiple() {
    let src = r#"
            partial event Action E, F;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E, F;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple",
                    3,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple",
                    3,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_Multiple",
            3,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Initializer (case 4)
#[test]
fn event_definition_initializer() {
    let src = r#"
            partial event Action E = null;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Initializer",
                    4,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Initializer",
                    4,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_Initializer",
            4,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple_Initializer (case 5)
#[test]
fn event_definition_multiple_initializer() {
    let src = r#"
            partial event Action E, F = null;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E, F = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple_Initializer",
                    5,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple_Initializer",
                    5,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_Multiple_Initializer",
            5,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_Multiple_Initializers (case 6)
#[test]
fn event_definition_multiple_initializers() {
    let src = r#"
            partial event Action E = null, F = null;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E = null, F = null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple_Initializers",
                    6,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_Multiple_Initializers",
                    6,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_Multiple_Initializers",
            6,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterEvent (case 7)
#[test]
fn event_definition_partial_after_event() {
    let src = r#"
            event partial Action E;
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            event partial Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterEvent",
                    7,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterEvent",
                    7,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_PartialAfterEvent",
            7,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterType (case 8)
#[test]
fn event_definition_partial_after_type() {
    let src = r#"
            event Action partial E;
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            event Action partial E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterType",
                    8,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterType",
                    8,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_PartialAfterType",
            8,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialAfterPublic (case 9)
#[test]
fn event_definition_partial_after_public() {
    let src = r#"
            public partial event Action E;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            public partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterPublic",
                    9,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialAfterPublic",
                    9,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_PartialAfterPublic",
            9,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_PartialBeforePublic (case 10)
#[test]
fn event_definition_partial_before_public() {
    let src = r#"
            partial public event Action E;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial public event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialBeforePublic",
                    10,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_PartialBeforePublic",
                    10,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_PartialBeforePublic",
            10,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_DoublePartial (case 11)
#[test]
fn event_definition_double_partial() {
    let src = r#"
            partial partial event Action E;
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial partial event Action E;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_DoublePartial",
                    11,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_DoublePartial",
                    11,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_DoublePartial",
            11,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Definition_MissingRest (case 12)
#[test]
fn event_definition_missing_rest() {
    let src = r#"
            partial event
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 3,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_MissingRest",
                    12,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Definition_MissingRest",
                    12,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Definition_MissingRest",
            12,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation (case 13)
#[test]
fn event_implementation() {
    let src = r#"
            partial event Action E { add { } remove { } }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E { add { } remove { } }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation",
                    13,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation",
                    13,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Implementation",
            13,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation_Multiple (case 14)
#[test]
fn event_implementation_multiple() {
    let src = r#"
            partial event Action E, F { add { } remove { } }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E, F { add { } remove { } }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_Multiple",
                    14,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_Multiple",
                    14,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Implementation_Multiple",
            14,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation_PartialAfterEvent (case 15)
#[test]
fn event_implementation_partial_after_event() {
    let src = r#"
            event partial Action E { add { } remove { } }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 4,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            event partial Action E { add { } remove { } }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_PartialAfterEvent",
                    15,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_PartialAfterEvent",
                    15,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Implementation_PartialAfterEvent",
            15,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation_SemicolonAccessors (case 16)
#[test]
fn event_implementation_semicolon_accessors() {
    let src = r#"
            partial event Action E { add; remove; }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E { add; remove; }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_SemicolonAccessors",
                    16,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_SemicolonAccessors",
                    16,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Implementation_SemicolonAccessors",
            16,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_Implementation_PartialAccessors (case 17)
#[test]
fn event_implementation_partial_accessors() {
    let src = r#"
            partial event Action E { partial add; partial remove; }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial event Action E { partial add; partial remove; }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_PartialAccessors",
                    17,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_Implementation_PartialAccessors",
                    17,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_Implementation_PartialAccessors",
            17,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Event_InPlaceOfIdentifier (case 18)
#[test]
fn event_in_place_of_identifier() {
    let src = r#"
            partial class C
            {
                [Attr(
                partial event Action E;
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_InPlaceOfIdentifier",
                    18,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Event_InPlaceOfIdentifier",
                    18,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Event_InPlaceOfIdentifier",
            18,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Tree (case 19)
#[test]
fn constructor_tree() {
    let src = r#"
            partial class C
            {
                partial C();
                partial C() { }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Tree",
                    19,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Tree",
                    19,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_Tree",
            19,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Declaration (case 20)
#[test]
fn constructor_declaration() {
    let src = r#"
            partial C() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() { }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Declaration",
                    20,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Declaration",
                    20,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_Declaration",
            20,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_Declaration_CSharp13 (case 21)
#[test]
fn constructor_declaration_csharp_13() {
    let src = r#"
            partial C() { }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() { }
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Declaration_CSharp13",
                    21,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_Declaration_CSharp13",
                    21,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_Declaration_CSharp13",
            21,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_ArrowBody (case 22)
#[test]
fn constructor_arrow_body() {
    let src = r#"
            partial C() => throw null;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C() => throw null;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_ArrowBody",
                    22,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_ArrowBody",
                    22,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_ArrowBody",
            22,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_NoParens (case 23)
#[test]
fn constructor_no_parens() {
    let src = r#"
            partial C;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C;
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_NoParens",
                    23,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_NoParens",
                    23,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_NoParens",
            23,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_NoName (case 24)
#[test]
fn constructor_no_name() {
    let src = r#"
            partial ();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial ();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_NoName",
                    24,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_NoName",
                    24,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_NoName",
            24,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAsName (case 25)
#[test]
fn constructor_partial_as_name() {
    let src = r#"
            partial partial();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial partial();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAsName",
                    25,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAsName",
                    25,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_PartialAsName",
            25,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAfterName (case 26)
#[test]
fn constructor_partial_after_name() {
    let src = r#"
            C partial();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            C partial();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAfterName",
                    26,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAfterName",
                    26,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_PartialAfterName",
            26,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialAfterPublic (case 27)
#[test]
fn constructor_partial_after_public() {
    let src = r#"
            public partial C();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            public partial C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAfterPublic",
                    27,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialAfterPublic",
                    27,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_PartialAfterPublic",
            27,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialBeforePublic (case 28)
#[test]
fn constructor_partial_before_public() {
    let src = r#"
            partial public C();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial public C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialBeforePublic",
                    28,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialBeforePublic",
                    28,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_PartialBeforePublic",
            28,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_TypeTwice (case 29)
#[test]
fn constructor_type_twice() {
    let src = r#"
            partial C C();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial C C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_TypeTwice",
                    29,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_TypeTwice",
                    29,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_TypeTwice",
            29,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_PartialEscaped (case 30)
#[test]
fn constructor_partial_escaped() {
    let src = r#"
            @partial C();
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let src2 = r#"class C { 
            @partial C();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialEscaped",
                    30,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_PartialEscaped",
                    30,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_PartialEscaped",
            30,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_KeywordName (case 31)
#[test]
fn constructor_keyword_name() {
    let src = r#"
            partial const();
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let src2 = r#"class C { 
            partial const();
             }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_KeywordName",
                    31,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src: src2,
                        original: Some(src),
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_KeywordName",
                    31,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_KeywordName",
            31,
            None,
            CaseData::File {
                unit: &unit,
                src: src2,
                original: Some(src),
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.Constructor_InPlaceOfIdentifier (case 32)
#[test]
fn constructor_in_place_of_identifier() {
    let src = r#"
            partial class C
            {
                [Attr(
                partial C();
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_InPlaceOfIdentifier",
                    32,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "Constructor_InPlaceOfIdentifier",
                    32,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "Constructor_InPlaceOfIdentifier",
            32,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_LocalFunction_InMethod (case 33)
#[test]
fn returning_partial_type_local_function_in_method() {
    let src = r#"
            class C
            {
                void M()
                {
                    partial F() => null;
                }
            }
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 2,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_InMethod",
                    33,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_InMethod",
                    33,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_LocalFunction_InMethod",
            33,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_LocalFunction_InMethod_CSharp13 (case 34)
#[test]
fn returning_partial_type_local_function_in_method_csharp_13() {
    let src = r#"
            class C
            {
                void M()
                {
                    partial F() => null;
                }
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_InMethod_CSharp13",
                    34,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_InMethod_CSharp13",
                    34,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_LocalFunction_InMethod_CSharp13",
            34,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_LocalFunction_TopLevel (case 35)
#[test]
fn returning_partial_type_local_function_top_level() {
    let src = r#"
            partial F() => null;
            "#;
    let expected = Some(ExpectedDiagnostics {
        count: 1,
        items: vec![],
    });
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_TopLevel",
                    35,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_TopLevel",
                    35,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_LocalFunction_TopLevel",
            35,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_LocalFunction_TopLevel_CSharp13 (case 36)
#[test]
fn returning_partial_type_local_function_top_level_csharp_13() {
    let src = r#"
            partial F() => null;
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_TopLevel_CSharp13",
                    36,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_LocalFunction_TopLevel_CSharp13",
                    36,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_LocalFunction_TopLevel_CSharp13",
            36,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_Method (case 37)
#[test]
fn returning_partial_type_method() {
    let src = r#"
            class C
            {
                partial M() => null;
                @partial M() => null;
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_Method",
                    37,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_Method",
                    37,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_Method",
            37,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}

/// Roslyn: PartialEventsAndConstructorsParsingTests.ReturningPartialType_Method_CSharp13 (case 38)
#[test]
fn returning_partial_type_method_csharp_13() {
    let src = r#"
            class C
            {
                partial M() => null;
                @partial M() => null;
            }
            "#;
    let expected: Option<ExpectedDiagnostics> = None;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Some(expected) = expected {
        match r {
            Ok((_rest, unit)) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_Method_CSharp13",
                    38,
                    Some(expected.clone()),
                    CaseData::File {
                        unit: &unit,
                        src,
                        original: None,
                    },
                );
            }
            Err(_) => {
                after_parse::after_parse_with_expected(
                    "partial_events_and_constructors_parsing_tests",
                    "PartialEventsAndConstructorsParsingTests",
                    "ReturningPartialType_Method_CSharp13",
                    38,
                    Some(expected.clone()),
                    CaseData::Empty,
                );
            }
        }
    } else {
        assert!(r.is_ok(), "parse failed: {:?}", r.err());
        let (_rest, unit) = r.unwrap();
        after_parse::after_parse_with_expected(
            "partial_events_and_constructors_parsing_tests",
            "PartialEventsAndConstructorsParsingTests",
            "ReturningPartialType_Method_CSharp13",
            38,
            None,
            CaseData::File {
                unit: &unit,
                src,
                original: None,
            },
        );
    }
}
