use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    CtorNoVirtualOrAbstract: "semantic.ctor.no_virtual_or_abstract", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member
                        && (ctor.modifiers.contains(&Modifier::Virtual)
                            || ctor.modifiers.contains(&Modifier::Abstract))
                    {
                        diag!(session, DiagnosticCode::BSE01003, at ctor);
                    }
                }
            }
        }
    }
}
