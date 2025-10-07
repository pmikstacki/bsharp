use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{ClassBodyDeclaration, ClassDeclaration, MethodDeclaration, NamespaceDeclaration, namespace_declaration::NamespaceBodyDeclaration};

/// Compute the fully-qualified name for a method: "<ns>.<Class[.Nested]>::<Method>".
/// Falls back to method name when no containing path can be determined.
pub fn method_fqn(cu: &CompilationUnit, method: &MethodDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some((cfqn, name)) = find_in_namespace(None, &fs.declarations, method) {
            return format!("{}::{}", cfqn, name);
        }

/// Fully-qualified name for a class including nested classes and containing namespace path.
pub fn class_fqn(cu: &CompilationUnit, class: &ClassDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some(cfqn) = find_class_in_namespace(None, &fs.declarations, class, &mut Vec::new()) {
            return cfqn;
        }
    }
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                let ns_path = ns.name.name.clone();
                if let Some(cfqn) = find_class_in_namespace(Some(&ns_path), &ns.declarations, class, &mut Vec::new()) {
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
    class.name.name.clone()
}

/// Fully-qualified namespace path for a namespace declaration.
pub fn namespace_fqn(cu: &CompilationUnit, ns: &NamespaceDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some(path) = find_namespace_path(None, &fs.declarations, ns) {
            return path;
        }
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(top) = decl {
            let top_seg = top.name.name.clone();
            if std::ptr::eq(top, ns) {
                return top_seg;
            }
            if let Some(path) = find_namespace_path(Some(&top_seg), &top.declarations, ns) {
                return path;
            }
        }
    }
    ns.name.name.clone()
}

fn find_namespace_path<'a>(prefix: Option<&str>, members: &'a [NamespaceBodyDeclaration], target: &NamespaceDeclaration) -> Option<String> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let seg = inner.name.name.clone();
                if std::ptr::eq(inner, target) {
                    return Some(match prefix { Some(p) => format!("{}.{}", p, seg), None => seg });
                }
                let next = match prefix { Some(p) => format!("{}.{}", p, seg), None => seg };
                if let Some(path) = find_namespace_path(Some(&next), &inner.declarations, target) {
                    return Some(path);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_class_in_namespace<'a>(ns_path: Option<&str>, members: &'a [NamespaceBodyDeclaration], target: &ClassDeclaration, stack: &mut Vec<String>) -> Option<String> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path { Some(p) => format!("{}.{}", p, inner.name.name), None => inner.name.name.clone() };
                if let Some(cfqn) = find_class_in_namespace(Some(&new_ns), &inner.declarations, target, stack) { return Some(cfqn); }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some(cfqn) = find_class_path(ns_path, class, target, stack) { return Some(cfqn); }
            }
            _ => {}
        }
    }
    None
}

fn find_class_path(ns_path: Option<&str>, class: &ClassDeclaration, target: &ClassDeclaration, stack: &mut Vec<String>) -> Option<String> {
    stack.push(class.name.name.clone());
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::NestedClass(nested) => {
                if let Some(path) = find_class_path(ns_path, nested, target, stack) { stack.pop(); return Some(path); }
            }
            _ => {}
        }
    }
    let class_path = stack.join(".");
    if std::ptr::eq(class, target) {
        let cfqn = match ns_path { Some(ns) => format!("{}.{}", ns, class_path), None => class_path };
        stack.pop();
        return Some(cfqn);
    }
    stack.pop();
    None
}
    }
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                let ns_path = ns.name.name.clone();
                if let Some((cfqn, name)) = find_in_namespace(Some(&ns_path), &ns.declarations, method) {
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
    method.name.name.clone()
}

fn find_in_namespace<'a>(ns_path: Option<&str>, members: &'a [NamespaceBodyDeclaration], method: &MethodDeclaration) -> Option<(String, String)> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path { Some(p) => format!("{}.{}", p, inner.name.name), None => inner.name.name.clone() };
                if let Some((cfqn, name)) = find_in_namespace(Some(&new_ns), &inner.declarations, method) { return Some((cfqn, name)); }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some((cfqn, name)) = find_in_class(ns_path, class, method, &mut Vec::new()) { return Some((cfqn, name)); }
            }
            _ => {}
        }
    }
    None
}

fn find_in_class(ns_path: Option<&str>, class: &ClassDeclaration, method: &MethodDeclaration, stack: &mut Vec<String>) -> Option<(String, String)> {
    stack.push(class.name.name.clone());
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Method(m) => {
                if std::ptr::eq(m, method) {
                    let class_path = stack.join(".");
                    let cfqn = match ns_path { Some(ns) => format!("{}.{}", ns, class_path), None => class_path };
                    let name = method.name.name.clone();
                    stack.pop();
                    return Some((cfqn, name));
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                if let Some(found) = find_in_class(ns_path, nested, method, stack) { stack.pop(); return Some(found); }
            }
            _ => {}
        }
    }
    stack.pop();
    None
}
