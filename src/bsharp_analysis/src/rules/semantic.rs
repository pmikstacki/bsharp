use crate::framework::{AnalysisSession, Rule, RuleSet};
use crate::framework::NodeRef;
use crate::syntax::ast::TopLevelDeclaration;
use crate::{diag, DiagnosticCode, rule, ruleset};
use bsharp_syntax::declarations::{ClassBodyDeclaration, Modifier};
use bsharp_syntax::types::{PrimitiveType, Type};

fn ident_text(id: &crate::syntax::Identifier) -> String {
    match id {
        crate::syntax::Identifier::Simple(s) => s.clone(),
        crate::syntax::Identifier::QualifiedIdentifier(parts) => parts.join("."),
        crate::syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

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
    },
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
    },
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
    },
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
    },
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
    },
    AsyncReturnsTask: "semantic.async.returns_task_or_task_t", "Semantic", {
        let Some(cu) = node.of::<crate::syntax::ast::CompilationUnit>() else {
            return;
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member
                        && m.modifiers.contains(&Modifier::Async)
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

ruleset! {
    semantic: CtorNoAsync, CtorNameMatchesClass, CtorNoVirtualOrAbstract, MethodNoAbstractBody, MethodNoStaticOverride, AsyncReturnsTask
}
