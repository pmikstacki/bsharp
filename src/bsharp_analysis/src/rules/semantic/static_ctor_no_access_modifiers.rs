use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    StaticCtorNoAccessModifiers: "semantic.ctor.static_no_access_modifiers", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        if ctor.modifiers.contains(&Modifier::Static) {
                            let has_access = ctor.modifiers.iter().any(|m| matches!(m, Modifier::Public | Modifier::Private | Modifier::Protected | Modifier::Internal | Modifier::File));
                            if has_access {
                                diag!(session, DiagnosticCode::BSE04005, at ctor);
                            }
                        }
                    }
                }
            }
        }
    }
}
