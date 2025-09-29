use crate::analysis::framework::diagnostic_builder::DiagnosticBuilder;
use crate::analysis::framework::rules::{Rule, RuleSet};
use crate::analysis::framework::session::AnalysisSession;
use crate::analysis::framework::walker::NodeRef;
use crate::analysis::diagnostics::diagnostic_code::DiagnosticCode;
use crate::syntax::ast::TopLevelDeclaration;
use crate::syntax::nodes::declarations::{ClassBodyDeclaration, Modifier};
use crate::syntax::nodes::types::{PrimitiveType, Type};

fn find_ctor_span(session: &AnalysisSession, class_name: &str) -> Option<(usize, usize)> {
    for (k, range) in session.spans.iter() {
        if k.starts_with("ctor::") && k.ends_with(&format!("::{}", class_name)) {
            return Some((range.start, range.end - range.start));
        }
    }
    None
}

fn find_method_span(session: &AnalysisSession, class_name: &str, method_name: &str) -> Option<(usize, usize)> {
    for (k, range) in session.spans.iter() {
        if k.starts_with("method::") && k.ends_with(&format!("::{}::{}", class_name, method_name)) {
            return Some((range.start, range.end - range.start));
        }
    }
    None
}

struct CtorNoAsync;
impl Rule for CtorNoAsync {
    fn id(&self) -> &'static str { "semantic.ctor.no_async" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        if ctor.modifiers.contains(&Modifier::Async) {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE01001);
                            if let Some((start, len)) = find_ctor_span(session, &class.name.name) {
                                b = b.at_span(session, start, len);
                            }
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct CtorNameMatchesClass;
impl Rule for CtorNameMatchesClass {
    fn id(&self) -> &'static str { "semantic.ctor.name_matches_class" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        if ctor.name.name != class.name.name {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE01005)
                                .with_message(format!(
                                    "Constructor name '{}' does not match class name '{}'",
                                    ctor.name.name, class.name.name
                                ));
                            if let Some((start, len)) = find_ctor_span(session, &class.name.name) {
                                b = b.at_span(session, start, len);
                            }
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct CtorNoVirtualOrAbstract;
impl Rule for CtorNoVirtualOrAbstract {
    fn id(&self) -> &'static str { "semantic.ctor.no_virtual_or_abstract" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Constructor(ctor) = member {
                        if ctor.modifiers.contains(&Modifier::Virtual) || ctor.modifiers.contains(&Modifier::Abstract) {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE01003);
                            if let Some((start, len)) = find_ctor_span(session, &class.name.name) {
                                b = b.at_span(session, start, len);
                            }
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct MethodNoAbstractBody;
impl Rule for MethodNoAbstractBody {
    fn id(&self) -> &'static str { "semantic.method.no_abstract_body" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member {
                        if m.modifiers.contains(&Modifier::Abstract) && m.body.is_some() {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE02001);
                            if let Some((start, len)) = find_method_span(session, &class.name.name, &m.name.name) {
                                b = b.at_span(session, start, len);
                            }
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct MethodNoStaticOverride;
impl Rule for MethodNoStaticOverride {
    fn id(&self) -> &'static str { "semantic.method.no_static_override" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member {
                        if m.modifiers.contains(&Modifier::Static) && m.modifiers.contains(&Modifier::Override) {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE02006);
                            if let Some((start, len)) = find_method_span(session, &class.name.name, &m.name.name) {
                                b = b.at_span(session, start, len);
                            }
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct AsyncReturnsTask;
impl Rule for AsyncReturnsTask {
    fn id(&self) -> &'static str { "semantic.async.returns_task_or_task_t" }
    fn category(&self) -> &'static str { "Semantic" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(class) = decl {
                for member in &class.body_declarations {
                    if let ClassBodyDeclaration::Method(m) = member {
                        if m.modifiers.contains(&Modifier::Async) {
                            let valid = match &m.return_type {
                                Type::Reference(rt) => rt.name == "Task",
                                Type::Generic { base, .. } => base.name == "Task",
                                Type::Primitive(PrimitiveType::Void) => true, // allowed but discouraged
                                _ => false,
                            };
                            if !valid {
                                let mut b = DiagnosticBuilder::new(DiagnosticCode::BSE02009);
                                if let Some((start, len)) = find_method_span(session, &class.name.name, &m.name.name) {
                                    b = b.at_span(session, start, len);
                                }
                                b.emit(session);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn ruleset() -> RuleSet {
    RuleSet::new("semantic")
        .with_rule(CtorNoAsync)
        .with_rule(CtorNameMatchesClass)
        .with_rule(CtorNoVirtualOrAbstract)
        .with_rule(MethodNoAbstractBody)
        .with_rule(MethodNoStaticOverride)
        .with_rule(AsyncReturnsTask)
}
