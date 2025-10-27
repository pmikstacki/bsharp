// Auto-generated STRUCTURE tests from Roslyn: TopLevelStatementsParsingTests
use crate::custom_asserts::structure_assert;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_syntax::span::Span;
#[test]
fn insert_open_brace_before_codes() {
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
fn incomplete_global_members() {
    let src = r#"
asas]
extern alias A;
asas
using System;
sadasdasd]

[assembly: goo]

class C
{
}


[a]fod;
[b"#;
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
                                    token_value: Some("asas".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("asas".to_string()),
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("sadasdasd".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("C".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "ExpressionStatement".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "AttributeList".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "Attribute".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("a".to_string()),
                                            children: vec![],
                                        }],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("fod".to_string()),
                                    children: vec![],
                                },
                            ],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "AttributeList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Attribute".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("b".to_string()),
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
fn incomplete_top_level_operator() {
    let src = r#"
fg implicit//
class C { }
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
                        kind: "OperatorDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("fg".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("C".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn global_namespace_with_open_brace_before_namespace() {
    let src = r#"{ namespace n { }"#;
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
                            kind: "Block".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "NamespaceDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("n".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_1056_err_unexpected_character_escaped_backslash() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("S".to_string()),
                            children: vec![],
                        }],
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
                                        token_value: Some("u0065".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("System".to_string()),
                                        children: vec![],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("A".to_string()),
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
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "EqualsValueClause".to_string(),
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
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn neg_invalid_extern_alias_01() {
    let src = r#"{ namespace n { }"#;
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
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("alias".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "VariableDeclarator".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "EqualsValueClause".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "SimpleMemberAccessExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some(
                                                            "other_library".to_string(),
                                                        ),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("dll".to_string()),
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
                    structure_assert::ExpectedNode {
                        kind: "ClassDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("myClass".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn separators_of_separated_syntax_lists() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "Block".to_string(),
                                token_value: None,
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
fn get_diagnostics_on_missing_token() {
    let src = r#"{ namespace n { }"#;
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
                            kind: "LessThanExpression".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("c1".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("t".to_string()),
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
fn get_next_token_excluding_skipped_tokens() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("goo".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("bar".to_string()),
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
fn get_diagnostics_on_missing_token_3() {
    let src = r#"{ namespace n { }"#;
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
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("c2".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "ExpressionStatement".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "NumericLiteralExpression".to_string(),
                                token_value: None,
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
fn main() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("Test".to_string()),
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
                                        token_value: Some("Itest".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "EventDeclaration".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("D".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "ExplicitInterfaceSpecifier".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("ITest".to_string()),
                                            children: vec![],
                                        }],
                                    },
                                ],
                            },
                        ],
                    },
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "TupleType".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
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
                                        kind: "ReturnStatement".to_string(),
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_1514_err_lbrace_expected_02() {
    let src = r#"{ namespace n { }"#;
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
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("S".to_string()),
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("D".to_string()),
                                    children: vec![],
                                }],
                            }],
                        }],
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
                                kind: "ExplicitInterfaceSpecifier".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("P".to_string()),
                                    children: vec![],
                                }],
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_1022_err_eofexpected_02() {
    let src = r#"{ namespace n { }"#;
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
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "GreaterThanExpression".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "SuppressNullableWarningExpression".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "SimpleMemberAccessExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "SimpleMemberAccessExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("Roslyn".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("Utilities".to_string()),
                                                        children: vec![],
                                                    },
                                                ],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("dll".to_string()),
                                                children: vec![],
                                            },
                                        ],
                                    }],
                                },
                            ],
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Basic".to_string()),
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
fn cs_0267_err_partial_misplaced_delegate_1() {
    let src = r#"{ namespace n { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![
                    structure_assert::ExpectedNode {
                        kind: "DelegateDeclaration".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("E".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "Block".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn cs_0116_err_namespace_unexpected() {
    let src = r#"{ namespace n { }"#;
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
                        kind: "Block".to_string(),
                        token_value: None,
                        children: vec![
                            structure_assert::ExpectedNode {
                                kind: "ExpressionStatement".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("get".to_string()),
                                    children: vec![],
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
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("ParseDefaultDir".to_string()),
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn multiplication() {
    let src = r#"{ namespace n { }"#;
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
                                    kind: "PointerType".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "QualifiedName".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("a".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("b".to_string()),
                                                children: vec![],
                                            },
                                        ],
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
fn neg_if_endif_directives_with_bad_code() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("aeu".to_string()),
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
fn extern_without_alias() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("a".to_string()),
                            children: vec![],
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
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_symbol_for_invalid_code() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("A".to_string()),
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
                    },
                    structure_assert::ExpectedNode {
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
                                        kind: "ReturnStatement".to_string(),
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
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn invalid_alias() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("alias".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
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
fn top_level_indexer() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "ElementAccessExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "ThisExpression".to_string(),
                                        token_value: None,
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "BracketedArgumentList".to_string(),
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
                                                    token_value: Some("E".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                        ],
                                    },
                                ],
                            }],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "Block".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "ExpressionStatement".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("get".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Block".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "ReturnStatement".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "IdentifierName".to_string(),
                                            token_value: Some("E".to_string()),
                                            children: vec![],
                                        }],
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
fn unrecognized_generic_type_reference() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "LessThanExpression".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "PredefinedType".to_string(),
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
fn incomplete_operator() {
    let src = r#"{ namespace n { }"#;
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    }],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "Parameter".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("C".to_string()),
                                        children: vec![],
                                    }],
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn new_keyword() {
    let src = r#"{ namespace n { }"#;
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
                            kind: "ObjectCreationExpression".to_string(),
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
fn tuple_unsupported_in_using_statement() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("VT2".to_string()),
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
fn metadata_reference_with_invalid_alias() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("alias".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "ParameterList".to_string(),
                                token_value: None,
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
fn identifier_01() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("e".to_string()),
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
fn identifier_02() {
    let src = r#"{ namespace n { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "IncompleteMember".to_string(),
                    token_value: None,
                    children: vec![
                        structure_assert::ExpectedNode {
                            kind: "AttributeList".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "Attribute".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Flags".to_string()),
                                    children: vec![],
                                }],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("e".to_string()),
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
fn skipped_text() {
    let src = r#"{ namespace n { }"#;
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
                    children: vec![],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_local_declaration_01() {
    let src = r#"{ namespace n { }"#;
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("var".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "VariableDeclarator".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "EqualsValueClause".to_string(),
                                        token_value: None,
                                        children: vec![structure_assert::ExpectedNode {
                                            kind: "ObjectCreationExpression".to_string(),
                                            token_value: None,
                                            children: vec![
                                                structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("MyDisposable".to_string()),
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
                        }],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_local_declaration_02() {
    let src = r#"{ namespace n { }"#;
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("type".to_string()),
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
fn using_local_declaration_03() {
    let src = r#"{ namespace n { }"#;
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
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_local_declaration_04() {
    let src = r#"{ namespace n { }"#;
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
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_local_declaration_05() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "RefType".to_string(),
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
fn using_local_declaration_06() {
    let src = r#"{ namespace n { }"#;
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
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_directive_01() {
    let src = r#"{ namespace n { }"#;
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
                        token_value: Some("type".to_string()),
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_directive_02() {
    let src = r#"{ namespace n { }"#;
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
                        token_value: Some("type".to_string()),
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_directive_03() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("alias".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("type".to_string()),
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
fn using_directive_04() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("ns".to_string()),
                                children: vec![],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("type".to_string()),
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
fn using_directive_05() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("alias".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("type".to_string()),
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
fn using_directive_06() {
    let src = r#"{ namespace n { }"#;
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
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                }],
                            }],
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Parse".to_string()),
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
                                                token_value: Some("value".to_string()),
                                                children: vec![],
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
fn using_directive_07() {
    let src = r#"{ namespace n { }"#;
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
                                    kind: "PredefinedType".to_string(),
                                    token_value: None,
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "VariableDeclarator".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "BracketedArgumentList".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("x".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "Argument".to_string(),
                                                token_value: None,
                                                children: vec![structure_assert::ExpectedNode {
                                                    kind: "IdentifierName".to_string(),
                                                    token_value: Some("y".to_string()),
                                                    children: vec![],
                                                }],
                                            },
                                        ],
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
fn using_directive_08() {
    let src = r#"{ namespace n { }"#;
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
fn repro_611177() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "AttributeList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "Attribute".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
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
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("_".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "ArrayRankSpecifier".to_string(),
                                                        token_value: None,
                                                        children: vec![
                                                            structure_assert::ExpectedNode {
                                                                kind: "AnonymousMethodExpression"
                                                                    .to_string(),
                                                                token_value: None,
                                                                children: vec![],
                                                            },
                                                        ],
                                                    },
                                                ],
                                            }],
                                        }],
                                    }],
                                }],
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
                                kind: "CharacterLiteralExpression".to_string(),
                                token_value: None,
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
fn constructor_like_01() {
    let src = r#"{ namespace n { }"#;
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
                                        token_value: Some("local".to_string()),
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
                            kind: "Block".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn constructor_like_02() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("local".to_string()),
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn constructor_like_03() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "AttributeList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "Attribute".to_string(),
                                    token_value: None,
                                    children: vec![structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("attribute".to_string()),
                                        children: vec![],
                                    }],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "IdentifierName".to_string(),
                                token_value: Some("local".to_string()),
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
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn using_alias_test() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("s".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "FunctionPointerType".to_string(),
                            token_value: None,
                            children: vec![structure_assert::ExpectedNode {
                                kind: "FunctionPointerParameterList".to_string(),
                                token_value: None,
                                children: vec![structure_assert::ExpectedNode {
                                    kind: "FunctionPointerParameter".to_string(),
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
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_01() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("ar".to_string()),
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
fn error_recovery_02() {
    let src = r#"{ namespace n { }"#;
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
                                        kind: "SimpleMemberAccessExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Console".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("WriteLine".to_string()),
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
                                    token_value: Some("ar".to_string()),
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
fn error_recovery_03() {
    let src = r#"{ namespace n { }"#;
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
                                    token_value: Some("ar".to_string()),
                                    children: vec![],
                                }],
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
                                        kind: "SimpleMemberAccessExpression".to_string(),
                                        token_value: None,
                                        children: vec![
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("Console".to_string()),
                                                children: vec![],
                                            },
                                            structure_assert::ExpectedNode {
                                                kind: "IdentifierName".to_string(),
                                                token_value: Some("WriteLine".to_string()),
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
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_04() {
    let src = r#"{ namespace n { }"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree {
            root: structure_assert::ExpectedNode {
                kind: "CompilationUnit".to_string(),
                token_value: None,
                children: vec![structure_assert::ExpectedNode {
                    kind: "IncompleteMember".to_string(),
                    token_value: None,
                    children: vec![structure_assert::ExpectedNode {
                        kind: "IdentifierName".to_string(),
                        token_value: Some("alias".to_string()),
                        children: vec![],
                    }],
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_05() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("aliasY".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "QualifiedName".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Y".to_string()),
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
fn error_recovery_06() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("X".to_string()),
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
                                    token_value: Some("aliasY".to_string()),
                                    children: vec![],
                                }],
                            },
                            structure_assert::ExpectedNode {
                                kind: "QualifiedName".to_string(),
                                token_value: None,
                                children: vec![
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("X".to_string()),
                                        children: vec![],
                                    },
                                    structure_assert::ExpectedNode {
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Y".to_string()),
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
fn error_recovery_07() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("aliasY".to_string()),
                                children: vec![],
                            }],
                        },
                        structure_assert::ExpectedNode {
                            kind: "QualifiedName".to_string(),
                            token_value: None,
                            children: vec![
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("X".to_string()),
                                    children: vec![],
                                },
                                structure_assert::ExpectedNode {
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("Y".to_string()),
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
fn error_recovery_08() {
    let src = r#"{ namespace n { }"#;
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
                                    token_value: Some("scoped".to_string()),
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
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "RefType".to_string(),
                            token_value: None,
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "StructDeclaration".to_string(),
                        token_value: None,
                        children: vec![],
                    },
                    structure_assert::ExpectedNode {
                        kind: "IncompleteMember".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("scoped".to_string()),
                            children: vec![],
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
fn error_recovery_09() {
    let src = r#"{ namespace n { }"#;
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
fn error_recovery_10() {
    let src = r#"{ namespace n { }"#;
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
                                token_value: Some("Point".to_string()),
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
fn error_recovery_11() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("Goo".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("Bar".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_12() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("Goo".to_string()),
                            children: vec![],
                        }],
                    },
                    structure_assert::ExpectedNode {
                        kind: "UsingDirective".to_string(),
                        token_value: None,
                        children: vec![structure_assert::ExpectedNode {
                            kind: "IdentifierName".to_string(),
                            token_value: Some("Bar".to_string()),
                            children: vec![],
                        }],
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_13() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("Goo".to_string()),
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
                                    kind: "IdentifierName".to_string(),
                                    token_value: Some("p".to_string()),
                                    children: vec![],
                                }],
                            }],
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Bar".to_string()),
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
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_14() {
    let src = r#"{ namespace n { }"#;
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
                            token_value: Some("Goo".to_string()),
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("p".to_string()),
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
                                        kind: "IdentifierName".to_string(),
                                        token_value: Some("Bar".to_string()),
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
                    },
                ],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn error_recovery_15() {
    let src = r#"{ namespace n { }"#;
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
                                kind: "IdentifierName".to_string(),
                                token_value: Some("W".to_string()),
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
fn empty_local_declaration() {
    let src = r#""" 
struct S { }
partial ext X
"""#;
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
