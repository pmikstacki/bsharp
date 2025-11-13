// Auto-generated STRUCTURE tests from Roslyn: UsingDirectiveParsingTests
#[test]
fn simple_using_directive_name_pointer() {
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
fn simple_using_directive_ref_type() {
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
fn simple_using_directive_function_pointer() {
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
fn simple_using_directive_predefined_type() {
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
fn simple_using_directive_predefined_type_pointer() {
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
fn simple_using_directive_tuple() {
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
fn static_using_directive_name_pointer() {
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
fn static_using_directive_ref_type() {
    let src = r#"using static x = ref int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn static_using_directive_function_pointer() {
    let src = r#"using static x = ref int;"#;
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
                            children: vec![structure_assert::ExpectedNode {
                                kind: "FunctionPointerType".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "FunctionPointerParameterList".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "FunctionPointerParameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "PredefinedType".to_string(),
                                                token_value: None,
                                                children: vec![],
                                            }],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "FunctionPointerParameter".to_string(),
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
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn static_using_directive_predefined_type() {
    let src = r#"using static x = ref int;"#;
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
fn static_using_directive_predefined_type_pointer() {
    let src = r#"using static x = ref int;"#;
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
                            children: vec![structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
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
fn static_using_directive_tuple() {
    let src = r#"using static x = ref int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
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
                                kind: "TupleExpression".to_string(),
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
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
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
fn alias_using_directive_name_pointer_1() {
    let src = r#"using x = A*;

struct A { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("x".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("A".to_string()),
                                    children: vec![],
                                }],
                            },
                        ],
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
fn alias_using_directive_name_pointer_2() {
    let src = r#"using unsafe x = A*;

struct A { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("x".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("A".to_string()),
                                    children: vec![],
                                }],
                            },
                        ],
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
fn alias_using_directive_function_pointer_1() {
    let src = r#"using x = delegate*<int, void>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "FunctionPointerType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "FunctionPointerParameterList".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "FunctionPointerParameter".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "FunctionPointerParameter".to_string(),
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
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_function_pointer_2() {
    let src = r#"using unsafe x = delegate*<int, void>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "FunctionPointerType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "FunctionPointerParameterList".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "FunctionPointerParameter".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "FunctionPointerParameter".to_string(),
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
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_unsafe_non_alias() {
    let src = r#"using unsafe System;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "IdentifierName".to_string(),
                        token_value: Some("System".to_string()),
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_predefined_type_csharp_11() {
    let src = r#"using x = int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
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
fn alias_using_directive_predefined_type_csharp_12() {
    let src = r#"using x = int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
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
fn alias_using_directive_predefined_type_preview() {
    let src = r#"using x = int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PredefinedType".to_string(),
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
fn alias_using_directive_ref_type() {
    let src = r#"using x = ref int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_ref_readonly_type() {
    let src = r#"using x = ref readonly int;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_predefined_type_pointer_1() {
    let src = r#"using x = int*;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PointerType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_predefined_type_pointer_2() {
    let src = r#"using unsafe x = int*;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "PointerType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_predefined_type_pointer_3() {
    let src = r#"
using unsafe X = int*;

namespace N
{
    using Y = X;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("X".to_string()),
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
fn alias_using_directive_predefined_type_pointer_4() {
    let src = r#"
using unsafe X = int*;

namespace N
{
    using unsafe Y = X;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("X".to_string()),
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
fn alias_using_directive_predefined_type_pointer_5() {
    let src = r#"
using X = int*;

namespace N
{
    using unsafe Y = X;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("X".to_string()),
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
fn alias_using_directive_predefined_type_pointer_6() {
    let src = r#"
using unsafe X = int*;

namespace N
{
    using Y = X[];
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("X".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
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
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_predefined_type_pointer_7() {
    let src = r#"
using unsafe X = int*;

namespace N
{
    using unsafe Y = X[];
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("X".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
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
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_tuple_1() {
    let src = r#"using x = (int, int);"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "TupleType".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "TupleElement".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "TupleElement".to_string(),
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_tuple_2() {
    let src = r#"""
            using X = (int, int);

            class C
            {
                X x = (0, 0);
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "TupleType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "TupleElement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "TupleElement".to_string(),
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
                                kind: "FieldDeclaration".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "VariableDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "VariableDeclarator".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "EqualsValueClause".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "TupleExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "Argument".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind:
                                                                        "NumericLiteralExpression"
                                                                            .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                        structure_assert::ExpectedNode {
                                                            kind: "Argument".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind:
                                                                        "NumericLiteralExpression"
                                                                            .to_string(),
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
fn alias_using_directive_tuple_3() {
    let src = r#"""
            using X = (int, int);

            class C
            {
                X x = (true, false);
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "TupleType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "TupleElement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "PredefinedType".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "TupleElement".to_string(),
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
                                kind: "FieldDeclaration".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "VariableDeclaration".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "VariableDeclarator".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "EqualsValueClause".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "TupleExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "Argument".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "TrueLiteralExpression"
                                                                        .to_string(),
                                                                    token_value: None,
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                        structure_assert::ExpectedNode {
                                                            kind: "Argument".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "FalseLiteralExpression"
                                                                        .to_string(),
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
fn alias_using_nullable_value_type() {
    let src = r#"using x = int?;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "NullableType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_nullable_reference_type_1() {
    let src = r#"using x = string?;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("x".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "NullableType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_nullable_reference_type_2() {
    let src = r#"""
            #nullable enable
            using X = string?;
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("X".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "NullableType".to_string(),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_nullable_reference_type_3() {
    let src = r#"""
            using X = string;
            namespace N
            {
                using Y = X?;
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "NullableType".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
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
fn alias_using_nullable_reference_type_4() {
    let src = r#"""
            #nullable enable
            using X = string;
            namespace N
            {
                using Y = X?;
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Y".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "NullableType".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
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
fn alias_using_void_pointer_1() {
    let src = r#"using unsafe VP = void*;

class C
{
    void M(VP vp) { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("VP".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("VP".to_string()),
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
fn alias_using_void_pointer_2() {
    let src = r#"using unsafe VP = void*;

class C
{
    unsafe void M(VP vp) { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("VP".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("VP".to_string()),
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
fn alias_using_void_pointer_3() {
    let src = r#"using VP = void*;

class C
{
    unsafe void M(VP vp) { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("VP".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PointerType".to_string(),
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("VP".to_string()),
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
fn alias_using_void_1() {
    let src = r#"using V = void;

class C
{
    void M(V v) { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("V".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("V".to_string()),
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
fn alias_using_void_2() {
    let src = r#"using V = void;

class C
{
    V M() { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("V".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
                            },
                        ],
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("V".to_string()),
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_void_3() {
    let src = r#"using V = void[];

class C
{
    V M() { }
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("V".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ArrayType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArrayRankSpecifier".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "OmittedArraySizeExpression".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
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
                                token_value: Some("C".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "MethodDeclaration".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("V".to_string()),
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_directive_dynamic_1() {
    let src = r#"
using dynamic;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "IdentifierName".to_string(),
                        token_value: Some("dynamic".to_string()),
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_dynamic_1() {
    let src = r#"
using D = dynamic;

class C
{
    void M(D d)
    {
        d.Goo();
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("D".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("dynamic".to_string()),
                                children: vec![],
                            },
                        ],
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("D".to_string()),
                                                children: vec![],
                                            }],
                                        }],
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
                                                        kind: "SimpleMemberAccessExpression"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some("d".to_string()),
                                                                children: vec![],
                                                            },
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some(
                                                                    "Goo".to_string(),
                                                                ),
                                                                children: vec![],
                                                            },
                                                        ],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_dynamic_2() {
    let src = r#"
using D = System.Collections.Generic.List<dynamic>;

class C
{
    void M(D d)
    {
        d[0].Goo();
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "UsingDirective".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "NameEquals".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("D".to_string()), children: vec![] }] },         structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "QualifiedName".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("System".to_string()), children: vec![] },                     structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Collections".to_string()), children: vec![] }] },                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Generic".to_string()), children: vec![] }] },             structure_assert::ExpectedNode { kind: "GenericName".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "TypeArgumentList".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("dynamic".to_string()), children: vec![] }] }] }] }] },     structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("D".to_string()), children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "ElementAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("d".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "BracketedArgumentList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] },                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Goo".to_string()), children: vec![] }] },                         structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_dynamic_3() {
    let src = r#"
using D = dynamic[];

class C
{
    void M(D d)
    {
        d[0].Goo();
    }
}"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "UsingDirective".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "NameEquals".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("D".to_string()), children: vec![] }] },         structure_assert::ExpectedNode { kind: "ArrayType".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("dynamic".to_string()), children: vec![] },             structure_assert::ExpectedNode { kind: "ArrayRankSpecifier".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "OmittedArraySizeExpression".to_string(), token_value: None, children: vec![] }] }] }] },     structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "Parameter".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("D".to_string()), children: vec![] }] }] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "ElementAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("d".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "BracketedArgumentList".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                        structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] },                             structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Goo".to_string()), children: vec![] }] },                         structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_dynamic_4() {
    let src = r#"
using D = dynamic;

class dynamic
{
    void M(D d)
    {
        d.Goo();
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("D".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("dynamic".to_string()),
                                children: vec![],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("dynamic".to_string()),
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
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("D".to_string()),
                                                children: vec![],
                                            }],
                                        }],
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
                                                        kind: "SimpleMemberAccessExpression"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some("d".to_string()),
                                                                children: vec![],
                                                            },
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some(
                                                                    "Goo".to_string(),
                                                                ),
                                                                children: vec![],
                                                            },
                                                        ],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_directive_dynamic_5() {
    let src = r#"
// Note: this is weird, but is supported by language.  It checks just that the ValueText is `dynamic`, not the raw text.
using D = @dynamic;

class C
{
    void M(D d)
    {
        d.Goo();
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("D".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("@dynamic".to_string()),
                                children: vec![],
                            },
                        ],
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
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "Parameter".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("D".to_string()),
                                                children: vec![],
                                            }],
                                        }],
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
                                                        kind: "SimpleMemberAccessExpression"
                                                            .to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some("d".to_string()),
                                                                children: vec![],
                                                            },
                                                            structure_assert::ExpectedNode {
                                                                kind: "IdentifierName".to_string(),
                                                                token_value: Some(
                                                                    "Goo".to_string(),
                                                                ),
                                                                children: vec![],
                                                            },
                                                        ],
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
                        ],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn alias_using_duplicate_1() {
    let src = r#"""
            using X = int?;
            using X = System;
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "NullableType".to_string(),
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("System".to_string()),
                                children: vec![],
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
fn alias_using_duplicate_2() {
    let src = r#"""
            using X = int?;
            using X = int;
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "NullableType".to_string(),
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "PredefinedType".to_string(),
                                token_value: None,
                                children: vec![],
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
fn alias_using_duplicate_3() {
    let src = r#"""
            using X = int?;
            using X = System.Int32;
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "NullableType".to_string(),
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
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
                                        token_value: Some("Int32".to_string()),
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
fn alias_using_not_duplicate_1() {
    let src = r#"""
            using X = int?;
            namespace N;
            using X = int;
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "NullableType".to_string(),
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
                        kind: "FileScopedNamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("N".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "UsingDirective".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "NameEquals".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
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
fn scoped_type_1() {
    let src = r#"
using scoped int;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("scoped".to_string()),
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
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
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
fn scoped_type_2() {
    let src = r#"
using X = scoped int;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("scoped".to_string()),
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
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
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
fn scoped_type_3() {
    let src = r#"
using X = scoped System;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("scoped".to_string()),
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("System".to_string()),
                                children: vec![],
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
fn scoped_type_4() {
    let src = r#"
using X = scoped System.AppDomain;
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
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("scoped".to_string()),
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
                                kind: "SimpleMemberAccessExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("System".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("AppDomain".to_string()),
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
fn obsolete_1() {
    let src = r#"""
            using System;
            using X = C;

            [Obsolete("", error: true)]
            class C
            {
            }

            class D
            {
                X x;
                C c;
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("System".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("C".to_string()),
                                children: vec![],
                            },
                        ],
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
                                kind: "AttributeList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "Attribute".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Obsolete".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "AttributeArgumentList".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "AttributeArgument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "StringLiteralExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "AttributeArgument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "NameColon".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "IdentifierName"
                                                                        .to_string(),
                                                                    token_value: Some(
                                                                        "error".to_string(),
                                                                    ),
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                        structure_assert::ExpectedNode {
                                                            kind: "TrueLiteralExpression"
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
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("D".to_string()),
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
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
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
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("C".to_string()),
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
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn obsolete_2() {
    let src = r#"""
            using System;
            using X = C[];

            [Obsolete("", error: true)]
            class C
            {
            }

            class D
            {
                X x1;
                C[] c1;
            }
            """#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("System".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "NameEquals".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ArrayType".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ArrayRankSpecifier".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "OmittedArraySizeExpression".to_string(),
                                            token_value: None,
                                            children: vec![],
                                        }],
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
                                token_value: Some("C".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "AttributeList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "Attribute".to_string(),
                                    token_value: None,
                                    children: vec![
                                        structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("Obsolete".to_string()),
                                            children: vec![],
                                        },
                                        structure_assert::ExpectedNode {
                                            kind: "AttributeArgumentList".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "AttributeArgument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "StringLiteralExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "AttributeArgument".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "NameColon".to_string(),
                                                            token_value: None,
                                                            children: vec![
                                                                structure_assert::ExpectedNode {
                                                                    kind: "IdentifierName"
                                                                        .to_string(),
                                                                    token_value: Some(
                                                                        "error".to_string(),
                                                                    ),
                                                                    children: vec![],
                                                                },
                                                            ],
                                                        },
                                                        structure_assert::ExpectedNode {
                                                            kind: "TrueLiteralExpression"
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
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("D".to_string()),
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
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("X".to_string()),
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
                                            kind: "ArrayType".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("C".to_string()),
                                                    children: vec![],
                                                },
                                                structure_assert::ExpectedNode {
                                                    kind: "ArrayRankSpecifier".to_string(),
                                                    token_value: None,
                                                    children: vec![
                                                        structure_assert::ExpectedNode {
                                                            kind: "OmittedArraySizeExpression"
                                                                .to_string(),
                                                            token_value: None,
                                                            children: vec![],
                                                        },
                                                    ],
                                                },
                                            ],
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
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn arg_list() {
    let src = r#"
using X = __arglist;
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "NameEquals".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("X".to_string()),
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
fn makeref() {
    let src = r#"
using X = __makeref;
"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "NameEquals".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("X".to_string()),
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
fn unsafe_static_1_csharp_11_no_unsafe_flag() {
    let src = r#"using unsafe static System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn unsafe_static_1_csharp_11_unsafe_flag() {
    let src = r#"using unsafe static System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn unsafe_static_1_csharp_12_no_unsafe_flag() {
    let src = r#"using unsafe static System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn unsafe_static_1_csharp_12_unsafe_flag() {
    let src = r#"using unsafe static System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn unsafe_static_2_csharp_11_no_unsafe_flag() {
    let src = r#"using unsafe static X = System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("X".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
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
                                    token_value: Some("Console".to_string()),
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
fn unsafe_static_2_csharp_11_unsafe_flag() {
    let src = r#"using unsafe static X = System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("X".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
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
                                    token_value: Some("Console".to_string()),
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
fn unsafe_static_2_csharp_12_no_unsafe_flag() {
    let src = r#"using unsafe static X = System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("X".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
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
                                    token_value: Some("Console".to_string()),
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
fn unsafe_static_2_csharp_12_unsafe_flag() {
    let src = r#"using unsafe static X = System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "NameEquals".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("X".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
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
                                    token_value: Some("Console".to_string()),
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
fn using_static_unsafe_safe_type_csharp_11_no_unsafe_flag() {
    let src = r#"using static unsafe System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn using_static_unsafe_safe_type_csharp_11_unsafe_flag() {
    let src = r#"using static unsafe System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn using_static_unsafe_safe_type_csharp_12_no_unsafe_flag() {
    let src = r#"using static unsafe System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn using_static_unsafe_safe_type_csharp_12_unsafe_flag() {
    let src = r#"using static unsafe System.Console;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
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
                                token_value: Some("Console".to_string()),
                                children: vec![],
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
fn using_static_unsafe_unsafe_type_csharp_11_no_unsafe_flag() {
    let src = r#"using static unsafe System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_unsafe_type_csharp_11_unsafe_flag() {
    let src = r#"using static unsafe System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_unsafe_type_csharp_12_no_unsafe_flag() {
    let src = r#"using static unsafe System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_unsafe_type_csharp_12_unsafe_flag() {
    let src = r#"using static unsafe System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_type_csharp_11_no_unsafe_flag() {
    let src = r#"using static System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_type_csharp_11_unsafe_flag() {
    let src = r#"using static System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_type_csharp_12_no_unsafe_flag() {
    let src = r#"using static System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
fn using_static_unsafe_type_csharp_12_unsafe_flag() {
    let src = r#"using static System.Collections.Generic.List<int*[]>;"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "UsingDirective".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "QualifiedName".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
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
                                                token_value: Some("Collections".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Generic".to_string()),
                                        children: vec![],
                                    },
                                ],
                            },
                            structure_assert::ExpectedNode {
                                kind: "GenericName".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "TypeArgumentList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ArrayType".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "PointerType".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "PredefinedType".to_string(),
                                                    token_value: None,
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "ArrayRankSpecifier".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "OmittedArraySizeExpression".to_string(),
                                                    token_value: None,
                                                    children: vec![],
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
