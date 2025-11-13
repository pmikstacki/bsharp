use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::ClassBodyDeclaration;
use super::utils::ident_text;

rule! {
    CtorNameMatchesClass: "semantic.ctor.name_matches_class", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        let ctor_name = ident_text(&ctor.name);
                        let class_name = ident_text(&class.name);
                        if ctor_name != class_name {
                            diag!(
                                session,
                                DiagnosticCode::BSE01005,
                                at ctor,
                                msg: format!(
                                    "Constructor name '{}' does not match class name '{}'",
                                    ctor_name, class_name
                                )
                            );
                        }
                    }
                }
            }
        }
    }
}
