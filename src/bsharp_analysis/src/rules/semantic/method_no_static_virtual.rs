use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    MethodNoStaticVirtual: "semantic.method.no_static_virtual", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else { return; };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member {
                        if m.modifiers.contains(&Modifier::Static) && m.modifiers.contains(&Modifier::Virtual) {
                            diag!(session, DiagnosticCode::BSE02005, at m);
                        }
                    }
                }
            }
        }
    }
}
