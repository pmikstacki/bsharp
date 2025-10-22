// Auto-generated from Roslyn: SyntaxTreeTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
/// Roslyn: SyntaxTreeTests.Create (case 1)
#[test]
fn create() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_tree_tests",
                "SyntaxTreeTests",
                "Create",
                1,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxTreeTests.Create_WithDiagnosticOptions (case 2)
#[test]
fn create_with_diagnostic_options() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_tree_tests",
                "SyntaxTreeTests",
                "Create_WithDiagnosticOptions",
                2,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_ParsedTree (case 3)
#[test]
fn with_root_and_options_parsed_tree() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_tree_tests",
                "SyntaxTreeTests",
                "WithRootAndOptions_ParsedTree",
                3,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_ParsedTreeWithText (case 4)
#[test]
fn with_root_and_options_parsed_tree_with_text() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_tree_tests",
                "SyntaxTreeTests",
                "WithRootAndOptions_ParsedTreeWithText",
                4,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}

/// Roslyn: SyntaxTreeTests.WithRootAndOptions_DummyTree (case 5)
#[test]
fn with_root_and_options_dummy_tree() {
    let src = r#"class C {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "syntax_tree_tests",
                "SyntaxTreeTests",
                "WithRootAndOptions_DummyTree",
                5,
                None,
                CaseData::File {
                    unit: &unit,
                    src,
                    original: None,
                },
            );
        }
        Err(e) => panic!("parse failed: {:?}", e),
    }
}
