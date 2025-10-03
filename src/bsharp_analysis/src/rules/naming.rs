use crate::framework::{AnalysisSession, NodeRef, Rule, RuleSet};
use crate::syntax::ast::TopLevelDeclaration;
use crate::{DiagnosticBuilder, DiagnosticCode};

fn is_pascal_case(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_uppercase() => {}
        _ => return false,
    }
    // Don't allow underscores for basic style rule
    !name.contains('_')
}

fn is_interface_ipascal_case(name: &str) -> bool {
    let mut chars = name.chars();
    match (chars.next(), chars.next()) {
        (Some('I'), Some(c2)) if c2.is_uppercase() => {}
        _ => return false,
    }
    !name.contains('_')
}

fn find_span_for_decl(session: &AnalysisSession, kind: &str, name: &str) -> Option<(usize, usize)> {
    // Try exact key first: kind::Name
    let key_exact = format!("{}::{}", kind, name);
    if let Some(range) = session.spans.get(&key_exact) {
        return Some((range.start, range.end - range.start));
    }

    // Try namespaced form: kind::<ns>::Name by scanning keys
    let suffix = format!("::{}", name);
    let prefix = format!("{}::", kind);
    for (k, range) in session.spans.iter() {
        if k.starts_with(&prefix) && k.ends_with(&suffix) {
            return Some((range.start, range.end - range.start));
        }
    }
    None
}

struct ClassPascalCase;
impl Rule for ClassPascalCase {
    fn id(&self) -> &'static str {
        "naming.class_pascal_case"
    }
    fn category(&self) -> &'static str {
        "Naming"
    }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node {
            NodeRef::CompilationUnit(cu) => cu,
            _ => return,
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                let name = c.name.name.as_str();
                if !is_pascal_case(name) {
                    let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                        .with_message(format!("Type '{}' should be PascalCase", name));
                    if let Some((start, len)) = find_span_for_decl(session, "class", name) {
                        b = b.at_span(session, start, len);
                    }
                    b.emit(session);
                }
            }
        }
    }
}

struct InterfaceIPascalCase;
impl Rule for InterfaceIPascalCase {
    fn id(&self) -> &'static str {
        "naming.interface_ipascal_case"
    }
    fn category(&self) -> &'static str {
        "Naming"
    }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node {
            NodeRef::CompilationUnit(cu) => cu,
            _ => return,
        };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Interface(i) = decl {
                let name = i.name.name.as_str();
                if !is_interface_ipascal_case(name) {
                    let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                        .with_message(format!("Interface '{}' should follow I* PascalCase", name));
                    if let Some((start, len)) = find_span_for_decl(session, "interface", name) {
                        b = b.at_span(session, start, len);
                    }
                    b.emit(session);
                }
            }
        }
    }
}

pub fn ruleset() -> RuleSet {
    RuleSet::new("naming")
        .with_rule(ClassPascalCase)
        .with_rule(InterfaceIPascalCase)
}
