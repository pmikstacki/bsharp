use crate::analysis::artifacts::symbols::{FqnMap, NameIndex, SymbolIndex, SymbolKind};
use crate::analysis::framework::passes::{AnalyzerPass, Phase};
use crate::analysis::framework::session::AnalysisSession;
use crate::syntax::ast::{CompilationUnit, TopLevelDeclaration};
use crate::syntax::nodes::declarations::{ClassBodyDeclaration, NamespaceBodyDeclaration, ClassDeclaration, NamespaceDeclaration};

pub struct IndexingPass;

impl AnalyzerPass for IndexingPass {
    fn id(&self) -> &'static str { "passes.indexing" }
    fn phase(&self) -> Phase { Phase::Index }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let mut symbols = SymbolIndex::new();
        let mut names = NameIndex::default();
        let mut fqn = FqnMap::default();

        for decl in &cu.declarations {
            match decl {
                TopLevelDeclaration::Namespace(ns) => index_namespace(ns, &mut symbols, &mut names, &mut fqn),
                TopLevelDeclaration::Class(c) => index_class(c, &mut symbols, &mut names),
                TopLevelDeclaration::Interface(i) => {
                    let name = i.name.name.clone();
                    symbols.insert(&name, SymbolKind::Interface, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Struct(s) => {
                    let name = s.name.name.clone();
                    symbols.insert(&name, SymbolKind::Struct, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Record(r) => {
                    let name = r.name.name.clone();
                    symbols.insert(&name, SymbolKind::Record, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Enum(e) => {
                    let name = e.name.name.clone();
                    symbols.insert(&name, SymbolKind::Enum, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::Delegate(d) => {
                    let name = d.name.name.clone();
                    symbols.insert(&name, SymbolKind::Delegate, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::FileScopedNamespace(fs) => {
                    let name = fs.name.name.clone();
                    symbols.insert(&name, SymbolKind::Namespace, None, None, None, None);
                    names.0.insert(name.clone(), 1);
                }
                TopLevelDeclaration::GlobalAttribute(_) => {}
            }
        }

        fn index_namespace(ns: &NamespaceDeclaration, symbols: &mut SymbolIndex, names: &mut NameIndex, fqn: &mut FqnMap) {
            let ns_name = ns.name.name.clone();
            symbols.insert(&ns_name, SymbolKind::Namespace, None, None, None, None);
            names.0.insert(ns_name.clone(), 1);
            fqn.0.entry(ns_name.clone()).or_default().push(ns_name.clone());
            for member in &ns.declarations {
                match member {
                    NamespaceBodyDeclaration::Namespace(inner) => index_namespace(inner, symbols, names, fqn),
                    NamespaceBodyDeclaration::Class(class) => index_class(class, symbols, names),
                    NamespaceBodyDeclaration::Struct(s) => {
                        let name = s.name.name.clone();
                        symbols.insert(&name, SymbolKind::Struct, None, None, None, None);
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Interface(i) => {
                        let name = i.name.name.clone();
                        symbols.insert(&name, SymbolKind::Interface, None, None, None, None);
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Enum(e) => {
                        let name = e.name.name.clone();
                        symbols.insert(&name, SymbolKind::Enum, None, None, None, None);
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Delegate(d) => {
                        let name = d.name.name.clone();
                        symbols.insert(&name, SymbolKind::Delegate, None, None, None, None);
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::Record(r) => {
                        let name = r.name.name.clone();
                        symbols.insert(&name, SymbolKind::Record, None, None, None, None);
                        names.0.insert(name.clone(), 1);
                    }
                    NamespaceBodyDeclaration::GlobalAttribute(_) | NamespaceBodyDeclaration::Preprocessor(_) => {}
                }
            }
        }

        fn index_class(class: &ClassDeclaration, symbols: &mut SymbolIndex, names: &mut NameIndex) {
            let name = class.name.name.clone();
            symbols.insert(&name, SymbolKind::Class, None, None, None, None);
            names.0.insert(name.clone(), 1);
            // Methods and nested classes
            for member in &class.body_declarations {
                match member {
                    ClassBodyDeclaration::Method(m) => {
                        let mname = m.name.name.clone();
                        symbols.insert(&mname, SymbolKind::Method, None, None, None, None);
                    }
                    ClassBodyDeclaration::NestedClass(nested) => index_class(nested, symbols, names),
                    _ => {}
                }
            }
        }

        // Store artifacts for later phases/rules
        session.artifacts.insert(symbols);
        session.artifacts.insert(names);
        session.artifacts.insert(fqn);
    }
}
