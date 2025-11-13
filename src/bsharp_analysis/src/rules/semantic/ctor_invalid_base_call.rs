use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::{ClassBodyDeclaration, ConstructorInitializer};

rule! {
    CtorInvalidBaseCall: "semantic.ctor.invalid_base_call", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        if let Some(ConstructorInitializer::Base(_)) = &ctor.initializer {
                            if class.base_types.is_empty() {
                                diag!(session, DiagnosticCode::BSE01007, at ctor);
                            }
                        }
                    }
                }
            }
        }
    }
}
