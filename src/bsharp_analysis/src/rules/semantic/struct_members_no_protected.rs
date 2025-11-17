use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{StructBodyDeclaration, Modifier};

rule! {
    StructMembersNoProtected: "semantic.struct.members_no_protected", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Struct(s) = decl {
                for member in &s.body_declarations {
                    match member {
                        StructBodyDeclaration::Field(f) => {
                            if f.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at f);
                            }
                        }
                        StructBodyDeclaration::Method(m) => {
                            if m.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at m);
                            }
                        }
                        StructBodyDeclaration::Property(p) => {
                            if p.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at p);
                            }
                        }
                        StructBodyDeclaration::Constructor(c) => {
                            if c.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at c);
                            }
                        }
                        StructBodyDeclaration::Event(e) => {
                            if e.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at e);
                            }
                        }
                        StructBodyDeclaration::Indexer(i) => {
                            if i.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at i);
                            }
                        }
                        StructBodyDeclaration::Operator(op) => {
                            if op.modifiers.contains(&Modifier::Protected) {
                                diag!(session, DiagnosticCode::BSE04003, at op);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
