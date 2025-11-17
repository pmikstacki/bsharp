// Auto-generated STRUCTURE tests from Roslyn: RecordParsing
#[test]
fn using_tree() {
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
fn using_tree_case_2() {
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
fn field_named_data() {
    let src = r#"
class C
{
    int data;
}"#;
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
                            token_value: Some("C".to_string()),
                            children: vec![],
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
fn record_parsing_01() {
    let src = r#"record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_01_case_2() {
    let src = r#"record C(int X, int Y);"#;
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("record".to_string()),
                                children: vec![],
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
                        ],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_02() {
    let src = r#"record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_03() {
    let src = r#"record C;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_04() {
    let src = r#"record C { public int record; }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
fn record_parsing_ambiguities() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
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
                        kind: "RecordDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "ParameterList".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "RecordDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "RecordDeclaration".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "ParameterList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "RecordDeclaration".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "ParameterList".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
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
fn record_parsing_constraint_and_semi_colon() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_constraint_and_semi_colon_missing_colon() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_two_constraints_and_semi_colon() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "TypeParameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T1".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "TypeParameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T2".to_string()),
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T1".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T2".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_constraint_and_semi_colon_class() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
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
                            token_value: Some("C".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_two_constraints_and_semi_colon_class() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
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
                            token_value: Some("C".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "TypeParameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T1".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "TypeParameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T2".to_string()),
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T1".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T2".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn abstract_method_constraints_and_semi_colon() {
    let src = r#"
record R1() { return null; }
abstract record D
{
    record R2() { return null; }
    abstract record R3();
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "MethodDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "TypeParameterList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeParameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "TypeParameterConstraintClause".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("T".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ReferenceType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                ],
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
fn record_parsing_block_body_and_semi_colon() {
    let src = r#"record C { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn class_with_multiple_constraints_001() {
    let src = r#"record C { };"#;
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
                            token_value: Some("a".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("b".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("b".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SpecificType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("c".to_string()),
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("b".to_string()),
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
fn class_with_multiple_constraints_002() {
    let src = r#"record C { };"#;
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
                            token_value: Some("a".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("b".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("b".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SpecificType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("c".to_string()),
                                        children: vec![],
                                    }],
                                },
                            ],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![],
                        },
                    ],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_constraints_and_curly_braces() {
    let src = r#"record C { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_constraint_and_comma_and_semi_colon() {
    let src = r#"record C { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
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
fn record_parsing_constraint_and_comma_and_new_and_semi_colon() {
    let src = r#"record C { };"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterConstraintClause".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ReferenceType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Constructor".to_string(),
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
fn where_where() {
    let src = r#"record C { };"#;
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
                            token_value: Some("Goo".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "QualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("System".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Object".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("where".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("where".to_string()),
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
fn where_where_where() {
    let src = r#"record C { };"#;
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
                            token_value: Some("Goo".to_string()),
                            children: vec![],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TypeParameterList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "TypeParameter".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("T".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "QualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("System".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Object".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("where".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("where".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("where".to_string()),
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
fn with_parsing_1() {
    let src = r#"
class C
{
    with { };
    x with { };
    int x = with { };
    int x = 0 with { };
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
                                token_value: Some("C".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IncompleteMember".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("with".to_string()),
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "AccessorList".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
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
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "VariableDeclarator".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "EqualsValueClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("with".to_string()),
                                                children: vec![],
                                            }],
                                        }],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "EmptyStatement".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
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
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "VariableDeclarator".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "EqualsValueClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "WithExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "NumericLiteralExpression"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "WithInitializerExpression"
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
fn with_parsing_2() {
    let src = r#"
class C
{
    int M()
    {
        int x = M() with { } + 3;
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "EqualsValueClause".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "AddExpression".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "WithExpression".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("M".to_string()), children: vec![] },                                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] },                                         structure_assert::ExpectedNode { kind: "WithInitializerExpression".to_string(), token_value: None, children: vec![] }] },                                     structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn parameter_list_and_base_list_on_class() {
    let src = r#"
class C(int X, int Y)
: B(X, Y)
{{ }}"#;
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
                            token_value: Some("C".to_string()),
                            children: vec![],
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
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "PrimaryConstructorBaseType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("B".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("X".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("Y".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                        ],
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
fn base_02() {
    let src = r#"record C(int X, int Y)" + @"
: B, D(X, Y)" + @"
" + (withBody ? "{ }" : "#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![
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
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("B".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("D".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("X".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SimpleBaseType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Y".to_string()),
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
fn base_03() {
    let src = r#"interface C : B;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "InterfaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "BaseList".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "SimpleBaseType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("B".to_string()),
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
fn base_04() {
    let src = r#"interface C(int X, int Y) : B;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "InterfaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![
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
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "SimpleBaseType".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("B".to_string()),
                                    children: vec![],
                                }],
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
fn base_05() {
    let src = r#"interface C : B(X, Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "InterfaceDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "BaseList".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "PrimaryConstructorBaseType".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("B".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "Argument".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("X".to_string()),
                                                children: vec![],
                                            }],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "Argument".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Y".to_string()),
                                                children: vec![],
                                            }],
                                        },
                                    ],
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
fn record_struct_parsing_record_named_struct() {
    let src = r#"record struct(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_record_named_struct_case_2() {
    let src = r#"record struct(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing() {
    let src = r#"record struct C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
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
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_case_2() {
    let src = r#"record struct C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
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
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_case_3() {
    let src = r#"record struct C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
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
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_with_body() {
    let src = r#"record struct C(int X, int Y) { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing() {
    let src = r#"record class C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing_case_2() {
    let src = r#"record class C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing_case_3() {
    let src = r#"record class C(int X, int Y);"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_interface_parsing() {
    let src = r#"record interface C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "RecordDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "InterfaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
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
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_record_parsing() {
    let src = r#"record record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "RecordDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
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
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("X".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("Y".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                        ],
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
fn record_struct_parsing_wrong_order_csharp_10() {
    let src = r#"struct record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_wrong_order_csharp_9() {
    let src = r#"struct record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn struct_named_record_csharp_8() {
    let src = r#"struct record { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "StructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn struct_named_record_csharp_9() {
    let src = r#"struct record { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "StructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn struct_named_record_csharp_10() {
    let src = r#"struct record { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "StructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing_wrong_order_csharp_10() {
    let src = r#"class record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing_wrong_order_csharp_9() {
    let src = r#"class record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_interface_parsing_wrong_order() {
    let src = r#"interface record C(int X, int Y);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "InterfaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
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
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("X".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("Y".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                        ],
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
fn record_struct_parsing_partial() {
    let src = r#"partial record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_class_parsing_partial() {
    let src = r#"partial record class S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_partial() {
    let src = r#"partial record S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_partial_with_parameter_list() {
    let src = r#"partial record struct S(int X);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_partial_with_parameter_list_and_members() {
    let src = r#"partial record struct S(int X) { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_readonly() {
    let src = r#"readonly record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_readonly_partial() {
    let src = r#"readonly partial record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_partial_readonly() {
    let src = r#"partial readonly record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("partial".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "RecordStructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_new() {
    let src = r#"new record struct S;"#;
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
                                kind: "ObjectCreationExpression".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_ref() {
    let src = r#"ref record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("record".to_string()),
                                children: vec![],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_ref_case_2() {
    let src = r#"ref record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("record".to_string()),
                                children: vec![],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_ref_case_3() {
    let src = r#"ref record struct S;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_parsing_ref() {
    let src = r#"ref record R;"#;
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
                        kind: "LocalDeclarationStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "VariableDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "RefType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("record".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "VariableDeclarator".to_string(),
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
fn record_parsing_ref_case_2() {
    let src = r#"ref record R;"#;
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
                        kind: "LocalDeclarationStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "VariableDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "RefType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("record".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "VariableDeclarator".to_string(),
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
fn record_parsing_ref_case_3() {
    let src = r#"ref record R;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordDeclaration".to_string(),
                    token_value: None,
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_const() {
    let src = r#"const record struct S;"#;
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
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_fixed() {
    let src = r#"fixed record struct S;"#;
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
                        children: vec![structure_assert::ExpectedNode {
                            kind: "VariableDeclaration".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("record".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "VariableDeclarator".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "BracketedArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Argument".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "OmittedArraySizeExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        }],
                                    }],
                                },
                            ],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn record_struct_parsing_base_list_with_parens() {
    let src = r#"record struct S : Base(1);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "BaseList".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "PrimaryConstructorBaseType".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Base".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "ArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "Argument".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "NumericLiteralExpression".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
                                    }],
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
fn record_struct_parsing_base_list_with_parens_with_positional_parameter_list() {
    let src = r#"record struct S(int X) : Base(1);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "RecordStructDeclaration".to_string(),
                    token_value: None,
                    children: vec![
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
                            kind: "BaseList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "PrimaryConstructorBaseType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Base".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Argument".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "NumericLiteralExpression".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        }],
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
