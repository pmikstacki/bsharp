use crate::framework::{AnalysisSession, NodeRef, Rule, RuleSet};
use crate::syntax::ast::TopLevelDeclaration;
use crate::{DiagnosticBuilder, DiagnosticCode};
use bsharp_syntax::declarations::ClassBodyDeclaration;

fn is_pascal_case(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_uppercase() => {}
        _ => return false,
    }
    // Don't allow underscores for basic style rule
    !name.contains('_')
}

struct PropertyPascalCase;
impl Rule for PropertyPascalCase {
    fn id(&self) -> &'static str { "naming.property_pascal_case" }
    fn category(&self) -> &'static str { "Naming" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu, _ => return };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Property(p) = m {
                        let name = p.name.name.as_str();
                        if !is_pascal_case(name) {
                            let b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                .with_message(format!("Property '{}' should be PascalCase", name));
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct FieldCamelOrConstUpper;
impl Rule for FieldCamelOrConstUpper {
    fn id(&self) -> &'static str { "naming.field_camel_or_const_upper" }
    fn category(&self) -> &'static str { "Naming" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu, _ => return };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Field(f) = m {
                        let name = f.name.name.as_str();
                        let is_const = f.modifiers.iter().any(|m| matches!(m, bsharp_syntax::declarations::Modifier::Const));
                        if is_const {
                            if !is_upper_case_constant(name) {
                                let b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                    .with_message(format!("Constant '{}' should be UPPER_CASE", name));
                                b.emit(session);
                            }
                        } else if !is_camel_case(name) {
                            let b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                .with_message(format!("Field '{}' should be camelCase", name));
                            b.emit(session);
                        }
                    }
                }
            }
        }
    }
}

struct ParameterCamelCase;
impl Rule for ParameterCamelCase {
    fn id(&self) -> &'static str { "naming.parameter_camel_case" }
    fn category(&self) -> &'static str { "Naming" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu, _ => return };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                for m in &c.body_declarations {
                    match m {
                        ClassBodyDeclaration::Method(md) => {
                            for p in &md.parameters {
                                let name = p.name.name.as_str();
                                if !is_camel_case(name) {
                                    let b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                        .with_message(format!("Parameter '{}' should be camelCase", name));
                                    b.emit(session);
                                }
                            }
                        }
                        ClassBodyDeclaration::Constructor(ctor) => {
                            for p in &ctor.parameters {
                                let name = p.name.name.as_str();
                                if !is_camel_case(name) {
                                    let b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                        .with_message(format!("Parameter '{}' should be camelCase", name));
                                    b.emit(session);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn is_camel_case(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_lowercase() => {}
        _ => return false,
    }
    !name.contains('_')
}

fn is_upper_case_constant(name: &str) -> bool {
    if name.is_empty() { return false; }
    name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn find_method_span(session: &AnalysisSession, class_name: &str, method_name: &str) -> Option<(usize, usize)> {
    let simple_class = class_name.rsplit('.').next().unwrap_or(class_name);
    for (k, range) in session.spans.iter() {
        if k.starts_with("method::") && k.ends_with(&format!("::{}::{}", simple_class, method_name)) {
            return Some((range.start, range.end - range.start));
        }
    }
    None
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
        .with_rule(MethodPascalCase)
        .with_rule(PropertyPascalCase)
        .with_rule(FieldCamelOrConstUpper)
        .with_rule(ParameterCamelCase)
}

struct MethodPascalCase;
impl Rule for MethodPascalCase {
    fn id(&self) -> &'static str { "naming.method_pascal_case" }
    fn category(&self) -> &'static str { "Naming" }
    fn visit(&self, node: &NodeRef, session: &mut AnalysisSession) {
        let cu = match node { NodeRef::CompilationUnit(cu) => cu, _ => return };
        for decl in &cu.declarations {
            if let TopLevelDeclaration::Class(c) = decl {
                let class_name = &c.name.name;
                for m in &c.body_declarations {
                    if let ClassBodyDeclaration::Method(md) = m {
                        let name = md.name.name.as_str();
                        if !is_pascal_case(name) {
                            let mut b = DiagnosticBuilder::new(DiagnosticCode::BSW02002)
                                .with_message(format!("Method '{}' should be PascalCase", name));
                            if let Some((start, len)) = find_method_span(session, class_name, name) {
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
