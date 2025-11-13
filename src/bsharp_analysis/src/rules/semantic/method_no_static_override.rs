use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    MethodNoStaticOverride: "semantic.method.no_static_override", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member
                        && m.modifiers.contains(&Modifier::Static)
                        && m.modifiers.contains(&Modifier::Override)
                    {
                        diag!(session, DiagnosticCode::BSE02006, at m);
                    }
                }
            }
        }
    }
}
