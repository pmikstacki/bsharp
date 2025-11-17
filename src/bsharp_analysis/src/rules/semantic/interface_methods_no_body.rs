use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::InterfaceBodyDeclaration;

rule! {
    InterfaceMethodsNoBody: "semantic.interface.methods_no_body", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Interface(interface) = decl {
                for member in &interface.body_declarations {
                    if let InterfaceBodyDeclaration::Method(m) = member {
                        if m.body.is_some() {
                            diag!(session, DiagnosticCode::BSE02008, at m);
                        }
                    }
                }
            }
        }
    }
}
