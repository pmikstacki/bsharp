// Auto-generated STRUCTURE tests from Roslyn: MemberDeclarationParsingTests
use crate::custom_asserts::structure_assert;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
#[test]
fn required_modifier_local_named_required_top_level_statements() {
    let src = r#"static implicit required operator C(S s) {}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "FieldDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "LocalDeclarationStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "VariableDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "VariableDeclarator".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "ExpressionStatement".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "SimpleAssignmentExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("required".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "TrueLiteralExpression".to_string(),
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
fn operator_declaration_explicit_implementation_11() {
    let src = r#"public int N.I.operator +(int x, int y) => x + y;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AddExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("x".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("y".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_12() {
    let src = r#"public int N.I.implicit (int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_13() {
    let src = r#"public int N.I.explicit (int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_14() {
    let src = r#"public int N.I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_15() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_16() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AliasQualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AddExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("x".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("y".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_17() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_18() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_19() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_20() {
    let src = r#"public int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "AliasQualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N1".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N2".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_33() {
    let src = r#"int N.I.operator +(int x, int y) => x + y;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AddExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("x".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("y".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_34() {
    let src = r#"int N.I.implicit (int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_35() {
    let src = r#"int N.I.explicit (int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_36() {
    let src = r#"int N.I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_37() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_38() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AliasQualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AddExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("x".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("y".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_39() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_40() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_41() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn operator_declaration_explicit_implementation_42() {
    let src = r#"int I operator +(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "OperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "AliasQualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N1".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N2".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_11() {
    let src = r#"explicit N.I.operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_12() {
    let src = r#"implicit N.I int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_13() {
    let src = r#"explicit N.I. int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_14() {
    let src = r#"implicit N.I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_15() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_16() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "AliasQualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("N".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_17() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_18() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("I".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_19() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_20() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ConversionOperatorDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "ExplicitInterfaceSpecifier".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "AliasQualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N1".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("N2".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("I".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Parameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ArrowExpressionClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn conversion_declaration_explicit_implementation_35() {
    let src = r#"explicit I operator int(int x) => x;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "ConversionOperatorDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "LocalDeclarationStatement".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "VariableDeclaration".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "GenericName".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "TypeArgumentList".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                },
                                            ],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "VariableDeclarator".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "EqualsValueClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "ParenthesizedLambdaExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "ParameterList".to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "Parameter".to_string(),
                                                                token_value: None,
                                                                children: vec![],
                                                            },
                                                        ],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "NumericLiteralExpression"
                                                            .to_string(),
                                                        token_value: None,
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
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn readonly_parameter_1() {
    let src = r#"explicit I operator int(int x) => x;"#;
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
                                token_value: Some("Base".to_string()),
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "PredefinedType".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "Block".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("Derived".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "BaseList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Base".to_string()),
                                        children: vec![],
                                    }],
                                }],
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "PredefinedType".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "Block".to_string(),
                                        token_value: None,
                                        children: vec![],
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
fn property_with_errant_semicolon_1() {
    let src = r#"
public class Class
{
    public int MyProperty; { get; set; }

    // Pretty much anything here causes an error
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ClassDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("Class".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PropertyDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "AccessorList".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "GetAccessorDeclaration".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "SetAccessorDeclaration".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn property_with_errant_semicolon_2() {
    let src = r#"
public class Class
{
    public int MyProperty; => 0;

    // Pretty much anything here causes an error
}
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "ClassDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("Class".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PropertyDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArrowExpressionClause".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "NumericLiteralExpression".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn method() {
    let src = r#"""
                namespace N
                {
                    class Type
                    {
                        } {{closeCurlyTrailingTrivia}}

                        private Constructor() { }
                        ~Destructor() { }
                        private static implicit operator int(Type t) => 0;
                        event Action E1 { add { } remove { } }
                        event Action E2, E3;
                        private int field1, field2;
                        private int this[int i] => 0;
                        private void Method() { }
                        public static Type operator+(Type t1, Type t2) => default;
                        private int Prop => 0;
                    }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Type".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ConstructorDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "ParameterList".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Block".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "DestructorDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "ParameterList".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Block".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ConversionOperatorDeclaration".to_string(),
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
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "Parameter".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("Type".to_string()),
                                                    children: vec![],
                                                }],
                                            }],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ArrowExpressionClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "NumericLiteralExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "EventDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Action".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "AccessorList".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "AddAccessorDeclaration".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "Block".to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "RemoveAccessorDeclaration".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "Block".to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "EventFieldDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "VariableDeclaration".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Action".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "VariableDeclarator".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "VariableDeclarator".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "FieldDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "VariableDeclaration".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PredefinedType".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "VariableDeclarator".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "VariableDeclarator".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "IndexerDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "BracketedParameterList".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "Parameter".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            }],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ArrowExpressionClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "NumericLiteralExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
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
                                            children: vec![],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "OperatorDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Type".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ParameterList".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "Parameter".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "IdentifierName".to_string(),
                                                            token_value: Some("Type".to_string()),
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "Parameter".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "IdentifierName".to_string(),
                                                            token_value: Some("Type".to_string()),
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                            ],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ArrowExpressionClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "DefaultLiteralExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        },
                                    ],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "PropertyDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "ArrowExpressionClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "NumericLiteralExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn constructor() {
    let src = r##"""
                namespace N
                {
                    class Type
                    {
                        {{closeCurlyLeadingTrivia}}
                        }

                        private Constructor() { }
                        {{(closeCurlyLeadingTrivia.Contains("#") ? "#endif" : "")}}
                    }
                }
                """##;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Type".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ConstructorDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "ParameterList".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Block".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn constructor_case_2() {
    let src = r#"""
                namespace N
                {
                    class Type
                    {
                        };

                        // Will not move into type
                        private Constructor() { }
                    }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("Type".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ConstructorDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "ParameterList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Block".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn constructor_case_3() {
    let src = r#"""
                namespace N
                {
                    class Type
                    {
                        } \

                        // Will not move into type
                        private Constructor() { }
                    }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("Type".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ConstructorDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "ParameterList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Block".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn method_case_2() {
    let src = r#"""
                namespace N
                {
                    enum Type
                    {
                    }

                    // This should not be sucked into the enum
                    private void Method() { }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "EnumDeclaration".to_string(),
                            token_value: None,
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
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn method_case_3() {
    let src = r#"""
                namespace N
                {
                    delegate int D();

                    // This should not be sucked into the delegate
                    private void Method() { }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "DelegateDeclaration".to_string(),
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
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn extra_type_only_members_do_not_pull_next_class_in_even_if_private() {
    let src = r#"""
                namespace N
                {
                    class C
                    {
                    }

                    // Will get pulled into C
                    void Method()
                    {
                    }

                    // Not currently pulled in.  But could consider it in the future.
                    private class T
                    {
                    }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("C".to_string()),
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
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("T".to_string()),
                                children: vec![],
                            }],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn extra_type_only_members_do_not_pull_next_class_in_even_if_private_pull_later_members_into_it() {
    let src = r#"""
                namespace N
                {
                    class C
                    {
                    }

                    // Will get pulled into C
                    void Method()
                    {
                    }

                    // Not currently pulled in.  But could consider it in the future.
                    private class T
                    {
                    }
                
                    // Will get pulled into T
                    void Method2()
                    {
                    }
                }
                """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "NamespaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("N".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("C".to_string()),
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
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "ClassDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
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
                                            children: vec![],
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}
