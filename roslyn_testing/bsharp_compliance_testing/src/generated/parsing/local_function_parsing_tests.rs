// Auto-generated STRUCTURE tests from Roslyn: LocalFunctionParsingTests
#[test]
fn incomplete_local_func() {
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
fn local_function_attribute() {
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
fn local_function_modifier_error_local_variable() {
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
fn local_function_no_body() {
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
fn local_function_extern() {
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
fn local_function_extern_case_2() {
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
fn local_function_extern_body() {
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
fn local_function_extern_body_case_2() {
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
fn local_function_attribute_error_local_variable() {
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
fn local_function_attribute_error_local_variable_multiple_declarators() {
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
fn local_function_attribute_error_incomplete_member() {
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
fn local_functions_with_await() {
    let src = r#"
class c
{
    void m()
    {
        int local() => 0;
    }
    void m2()
    {
        int local() { return 0; }
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("c".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "MethodDeclaration".to_string(),
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
                                            kind: "LocalFunctionStatement".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("await".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "ParameterList".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "ArrowExpressionClause".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "ObjectCreationExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "IdentifierName"
                                                                        .to_string(),
                                                                    token_value: Some(
                                                                        "await".to_string(),
                                                                    ),
                                                                    children: vec![],
                                                                },
                                                                structure_assert::ExpectedNode {
                                                                    kind: "ArgumentList"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                },
                                            ],
                                        }],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "MethodDeclaration".to_string(),
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
                                                kind: "ParenthesizedLambdaExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("await".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "ParameterList".to_string(),
                                                        token_value: None,
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "ObjectCreationExpression"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some(
                                                                    "await".to_string(),
                                                                ),
                                                                children: vec![],
                                                            },
                                                            structure_assert::ExpectedNode {
                                                                kind: "ArgumentList".to_string(),
                                                                token_value: None,
                                                                children: vec![],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            }],
                                        }],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "MethodDeclaration".to_string(),
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
                                                kind: "AwaitExpression".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "ParenthesizedExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            }],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ExpressionStatement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "ObjectCreationExpression".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("await".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "ArgumentList".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                            ],
                                        }],
                                    },
                                ],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "MethodDeclaration".to_string(),
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
                                    kind: "LocalFunctionStatement".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("async".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ParameterList".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ArrowExpressionClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "ObjectCreationExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("await".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "ArgumentList".to_string(),
                                                        token_value: None,
                                                        children: vec![],
                                                    },
                                                ],
                                            }],
                                        },
                                    ],
                                }],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "MethodDeclaration".to_string(),
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
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "ExpressionStatement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "AwaitExpression".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "InvocationExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("async".to_string()),
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
                                        kind: "ExpressionStatement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "ObjectCreationExpression".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("await".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "ArgumentList".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                            ],
                                        }],
                                    },
                                ],
                            },
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn my_attribute() {
    let src = r#"
class c
{
    void m()
    {
        int local() => 0;
    }
    void m2()
    {
        int local() { return 0; }
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                             structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                     structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] }] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "CollectionExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "ExpressionElement".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("My".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                                        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("nameof".to_string()), children: vec![] },                                                         structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                                            structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("parameter".to_string()), children: vec![] }] }] }] }] }] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "AnonymousMethodExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "ReturnStatement".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },     structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("MyAttribute".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "BaseList".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "SimpleBaseType".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Attribute".to_string()), children: vec![] }] }] }] },         structure_assert::ExpectedNode { kind: "ConstructorDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn sum() {
    let src = r#"""
                public class C
                {
                    public void M()
                    {
                        int sum0 = Sum(1, 2));

                        void Local()
                        {
                            AnotherLocal());

                            int sum1 = Sum(1, 2));
                            int sum2 = Sum(1, 3));

                            void AnotherLocal()
                            {
                                int x = sum2 + 2;
                            }
                        }
                    }

                    public static int Sum(int a, int b) => a + b;
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Sum".to_string()), children: vec![] },                                     structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "LocalFunctionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("AnotherLocal".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] },                         structure_assert::ExpectedNode { kind: "EmptyStatement".to_string(), token_value: None, children: vec![] },                         structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                 structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Sum".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] },                                                 structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },                         structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                 structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Sum".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] },                                                 structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] },                         structure_assert::ExpectedNode { kind: "LocalFunctionStatement".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },                             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                                structure_assert::ExpectedNode { kind: "AddExpression".to_string(), token_value: None, children: vec![                                                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("sum2".to_string()), children: vec![] },                                                     structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] }] }] }] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] },                 structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "AddExpression".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("a".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("b".to_string()), children: vec![] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}
