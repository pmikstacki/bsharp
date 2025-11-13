use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    CtorNoAsync: "semantic.ctor.no_async", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member
                        && ctor.modifiers.contains(&Modifier::Async)
                    {
                        diag!(session, DiagnosticCode::BSE01001, at ctor);
                    }
                }
            }
        }
    }
}
