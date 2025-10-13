use crate::artifacts::symbols::{FqnMap, NameIndex, SymbolIndex, SymbolKind};
use crate::framework::{
    class_fqn, method_fqn, namespace_fqn, AnalysisSession, AnalyzerPass, Phase, Query,
};
use crate::syntax::ast::CompilationUnit;
use bsharp_syntax::declarations::{ClassDeclaration, MethodDeclaration, NamespaceDeclaration};

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

        // Helper to insert symbols and names
        fn push_symbol(
            symbols: &mut SymbolIndex,
            names: &mut NameIndex,
            name: &str,
            kind: SymbolKind,
            fqn: Option<String>,
            file_path: &str,
        ) {
            symbols.insert(name, kind, fqn, Some(file_path.to_string()), None, None);
            names.0.insert(name.to_string(), 1);
        }

        // Namespaces
        for ns in Query::from(cu).of::<NamespaceDeclaration>() {
            let ns_path = namespace_fqn(cu, ns);
            let seg = ns.name.name.clone();
            push_symbol(
                &mut symbols,
                &mut names,
                &seg,
                SymbolKind::Namespace,
                Some(ns_path.clone()),
                &file_path,
            );
            fqn.0.entry(seg).or_default().push(ns_path);
        }

        // Classes and Methods
        for class in Query::from(cu).of::<ClassDeclaration>() {
            let cfqn = class_fqn(cu, class);
            let name = class.name.name.clone();
            push_symbol(
                &mut symbols,
                &mut names,
                &name,
                SymbolKind::Class,
                Some(cfqn),
                &file_path,
            );
        }
        for m in Query::from(cu).of::<MethodDeclaration>() {
            let mfqn = method_fqn(cu, m);
            let name = m.name.name.clone();
            push_symbol(
                &mut symbols,
                &mut names,
                &name,
                SymbolKind::Method,
                Some(mfqn),
                &file_path,
            );
        }

        // Store artifacts for later phases/rules
        session.insert_artifact(symbols);
        session.insert_artifact(names);
        session.insert_artifact(fqn);
    }
}
