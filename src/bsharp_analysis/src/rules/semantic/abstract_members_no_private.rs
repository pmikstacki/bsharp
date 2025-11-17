use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    AbstractMembersNoPrivate: "semantic.members.abstract_no_private", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    match member {
                        ClassBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Abstract) && m.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04006, at m);
                            }
                        }
                        ClassBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Abstract) && p.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04006, at p);
                            }
                        }
                        ClassBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Abstract) && e.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04006, at e);
                            }
                        }
                        ClassBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Abstract) && i.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04006, at i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
