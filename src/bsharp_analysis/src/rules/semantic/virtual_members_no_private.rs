use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    VirtualMembersNoPrivate: "semantic.members.virtual_no_private", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    match member {
                        ClassBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Virtual) && m.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04007, at m);
                            }
                        }
                        ClassBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Virtual) && p.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04007, at p);
                            }
                        }
                        ClassBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Virtual) && e.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04007, at e);
                            }
                        }
                        ClassBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Virtual) && i.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04007, at i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
