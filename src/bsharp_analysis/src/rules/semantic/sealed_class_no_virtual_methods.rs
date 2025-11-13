use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    SealedClassNoVirtualMethods: "semantic.class.sealed_no_virtual_methods", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                let is_sealed = class.modifiers.contains(&Modifier::Sealed);
                if !is_sealed { continue; }

                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member {
                        if m.modifiers.contains(&Modifier::Virtual) {
                            diag!(session, DiagnosticCode::BSE02003, at m);
                        }
                    }
                }
            }
        }
    }
}
