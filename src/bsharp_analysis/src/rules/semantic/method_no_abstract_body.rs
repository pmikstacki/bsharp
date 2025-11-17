use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};

rule! {
    MethodNoAbstractBody: "semantic.method.no_abstract_body", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member
                        && m.modifiers.contains(&Modifier::Abstract)
                        && m.body.is_some()
                    {
                        diag!(session, DiagnosticCode::BSE02001, at m);
                    }
                }
            }
        }
    }
}
