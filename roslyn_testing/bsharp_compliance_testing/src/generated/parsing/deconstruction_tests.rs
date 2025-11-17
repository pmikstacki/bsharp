// Auto-generated STRUCTURE tests from Roslyn: DeconstructionTests
#[test]
fn paren_expression() {
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
fn tuple_type_with_element_names() {
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
fn tuple_type() {
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
fn tuple_type_array() {
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
fn tuple_literal() {
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
fn deconstruction_assignment() {
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
fn simple_declaration() {
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
fn nested_deconstruction_assignment() {
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
fn deconstruction_declaration() {
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
fn nested_deconstruction_declaration() {
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
fn var_deconstruction_declaration() {
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
fn var_nested_deconstruction_declaration() {
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
fn var_method_call() {
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
fn mixed_deconstruction_variables() {
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
fn deconstruction_for() {
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
fn var_deconstruction_for() {
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
fn deconstruction_foreach() {
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
fn var_deconstruction_foreach() {
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
fn deconstruction_in_script() {
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
fn deconstruction_for_each_in_script() {
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
fn deconstruction_declaration_with_discard() {
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
fn discards_in_deconstruction_01() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "ExpressionStatement".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "SimpleAssignmentExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "DeclarationExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("var".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "ParenthesizedVariableDesignation"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "SingleVariableDesignation"
                                                                    .to_string(),
                                                                token_value: None,
                                                                children: vec![],
                                                            },
                                                            structure_assert::ExpectedNode {
                                                                kind: "DiscardDesignation"
                                                                    .to_string(),
                                                                token_value: None,
                                                                children: vec![],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("e".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_deconstruction_02() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "GlobalStatement".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "LocalFunctionStatement".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "DiscardDesignation".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_out_01() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "ExpressionStatement".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "InvocationExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("M".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArgumentList".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "Argument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "DeclarationExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "IdentifierName"
                                                                        .to_string(),
                                                                    token_value: Some(
                                                                        "var".to_string(),
                                                                    ),
                                                                    children: vec![],
                                                                },
                                                                structure_assert::ExpectedNode {
                                                                    kind: "DiscardDesignation"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                }],
                                            },
                                        ],
                                    }],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_out_02() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "ExpressionStatement".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "InvocationExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("M".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArgumentList".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "Argument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "DeclarationExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "PredefinedType"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                                structure_assert::ExpectedNode {
                                                                    kind: "DiscardDesignation"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                }],
                                            },
                                        ],
                                    }],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_pattern_01() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IfStatement".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IsPatternExpression".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("e".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "DeclarationPattern".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "PredefinedType".to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                        structure_assert::ExpectedNode {
                                                            kind: "DiscardDesignation".to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Block".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                    ],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_pattern_02() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IfStatement".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IsPatternExpression".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("e".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "VarPattern".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "DiscardDesignation".to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Block".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                    ],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_pattern_03() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "SwitchStatement".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("e".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "SwitchSection".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "CasePatternSwitchLabel".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "DeclarationPattern".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "PredefinedType"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                                structure_assert::ExpectedNode {
                                                                    kind: "DiscardDesignation"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "BreakStatement".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                            ],
                                        },
                                    ],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn discards_in_pattern_04() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "GlobalStatement".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "LocalFunctionStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "Block".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "SwitchStatement".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("e".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "SwitchSection".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "CasePatternSwitchLabel".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "VarPattern".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "DiscardDesignation"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "BreakStatement".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                            ],
                                        },
                                    ],
                                }],
                            },
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn pointer_type_in_deconstruction() {
    let src = r#"(x, y)? z = M();"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("x1".to_string()), children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "ArrayType".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PointerType".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "ArrayRankSpecifier".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "OmittedArraySizeExpression".to_string(), token_value: None, children: vec![] }] }] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "ArrayType".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PointerType".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "ArrayRankSpecifier".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "OmittedArraySizeExpression".to_string(), token_value: None, children: vec![] }] }] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("x4".to_string()), children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "DeclarationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "SingleVariableDesignation".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("x5".to_string()), children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("y5".to_string()), children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "TupleExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("x6".to_string()), children: vec![] }] }] },                             structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "MultiplyExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("y6".to_string()), children: vec![] }] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}
