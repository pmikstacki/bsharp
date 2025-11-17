use syntax::ast::{CompilationUnit, TopLevelDeclaration};
use syntax::declarations::{
    namespace_declaration::NamespaceBodyDeclaration,
    ClassBodyDeclaration,
    ClassDeclaration,
    ConstructorDeclaration,
    MethodDeclaration,
    NamespaceDeclaration,
    PropertyDeclaration,
};
use syntax::identifier::Identifier as SynIdentifier;
use syntax::node::dyn_node_ref::DynNodeRef;
use syntax::spans::span_db::SpanDb;
use std::ops::Range;
use std::collections::HashMap;

pub type SpanTable = HashMap<String, Range<usize>>;

fn ident_text(id: &SynIdentifier) -> String {
    match id {
        SynIdentifier::Simple(s) => s.clone(),
        SynIdentifier::QualifiedIdentifier(segs) => segs.join("."),
        SynIdentifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

fn method_fqn(cu: &CompilationUnit, method: &MethodDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some((cfqn, name)) = find_in_namespace(None, &fs.declarations, method) {
            return format!("{}::{}", cfqn, name);
        }
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(ns) = decl {
            let ns_path = ident_text(&ns.name);
            if let Some((cfqn, name)) = find_in_namespace(Some(&ns_path), &ns.declarations, method)
            {
                return format!("{}::{}", cfqn, name);
            }
        } else if let TopLevelDeclaration::Class(c) = decl {
            if let Some((cfqn, name)) = find_in_class(None, c, method, &mut Vec::new()) {
                return format!("{}::{}", cfqn, name);
            }
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
                if let Some((cfqn, name)) =
                    find_in_namespace(Some(&new_ns), &inner.declarations, method)
                {
                    return Some((cfqn, name));
                }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some((cfqn, name)) = find_in_class(ns_path, class, method, &mut Vec::new())
                {
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
                        Some(ns) => format!("{}.{class}", ns, class = class_path),
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

fn class_fqn(cu: &CompilationUnit, class: &ClassDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some(cfqn) = find_class_in_namespace(None, &fs.declarations, class, &mut Vec::new())
        {
            return cfqn;
        }
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(ns) = decl {
            let ns_path = ident_text(&ns.name);
            if let Some(cfqn) = find_class_in_namespace(
                Some(&ns_path),
                &ns.declarations,
                class,
                &mut Vec::new(),
            ) {
                return cfqn;
            }
        } else if let TopLevelDeclaration::Class(c) = decl {
            if let Some(cfqn) = find_class_path(None, c, class, &mut Vec::new()) {
                return cfqn;
            }
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
                if let Some(cfqn) =
                    find_class_in_namespace(Some(&new_ns), &inner.declarations, target, stack)
                {
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
        if let ClassBodyDeclaration::NestedClass(nested) = member {
            if let Some(path) = find_class_path(ns_path, nested, target, stack) {
                stack.pop();
                return Some(path);
            }
        }
    }
    let class_path = stack.join(".");
    if std::ptr::eq(class, target) {
        let cfqn = match ns_path {
            Some(ns) => format!("{}.{class}", ns, class = class_path),
            None => class_path,
        };
        stack.pop();
        return Some(cfqn);
    }
    stack.pop();
    None
}

fn namespace_fqn(cu: &CompilationUnit, ns: &NamespaceDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some(path) = find_namespace_path(None, &fs.declarations, ns) {
            return path;
        }
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

fn constructor_owner_fqn(cu: &CompilationUnit, ctor: &ConstructorDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some(cfqn) = find_ctor_in_namespace(None, &fs.declarations, ctor) {
            return cfqn;
        }
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(ns) = decl {
            let ns_path = ident_text(&ns.name);
            if let Some(cfqn) = find_ctor_in_namespace(Some(&ns_path), &ns.declarations, ctor) {
                return cfqn;
            }
        } else if let TopLevelDeclaration::Class(c) = decl {
            if let Some(cfqn) = find_ctor_in_class(None, c, ctor, &mut Vec::new()) {
                return cfqn;
            }
        }
    }
    String::new()
}

fn find_ctor_in_namespace(
    ns_path: Option<&str>,
    members: &[NamespaceBodyDeclaration],
    ctor: &ConstructorDeclaration,
) -> Option<String> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path {
                    Some(p) => format!("{}.{}", p, ident_text(&inner.name)),
                    None => ident_text(&inner.name),
                };
                if let Some(cfqn) = find_ctor_in_namespace(Some(&new_ns), &inner.declarations, ctor)
                {
                    return Some(cfqn);
                }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some(cfqn) = find_ctor_in_class(ns_path, class, ctor, &mut Vec::new()) {
                    return Some(cfqn);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_ctor_in_class(
    ns_path: Option<&str>,
    class: &ClassDeclaration,
    ctor: &ConstructorDeclaration,
    stack: &mut Vec<String>,
) -> Option<String> {
    stack.push(ident_text(&class.name));
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Constructor(c) => {
                if std::ptr::eq(c, ctor) {
                    let class_path = stack.join(".");
                    let cfqn = match ns_path {
                        Some(ns) => format!("{}.{class}", ns, class = class_path),
                        None => class_path,
                    };
                    stack.pop();
                    return Some(cfqn);
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                if let Some(cfqn) = find_ctor_in_class(ns_path, nested, ctor, stack) {
                    stack.pop();
                    return Some(cfqn);
                }
            }
            _ => {}
        }
    }
    stack.pop();
    None
}

fn property_fqn(cu: &CompilationUnit, prop: &PropertyDeclaration) -> String {
    if let Some(fs) = &cu.file_scoped_namespace {
        if let Some((cfqn, name)) = find_prop_in_namespace(None, &fs.declarations, prop) {
            return format!("{}::{}", cfqn, name);
        }
    }
    for decl in &cu.declarations {
        if let TopLevelDeclaration::Namespace(ns) = decl {
            let ns_path = ident_text(&ns.name);
            if let Some((cfqn, name)) = find_prop_in_namespace(Some(&ns_path), &ns.declarations, prop) {
                return format!("{}::{}", cfqn, name);
            }
        } else if let TopLevelDeclaration::Class(c) = decl {
            if let Some((cfqn, name)) = find_prop_in_class(None, c, prop, &mut Vec::new()) {
                return format!("{}::{}", cfqn, name);
            }
        }
    }
    String::new()
}

fn find_prop_in_namespace(
    ns_path: Option<&str>,
    members: &[NamespaceBodyDeclaration],
    prop: &PropertyDeclaration,
) -> Option<(String, String)> {
    for m in members {
        match m {
            NamespaceBodyDeclaration::Namespace(inner) => {
                let new_ns = match ns_path {
                    Some(p) => format!("{}.{}", p, ident_text(&inner.name)),
                    None => ident_text(&inner.name),
                };
                if let Some(found) =
                    find_prop_in_namespace(Some(&new_ns), &inner.declarations, prop)
                {
                    return Some(found);
                }
            }
            NamespaceBodyDeclaration::Class(class) => {
                if let Some(found) = find_prop_in_class(ns_path, class, prop, &mut Vec::new()) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_prop_in_class(
    ns_path: Option<&str>,
    class: &ClassDeclaration,
    prop: &PropertyDeclaration,
    stack: &mut Vec<String>,
) -> Option<(String, String)> {
    stack.push(ident_text(&class.name));
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Property(p) => {
                if std::ptr::eq(p, prop) {
                    let class_path = stack.join(".");
                    let cfqn = match ns_path {
                        Some(ns) => format!("{}.{class}", ns, class = class_path),
                        None => class_path,
                    };
                    let name = ident_text(&p.name);
                    stack.pop();
                    return Some((cfqn, name));
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                if let Some(found) = find_prop_in_class(ns_path, nested, prop, stack) {
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

pub fn build_span_db_from_table(cu: &CompilationUnit, table: &SpanTable) -> SpanDb {
    let mut db = SpanDb::new();
    // Namespaces and Classes
    for decl in &cu.declarations {
        match decl {
            TopLevelDeclaration::Namespace(ns) => {
                let nfqn = namespace_fqn(cu, ns);
                if let Some(range) = table.get(&format!("namespace::{}", nfqn)) {
                    db.insert(DynNodeRef(ns), range.clone());
                }
                for m in &ns.declarations {
                    if let NamespaceBodyDeclaration::Class(class) = m {
                        let cfqn = class_fqn(cu, class);
                        if let Some(range) = table.get(&format!("class::{}", cfqn)) {
                            db.insert(DynNodeRef(class), range.clone());
                        }
                        populate_class_members(cu, class, table, &mut db);
                    } else if let NamespaceBodyDeclaration::Namespace(inner) = m {
                        let infqn = namespace_fqn(cu, inner);
                        if let Some(range) = table.get(&format!("namespace::{}", infqn)) {
                            db.insert(DynNodeRef(inner), range.clone());
                        }
                    }
                }
            }
            TopLevelDeclaration::Class(class) => {
                let cfqn = class_fqn(cu, class);
                if let Some(range) = table.get(&format!("class::{}", cfqn)) {
                    db.insert(DynNodeRef(class), range.clone());
                }
                populate_class_members(cu, class, table, &mut db);
            }
            _ => {}
        }
    }
    db
}

fn populate_class_members(cu: &CompilationUnit, class: &ClassDeclaration, table: &SpanTable, db: &mut SpanDb) {
    for member in &class.body_declarations {
        match member {
            ClassBodyDeclaration::Method(m) => {
                let mfqn = method_fqn(cu, m);
                if let Some(range) = table.get(&format!("method::{}", mfqn)) {
                    db.insert(DynNodeRef(m), range.clone());
                }
            }
            ClassBodyDeclaration::Constructor(c) => {
                let owner = constructor_owner_fqn(cu, c);
                if !owner.is_empty() {
                    if let Some(range) = table.get(&format!("ctor::{}", owner)) {
                        db.insert(DynNodeRef(c), range.clone());
                    }
                }
            }
            ClassBodyDeclaration::Property(p) => {
                let pfqn = property_fqn(cu, p);
                if !pfqn.is_empty() {
                    if let Some(range) = table.get(&format!("property::{}", pfqn)) {
                        db.insert(DynNodeRef(p), range.clone());
                    }
                }
            }
            ClassBodyDeclaration::NestedClass(nested) => {
                let cfqn = class_fqn(cu, nested);
                if let Some(range) = table.get(&format!("class::{}", cfqn)) {
                    db.insert(DynNodeRef(nested), range.clone());
                }
                populate_class_members(cu, nested, table, db);
            }
            _ => {}
        }
    }
}
