// Auto-generated STRUCTURE tests from Roslyn: StatementParsingTests
use crate::custom_asserts::structure_assert;
use bsharp_parser::bsharp::parse_csharp_source_strict;
use bsharp_parser::syntax::span::Span;
#[test]
fn await_using_var_with_var_decl_reversed() {
    let src = r#"await using var a = b;"#;
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
                                            kind: "LocalDeclarationStatement".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "VariableDeclaration".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("await".to_string()),
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
                                            kind: "ExpressionStatement".to_string(),
                                            token_value: None,
                                            children: vec![structure_assert::ExpectedNode {
                                                kind: "SimpleAssignmentExpression".to_string(),
                                                token_value: None,
                                                children: vec![
                                                    structure_assert::ExpectedNode {
                                                        kind: "IdentifierName".to_string(),
                                                        token_value: Some("x".to_string()),
                                                        children: vec![],
                                                    },
                                                    structure_assert::ExpectedNode {
                                                        kind: "NullLiteralExpression".to_string(),
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
                }],
            },
        };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn parse_case_without_switch() {
    let src = r#"if (true)
System.Console.WriteLine(true)"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SwitchStatement".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "SwitchSection".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "CasePatternSwitchLabel".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypePattern".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] },                         structure_assert::ExpectedNode { kind: "WhenClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("SomeTest".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },                     structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Console".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("WriteLine".to_string()), children: vec![] }] },                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "StringLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] },                     structure_assert::ExpectedNode { kind: "BreakStatement".to_string(), token_value: None, children: vec![] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn parse_errant_statement_in_case_1() {
    let src = r#"if (true)
System.Console.WriteLine(true)"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SwitchStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("expr".to_string()), children: vec![] }] },                 structure_assert::ExpectedNode { kind: "LocalDeclarationStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "VariableDeclaration".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                         structure_assert::ExpectedNode { kind: "VariableDeclarator".to_string(), token_value: None, children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "SwitchStatement".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "SwitchSection".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "CasePatternSwitchLabel".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypePattern".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] },                         structure_assert::ExpectedNode { kind: "WhenClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("SomeTest".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },                     structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Console".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("WriteLine".to_string()), children: vec![] }] },                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "StringLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] },                     structure_assert::ExpectedNode { kind: "BreakStatement".to_string(), token_value: None, children: vec![] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}

#[test]
fn parse_errant_statement_in_case_2() {
    let src = r#"if (true)
System.Console.WriteLine(true)"#;
    let span = Span::new(src);
    let r = parse_csharp_source_strict(span);
    if let Ok((_rest, unit)) = r {
        let expected = structure_assert::ExpectedTree { root: structure_assert::ExpectedNode { kind: "CompilationUnit".to_string(), token_value: None, children: vec![    structure_assert::ExpectedNode { kind: "ClassDeclaration".to_string(), token_value: None, children: vec![        structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("C".to_string()), children: vec![] },         structure_assert::ExpectedNode { kind: "MethodDeclaration".to_string(), token_value: None, children: vec![            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },             structure_assert::ExpectedNode { kind: "Block".to_string(), token_value: None, children: vec![                structure_assert::ExpectedNode { kind: "SwitchStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "ObjectCreationExpression".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                         structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] },                 structure_assert::ExpectedNode { kind: "LocalFunctionStatement".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ParameterList".to_string(), token_value: None, children: vec![] },                     structure_assert::ExpectedNode { kind: "ArrowExpressionClause".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "IsPatternExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("o".to_string()), children: vec![] },                             structure_assert::ExpectedNode { kind: "ConstantPattern".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "NumericLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] },                 structure_assert::ExpectedNode { kind: "SwitchStatement".to_string(), token_value: None, children: vec![] },                 structure_assert::ExpectedNode { kind: "SwitchSection".to_string(), token_value: None, children: vec![                    structure_assert::ExpectedNode { kind: "CasePatternSwitchLabel".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "TypePattern".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "PredefinedType".to_string(), token_value: None, children: vec![] }] },                         structure_assert::ExpectedNode { kind: "WhenClause".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("SomeTest".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![] }] }] }] },                     structure_assert::ExpectedNode { kind: "ExpressionStatement".to_string(), token_value: None, children: vec![                        structure_assert::ExpectedNode { kind: "InvocationExpression".to_string(), token_value: None, children: vec![                            structure_assert::ExpectedNode { kind: "SimpleMemberAccessExpression".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("Console".to_string()), children: vec![] },                                 structure_assert::ExpectedNode { kind: "IdentifierName".to_string(), token_value: Some("WriteLine".to_string()), children: vec![] }] },                             structure_assert::ExpectedNode { kind: "ArgumentList".to_string(), token_value: None, children: vec![                                structure_assert::ExpectedNode { kind: "Argument".to_string(), token_value: None, children: vec![                                    structure_assert::ExpectedNode { kind: "StringLiteralExpression".to_string(), token_value: None, children: vec![] }] }] }] }] },                     structure_assert::ExpectedNode { kind: "BreakStatement".to_string(), token_value: None, children: vec![] }] }] }] }] }] } };
        structure_assert::assert_tree(&expected, &unit);
    }
}
