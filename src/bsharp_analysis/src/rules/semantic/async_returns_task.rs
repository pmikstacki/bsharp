use crate::{diag, DiagnosticCode, rule};
use crate::syntax::ast::TopLevelDeclaration;
use bsharp_syntax::declarations::ClassBodyDeclaration;
use bsharp_syntax::types::{PrimitiveType, Type};
use super::utils::ident_text;

rule! {
    AsyncReturnsTask: "semantic.async.returns_task_or_task_t", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member
                        && m.modifiers.contains(&bsharp_syntax::declarations::Modifier::Async)
                    {
                        let valid = match &m.return_type {
                            Type::Reference(rt) => ident_text(rt) == "Task",
                            Type::Generic { base, .. } => ident_text(base) == "Task",
                            Type::Primitive(PrimitiveType::Void) => true, // allowed but discouraged
                            _ => false,
                        };
                        if !valid {
                            diag!(session, DiagnosticCode::BSE02009, at m);
                        }
                    }
                }
            }
        }
    }
}
