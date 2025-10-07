use crate::artifacts::symbols::{FqnMap, NameIndex, SymbolIndex, SymbolKind};
use crate::framework::{AnalysisSession, AnalyzerPass, Phase};
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{
    ClassBodyDeclaration, ClassDeclaration, NamespaceBodyDeclaration, NamespaceDeclaration,
};

pub struct IndexingPass;

impl AnalyzerPass for IndexingPass {
    fn id(&self) -> &'static str {
        "passes.indexing"
    }
    fn phase(&self) -> Phase {
        Phase::Index
    }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut symbols = SymbolIndex::new();
        let mut names = NameIndex::default();
        let mut fqn = FqnMap::default();
        let file_path = session.ctx.file_path().to_string();

        fn push_symbol(
            symbols: &mut SymbolIndex,
            names: &mut NameIndex,
            name: &str,
            kind: SymbolKind,
            fqn: Option<String>,
            file_path: &str,
        ) {
            symbols.insert(
                name,
                kind,
                fqn,
                Some(file_path.to_string()),
                None,
                None,
            );
            names.0.insert(name.to_string(), 1);
        }

        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Namespace(ns) => index_namespace(
                    ns,
                    None,
                    &file_path,
                    &mut symbols,
                    &mut names,
                    &mut fqn,
                    &mut Vec::new(),
                ),
                TopLevelDeclaration::Class(c) => index_class(
                    c,
                    None,
                    &file_path,
                    &mut symbols,
                    &mut names,
                    &mut Vec::new(),
                ),
                TopLevelDeclaration::Interface(i) => push_symbol(&mut symbols, &mut names, &i.name.name, SymbolKind::Interface, None, &file_path),
                TopLevelDeclaration::Struct(s) => push_symbol(&mut symbols, &mut names, &s.name.name, SymbolKind::Struct, None, &file_path),
                TopLevelDeclaration::Record(r) => push_symbol(&mut symbols, &mut names, &r.name.name, SymbolKind::Record, None, &file_path),
                TopLevelDeclaration::Enum(e) => push_symbol(&mut symbols, &mut names, &e.name.name, SymbolKind::Enum, None, &file_path),
                TopLevelDeclaration::Delegate(d) => push_symbol(&mut symbols, &mut names, &d.name.name, SymbolKind::Delegate, None, &file_path),
                TopLevelDeclaration::FileScopedNamespace(fs) => {
                    let name = fs.name.name.clone();
                    // File-scoped namespace acts as a root namespace path
                    let ns_path = name.clone();
                    push_symbol(&mut symbols, &mut names, &name, SymbolKind::Namespace, Some(ns_path.clone()), &file_path);
                    fqn.0.entry(name.clone()).or_default().push(ns_path);
                }
                TopLevelDeclaration::GlobalAttribute(_) => {}
            }
        }

        fn index_namespace(
            ns: &NamespaceDeclaration,
            ns_path: Option<String>,
            file_path: &str,
            symbols: &mut SymbolIndex,
            names: &mut NameIndex,
            fqn: &mut FqnMap,
            class_stack: &mut Vec<String>,
        ) {
            let seg = ns.name.name.clone();
            let full_ns = match ns_path {
                Some(ref p) if !p.is_empty() => format!("{}.{}", p, seg),
                _ => seg.clone(),
            };
            push_symbol(symbols, names, &seg, SymbolKind::Namespace, Some(full_ns.clone()), file_path);
            fqn.0.entry(seg.clone()).or_default().push(full_ns.clone());
            for member in &ns.declarations {
                match member {
                    NamespaceBodyDeclaration::Namespace(inner) => index_namespace(
                        inner,
                        Some(full_ns.clone()),
                        file_path,
                        symbols,
                        names,
                        fqn,
                        class_stack,
                    ),
                    NamespaceBodyDeclaration::Class(class) => index_class(
                        class,
                        Some(full_ns.clone()),
                        file_path,
                        symbols,
                        names,
                        class_stack,
                    ),
                    NamespaceBodyDeclaration::Struct(s) => {
                        let name = s.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        push_symbol(symbols, names, &name, SymbolKind::Struct, Some(type_fqn), file_path);
                    }
                    NamespaceBodyDeclaration::Interface(i) => {
                        let name = i.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        push_symbol(symbols, names, &name, SymbolKind::Interface, Some(type_fqn), file_path);
                    }
                    NamespaceBodyDeclaration::Enum(e) => {
                        let name = e.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        push_symbol(symbols, names, &name, SymbolKind::Enum, Some(type_fqn), file_path);
                    }
                    NamespaceBodyDeclaration::Delegate(d) => {
                        let name = d.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        push_symbol(symbols, names, &name, SymbolKind::Delegate, Some(type_fqn), file_path);
                    }
                    NamespaceBodyDeclaration::Record(r) => {
                        let name = r.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        push_symbol(symbols, names, &name, SymbolKind::Record, Some(type_fqn), file_path);
                    }
                    NamespaceBodyDeclaration::GlobalAttribute(_) => {}
                }
            }
        }

        fn index_class(
            class: &ClassDeclaration,
            ns_path: Option<String>,
            file_path: &str,
            symbols: &mut SymbolIndex,
            names: &mut NameIndex,
            class_stack: &mut Vec<String>,
        ) {
            class_stack.push(class.name.name.clone());
            let class_path = class_stack.join(".");
            let class_fqn = if let Some(ref ns) = ns_path {
                format!("{}.{}", ns, class_path)
            } else {
                class_path.clone()
            };
            let name = class.name.name.clone();
            push_symbol(symbols, names, &name, SymbolKind::Class, Some(class_fqn.clone()), file_path);
            // Methods and nested classes
            for member in &class.body_declarations {
                match member {
                    ClassBodyDeclaration::Method(m) => {
                        let mname = m.name.name.clone();
                        let mfqn = format!("{}::{}", class_fqn, mname);
                        push_symbol(symbols, names, &mname, SymbolKind::Method, Some(mfqn), file_path);
                    }
                    ClassBodyDeclaration::NestedClass(nested) => index_class(
                        nested,
                        ns_path.clone(),
                        file_path,
                        symbols,
                        names,
                        class_stack,
                    ),
                    _ => {}
                }
            }
            class_stack.pop();
        }

        // Store artifacts for later phases/rules
        session.insert_artifact(symbols);
        session.insert_artifact(names);
        session.insert_artifact(fqn);
    }
}
