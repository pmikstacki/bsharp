use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, StructBodyDeclaration, Modifier};

rule! {
    CtorsNoOverride: "semantic.ctor.no_override", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Class(class) => {
                    for member in &class.body_declarations {
                        if let ClassBodyDeclaration::Constructor(c) = member {
                            if c.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE01009, at c);
                            }
                        }
                    }
                }
                TopLevelDeclaration::Struct(s) => {
                    for member in &s.body_declarations {
                        if let StructBodyDeclaration::Constructor(c) = member {
                            if c.modifiers.contains(&Modifier::Override) {
                                diag!(session, DiagnosticCode::BSE01009, at c);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
