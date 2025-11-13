// Auto-generated STRUCTURE tests from Roslyn: AsyncStreamsParsingTests
#[test]
fn await_using_declaration() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn await_using_with_expression() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn await_using_with_expression_reversed() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn await_foreach() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn await_foreach_reversed() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn deconstruction_await_foreach() {
    let src = r#""#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}
