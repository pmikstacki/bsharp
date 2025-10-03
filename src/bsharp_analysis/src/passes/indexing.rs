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
                TopLevelDeclaration::Interface(i) => {
                    let name = i.name.name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Interface,
                        None,
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Struct(s) => {
                    let name = s.name.name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Struct,
                        None,
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Record(r) => {
                    let name = r.name.name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Record,
                        None,
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Enum(e) => {
                    let name = e.name.name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Enum,
                        None,
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Delegate(d) => {
                    let name = d.name.name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Delegate,
                        None,
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::FileScopedNamespace(fs) => {
                    let name = fs.name.name.clone();
                    // File-scoped namespace acts as a root namespace path
                    let ns_path = name.clone();
                    symbols.insert(
                        &name,
                        SymbolKind::Namespace,
                        Some(ns_path.clone()),
                        Some(file_path.clone()),
                        None,
                        None,
                    );
                    names.0.insert(name.clone(), 1);
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
            symbols.insert(
                &seg,
                SymbolKind::Namespace,
                Some(full_ns.clone()),
                Some(file_path.to_string()),
                None,
                None,
            );
            names.0.insert(seg.clone(), 1);
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
                        symbols.insert(
                            &name,
                            SymbolKind::Struct,
                            Some(type_fqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Interface(i) => {
                        let name = i.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        symbols.insert(
                            &name,
                            SymbolKind::Interface,
                            Some(type_fqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Enum(e) => {
                        let name = e.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        symbols.insert(
                            &name,
                            SymbolKind::Enum,
                            Some(type_fqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Delegate(d) => {
                        let name = d.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        symbols.insert(
                            &name,
                            SymbolKind::Delegate,
                            Some(type_fqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Record(r) => {
                        let name = r.name.name.clone();
                        let type_fqn = format!("{}.{}", full_ns, name);
                        symbols.insert(
                            &name,
                            SymbolKind::Record,
                            Some(type_fqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
                        names.0.insert(name.clone(), 1);
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
            symbols.insert(
                &name,
                SymbolKind::Class,
                Some(class_fqn.clone()),
                Some(file_path.to_string()),
                None,
                None,
            );
            names.0.insert(name.clone(), 1);
            // Methods and nested classes
            for member in &class.body_declarations {
                match member {
                    ClassBodyDeclaration::Method(m) => {
                        let mname = m.name.name.clone();
                        let mfqn = format!("{}::{}", class_fqn, mname);
                        symbols.insert(
                            &mname,
                            SymbolKind::Method,
                            Some(mfqn),
                            Some(file_path.to_string()),
                            None,
                            None,
                        );
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
        session.artifacts.insert(symbols);
        session.artifacts.insert(names);
        session.artifacts.insert(fqn);
    }
}
