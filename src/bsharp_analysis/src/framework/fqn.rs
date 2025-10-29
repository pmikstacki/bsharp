use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::declarations::{
    ClassBodyDeclaration, ClassDeclaration, MethodDeclaration, NamespaceDeclaration,
    namespace_declaration::NamespaceBodyDeclaration,
};

fn ident_text(id: &crate::syntax::Identifier) -> String {
    match id {
        crate::syntax::Identifier::Simple(s) => s.clone(),
        crate::syntax::Identifier::QualifiedIdentifier(parts) => parts.join("."),
        crate::syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

pub fn method_fqn(cu: &CompilationUnit, method: &MethodDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace
        && let Some((cfqn, name)) = find_in_namespace(None, &fs.declarations, method)
    {
        return format!("{}::{}", cfqn, name);
    }
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                let ns_path = ident_text(&ns.name);
                if let Some((cfqn, name)) =
                    find_in_namespace(Some(&ns_path), &ns.declarations, method)
                {
                    return format!("{}::{}", cfqn, name);
                }
            }
            TopLevelDeclaration::Class(c) => {
                if let Some((cfqn, name)) = find_in_class(None, c, method, &mut Vec::new()) {
                    return format!("{}::{}", cfqn, name);
                }
            }
            _ => {}
        }
    }
    ident_text(&method.name)
}

fn find_in_namespace(
    ns_path: Option<&str>,
    members: &[NamespaceBodyDeclaration],
    method: &MethodDeclaration,
) -> Option<(String, String)> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path {
                    Some(p) => format!("{}.{}", p, ident_text(&inner.name)),
                    None => ident_text(&inner.name),
                };
                if let Some((cfqn, name)) = find_in_namespace(Some(&new_ns), &inner.declarations, method) {
                    return Some((cfqn, name));
                }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some((cfqn, name)) = find_in_class(ns_path, class, method, &mut Vec::new()) {
                    return Some((cfqn, name));
                }
            }
            _ => {}
        }
    }
    None
}

fn find_in_class(
    ns_path: Option<&str>,
    class: &ClassDeclaration,
    method: &MethodDeclaration,
    stack: &mut Vec<String>,
) -> Option<(String, String)> {
    stack.push(ident_text(&class.name));
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Method(m) => {
                if std::ptr::eq(m, method) {
                    let class_path = stack.join(".");
                    let cfqn = match ns_path {
                        Some(ns) => format!("{}.{}", ns, class_path),
                        None => class_path,
                    };
                    let name = ident_text(&method.name);
                    stack.pop();
                    return Some((cfqn, name));
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                if let Some(found) = find_in_class(ns_path, nested, method, stack) {
                    stack.pop();
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    stack.pop();
    None
}

pub fn class_fqn(cu: &CompilationUnit, class: &ClassDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace
        && let Some(cfqn) = find_class_in_namespace(None, &fs.declarations, class, &mut Vec::new())
    {
        return cfqn;
    }
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                let ns_path = ident_text(&ns.name);
                if let Some(cfqn) = find_class_in_namespace(
                    Some(&ns_path),
                    &ns.declarations,
                    class,
                    &mut Vec::new(),
                ) {
                    return cfqn;
                }
            }
            TopLevelDeclaration::Class(c) => {
                if let Some(cfqn) = find_class_path(None, c, class, &mut Vec::new()) {
                    return cfqn;
                }
            }
            _ => {}
        }
    }
    ident_text(&class.name)
}

fn find_class_in_namespace(
    ns_path: Option<&str>,
    members: &[NamespaceBodyDeclaration],
    target: &ClassDeclaration,
    stack: &mut Vec<String>,
) -> Option<String> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path {
                    Some(p) => format!("{}.{}", p, ident_text(&inner.name)),
                    None => ident_text(&inner.name),
                };
                if let Some(cfqn) = find_class_in_namespace(Some(&new_ns), &inner.declarations, target, stack) {
                    return Some(cfqn);
                }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some(cfqn) = find_class_path(ns_path, class, target, stack) {
                    return Some(cfqn);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_class_path(
    ns_path: Option<&str>,
    class: &ClassDeclaration,
    target: &ClassDeclaration,
    stack: &mut Vec<String>,
) -> Option<String> {
    stack.push(ident_text(&class.name));
    for member in &class.body_declarations {
        if let ClassBodyDeclaration::NestedClass(nested) = member
            && let Some(path) = find_class_path(ns_path, nested, target, stack)
        {
            stack.pop();
            return Some(path);
        }
    }
    let class_path = stack.join(".");
    if std::ptr::eq(class, target) {
        let cfqn = match ns_path {
            Some(ns) => format!("{}.{}", ns, class_path),
            None => class_path,
        };
        stack.pop();
        return Some(cfqn);
    }
    stack.pop();
    None
}

pub fn namespace_fqn(cu: &CompilationUnit, ns: &NamespaceDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace
        && let Some(path) = find_namespace_path(None, &fs.declarations, ns)
    {
        return path;
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(top) = decl {
            let top_seg = ident_text(&top.name);
            if std::ptr::eq(top, ns) {
                return top_seg;
            }
            if let Some(path) = find_namespace_path(Some(&top_seg), &top.declarations, ns) {
                return path;
            }
        }
    }
    ident_text(&ns.name)
}

fn find_namespace_path(
    prefix: Option<&str>,
    members: &[NamespaceBodyDeclaration],
    target: &NamespaceDeclaration,
) -> Option<String> {
    for m in members {
        if let NamespaceBodyDeclaration::Namespace(inner) = m {
            let seg = ident_text(&inner.name);
            if std::ptr::eq(inner, target) {
                return Some(match prefix {
                    Some(p) => format!("{}.{}", p, seg),
                    None => seg,
                });
            }
            let next = match prefix {
                Some(p) => format!("{}.{}", p, seg),
                None => seg,
            };
            if let Some(path) = find_namespace_path(Some(&next), &inner.declarations, target) {
                return Some(path);
            }
        }
    }
    None
}
