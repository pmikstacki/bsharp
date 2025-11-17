use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    AbstractMembersOnlyInAbstractClass: "semantic.members.abstract_only_in_abstract_class", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                let class_is_abstract = class.modifiers.contains(&Modifier::Abstract);
                if class_is_abstract { continue; }

                for member in &class.body_declarations {
                    match member {
                        ClassBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Abstract) {
                                diag!(session, DiagnosticCode::BSE04010, at m);
                            }
                        }
                        ClassBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Abstract) {
                                diag!(session, DiagnosticCode::BSE04010, at p);
                            }
                        }
                        ClassBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Abstract) {
                                diag!(session, DiagnosticCode::BSE04010, at e);
                            }
                        }
                        ClassBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Abstract) {
                                diag!(session, DiagnosticCode::BSE04010, at i);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
