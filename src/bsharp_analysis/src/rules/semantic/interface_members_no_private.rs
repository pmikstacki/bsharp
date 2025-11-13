use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{InterfaceBodyDeclaration, Modifier};

rule! {
    InterfaceMembersNoPrivate: "semantic.interface.members_no_private", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Interface(interface) = decl {
                for member in &interface.body_declarations {
                    match member {
                        InterfaceBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04002, at m);
                            }
                        }
                        InterfaceBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04002, at p);
                            }
                        }
                        InterfaceBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04002, at e);
                            }
                        }
                        InterfaceBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Private) {
                                diag!(session, DiagnosticCode::BSE04002, at i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
