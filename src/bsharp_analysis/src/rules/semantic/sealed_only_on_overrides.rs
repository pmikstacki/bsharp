use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    SealedOnlyOnOverrides: "semantic.members.sealed_only_on_overrides", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    match member {
                        ClassBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Sealed) && !m.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE04009, at m);
                            }
                        }
                        ClassBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Sealed) && !p.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE04009, at p);
                            }
                        }
                        ClassBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Sealed) && !e.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE04009, at e);
                            }
                        }
                        ClassBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Sealed) && !i.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE04009, at i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
