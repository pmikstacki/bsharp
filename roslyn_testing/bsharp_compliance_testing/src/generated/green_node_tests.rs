// Auto-generated from Roslyn: GreenNodeTests
use crate::custom_asserts::after_parse;
use crate::custom_asserts::after_parse::CaseData;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
/// Roslyn: GreenNodeTests.ConvenienceSwitchStatementFactoriesAddParensWhenNeeded_01 (case 1)
#[test]
fn convenience_switch_statement_factories_add_parens_when_needed_01() {
    let src = r#"x"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { x; } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "green_node_tests",
                "GreenNodeTests",
                "ConvenienceSwitchStatementFactoriesAddParensWhenNeeded_01",
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

/// Roslyn: GreenNodeTests.ConvenienceSwitchStatementFactoriesAddParensWhenNeeded_02 (case 2)
#[test]
fn convenience_switch_statement_factories_add_parens_when_needed_02() {
    let src = r#"(x)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (x); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "green_node_tests",
                "GreenNodeTests",
                "ConvenienceSwitchStatementFactoriesAddParensWhenNeeded_02",
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

/// Roslyn: GreenNodeTests.ConvenienceSwitchStatementFactoriesOmitParensWhenPossible (case 3)
#[test]
fn convenience_switch_statement_factories_omit_parens_when_possible() {
    let src = r#"(1, 2)"#;
    let span = Span::new(src);
    let src2 = r#"class C { void M() { (1, 2); } }"#;
    let span2 = Span::new(src2);
    let r = parse_csharp_source_strict(span2);
    match r {
        Ok((_rest, unit)) => {
            after_parse::after_parse_with_expected(
                "green_node_tests",
                "GreenNodeTests",
                "ConvenienceSwitchStatementFactoriesOmitParensWhenPossible",
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
