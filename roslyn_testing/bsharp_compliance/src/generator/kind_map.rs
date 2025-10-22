#[derive(Debug, Clone)]
pub struct MatchSpec {
    pub roslyn_kind: String,
    pub our_kind: String,
    pub token_field: Option<String>,
}

#[allow(dead_code)]
pub fn map_kind(roslyn_kind: &str) -> Option<MatchSpec> {
    // Minimal initial mapping for pilot; expand incrementally.
    match roslyn_kind {
        "CompilationUnit" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "CompilationUnit".into(),
            token_field: None,
        }),
        // Declarations
        "NamespaceDeclaration" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "NamespaceDeclaration".into(),
            token_field: None,
        }),
        "FileScopedNamespaceDeclaration" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "FileScopedNamespaceDeclaration".into(),
            token_field: None,
        }),
        "ClassDeclaration" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "ClassDeclaration".into(),
            token_field: None,
        }),
        "RecordStructDeclaration" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "RecordDeclaration".into(),
            token_field: None,
        }),
        // Type parameters and constraints
        "TypeParameterList" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "TypeParameterList".into(),
            token_field: None,
        }),
        "TypeParameter" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "TypeParameter".into(),
            token_field: None,
        }),
        "TypeParameterConstraintClause" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "TypeParameterConstraintClause".into(),
            token_field: None,
        }),
        // Constraint kinds mapping to our enum variants
        "ClassConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "ReferenceType".into(),
            token_field: None,
        }),
        "StructConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "ValueType".into(),
            token_field: None,
        }),
        "UnmanagedConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "Unmanaged".into(),
            token_field: None,
        }),
        "NotNullConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "NotNull".into(),
            token_field: None,
        }),
        "ConstructorConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "Constructor".into(),
            token_field: None,
        }),
        "TypeConstraint" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "SpecificType".into(),
            token_field: None,
        }),
        // Allows family (Roslyn specific) -> our canonical variant
        "AllowsConstraintClause" => Some(MatchSpec {
            roslyn_kind: roslyn_kind.into(),
            our_kind: "AllowsRefStruct".into(),
            token_field: None,
        }),
        _ => None,
    }
}

/// Translate a Roslyn-structured ExpectedTree into our canonical ExpectedTree
/// - Strips token/keyword nodes (e.g., IdentifierToken, ClassKeyword, OpenBraceToken)
/// - Lifts identifier token text into parent IdentifierName when present
/// - Applies simple renames (e.g., drop any trailing "Syntax" if encountered)
pub fn translate_expected_tree(
    t: &crate::generator::structure_dsl::ExpectedTree,
) -> crate::generator::structure_dsl::ExpectedTree {
    use crate::generator::structure_dsl::{ExpectedNode, ExpectedTree};
    fn is_trivia_token(kind: &str) -> bool {
        kind.ends_with("Token") || kind.ends_with("Keyword")
    }

    fn rename_kind(kind: &str) -> String {
        // Drop any accidental Roslyn suffixes if present
        if let Some(stripped) = kind.strip_suffix("Syntax") {
            return stripped.to_string();
        }
        kind.to_string()
    }

    fn translate_node(n: &ExpectedNode) -> Option<ExpectedNode> {
        let kind0 = rename_kind(&n.kind);

        // Filter out pure token nodes
        if is_trivia_token(&kind0) {
            return None;
        }

        // Translate children first
        let mut new_children: Vec<ExpectedNode> = Vec::new();
        let mut lifted_token: Option<String> = None;
        for ch in &n.children {
            if ch.kind == "IdentifierToken" {
                if let Some(tv) = ch.token_value.as_ref() {
                    lifted_token = Some(tv.clone());
                }
                continue; // drop token child
            }
            if let Some(tch) = translate_node(ch) {
                new_children.push(tch);
            }
        }

        let mut token_value = n.token_value.clone();
        if kind0 == "IdentifierName" {
            // Lift identifier text if available
            if token_value.is_none() {
                token_value = lifted_token.clone();
            }
        }

        // Apply mapping table where available
        let our_kind = map_kind(&kind0)
            .map(|m| m.our_kind)
            .unwrap_or_else(|| kind0.clone());

        // Enrich certain nodes with synthesized children
        let mut out_children = new_children;
        if our_kind == "ClassDeclaration" {
            if let Some(tv) = lifted_token.clone() {
                out_children.insert(
                    0,
                    ExpectedNode {
                        kind: "IdentifierName".to_string(),
                        token_value: Some(tv),
                        children: vec![],
                    },
                );
            }
        } else if our_kind == "TypeParameter" {
            if let Some(tv) = lifted_token.clone() {
                out_children.push(ExpectedNode {
                    kind: "IdentifierName".to_string(),
                    token_value: Some(tv),
                    children: vec![],
                });
            }
        } else if our_kind == "SpecificType" {
            if let Some(tv) = lifted_token.clone() {
                out_children.push(ExpectedNode {
                    kind: "IdentifierName".to_string(),
                    token_value: Some(tv),
                    children: vec![],
                });
            }
        } else if our_kind == "AllowsRefStruct" {
            // Flattened marker, no children
            out_children.clear();
        }

        Some(ExpectedNode {
            kind: our_kind,
            token_value,
            children: out_children,
        })
    }

    let mut root = translate_node(&t.root).unwrap_or_else(|| t.root.clone());

    // Normalize Roslyn harness: ClassDeclaration -> MethodDeclaration -> Block(statements)
    // into CompilationUnit-level GlobalStatement nodes to match our parser's top-level statements model.
    if root.kind == "CompilationUnit" {
        let mut new_children: Vec<ExpectedNode> = Vec::new();
        for ch in root.children.into_iter() {
            if ch.kind == "ClassDeclaration" {
                // Try to find MethodDeclaration -> Block
                let mut stmt_count = 0usize;
                for md in &ch.children {
                    if md.kind == "MethodDeclaration" {
                        if let Some(block) = md.children.iter().find(|c| c.kind == "Block") {
                            stmt_count = block.children.len();
                            break;
                        }
                    }
                }
                if stmt_count == 0 {
                    stmt_count = 1;
                }
                for _ in 0..stmt_count {
                    new_children.push(ExpectedNode {
                        kind: "GlobalStatement".to_string(),
                        token_value: None,
                        children: vec![],
                    });
                }
                continue;
            }
            new_children.push(ch);
        }
        root.children = new_children;
    }

    crate::generator::structure_dsl::ExpectedTree { root }
}
