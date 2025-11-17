// Auto-generated STRUCTURE tests from Roslyn: ParsingErrorRecoveryTests
#[test]
fn semicolon_after_object_initializer_member_2() {
    let src = r#"class c { void m() { var x = new C { a = b; }; var y = 5; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ObjectInitializerExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn semicolon_after_object_initializer_member_3() {
    let src =
        r#"class c { void m() { var x = new C { a = b; c = d, e = f; g = h }; var y = 5; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ObjectInitializerExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("d".to_string()), children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("e".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("f".to_string()), children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("g".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("h".to_string()), children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn semicolon_after_object_initializer_member_4() {
    let src = r#"class c { void m() { var x = new C { a = b; }; if (true) return; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ObjectInitializerExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "IfStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "TrueLiteralExpression".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn semicolon_after_object_initializer_member_5() {
    let src = r#"class c { void m() { var x = new C { a = b; if (true) return; }; var y = 5; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ObjectInitializerExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "IfStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "TrueLiteralExpression".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "FieldDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn semicolon_after_object_initializer_member_6() {
    let src = r#"class c { void m() { var x = new C { a = b; return; }; var y = 5; } }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("c".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ObjectInitializerExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "SimpleAssignmentExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![] }] }] },         structure_assert::ExpectedNode { kind: "FieldDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("var".to_string()), children: vec![] },                 structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn razor_comment_recovery_space() {
    let src = r#"
_ _::this
"#;
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
                        kind: "ExpressionStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "PointerIndirectionExpression".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "PointerIndirectionExpression".to_string(),
                                token_value: None,
                                children: vec![],
                            }],
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn razor_comment_recovery_no_start() {
    let src = r#"
_ _::this
"#;
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
                        kind: "ExpressionStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "PointerIndirectionExpression".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_01() {
    let src = r#"
_ _::this
"#;
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
                        kind: "IfStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_01_whitespace_before_hash() {
    let src = r#"
_ _::this
"#;
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
                        kind: "IfStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_01_whitespace_after_hash() {
    let src = r#"
_ _::this
"#;
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
                        kind: "IfStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_02() {
    let src = r#"
_ _::this
"#;
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
                        kind: "IfStatement".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ExpressionStatement".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("y".to_string()),
                                    children: vec![],
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
fn preprocessor_directive_trailing_03() {
    let src = r#"
_ _::this
"#;
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
                        kind: "ExpressionStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "InvocationExpression".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("a".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_04() {
    let src = r#"
_ _::this
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("a".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("b".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("c".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_05() {
    let src = r#"
_ _::this
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("a".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("b".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
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
                                        token_value: Some("d".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_define() {
    let src = r#"
_ _::this
"#;
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
                        kind: "ExpressionStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "InvocationExpression".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("y".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_undefine() {
    let src = r#"
_ _::this
"#;
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
                        kind: "ExpressionStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "InvocationExpression".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("x".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn preprocessor_directive_trailing_error_warning() {
    let src = r#"
_ _::this
"#;
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
fn preprocessor_directive_trailing_line() {
    let src = r#"
_ _::this
"#;
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
fn missing_node_with_skipped_tokens_1() {
    let src = r#"
_ _::this
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "LocalDeclarationStatement".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "VariableDeclaration".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("i".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "InterfaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}
