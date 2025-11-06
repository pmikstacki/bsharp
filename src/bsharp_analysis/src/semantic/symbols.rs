use crate::framework::{AnalysisSession, AnalyzerPass, Phase, Query};
use crate::syntax::ast::CompilationUnit;
use bsharp_syntax::declarations::{NamespaceDeclaration, TypeDeclaration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Namespace,
    Class,
    Struct,
    Interface,
    Record,
    Enum,
    Delegate,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub name: String,
    pub kind: SymbolKind,
    pub fqn: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct SymbolTable {
    pub by_fqn: HashMap<String, SymbolEntry>,
    pub by_simple: HashMap<String, Vec<String>>, // simple name -> fqns
}

impl SymbolTable {
    pub fn insert(&mut self, se: SymbolEntry) {
        self.by_simple.entry(se.name.clone()).or_default().push(se.fqn.clone());
        self.by_fqn.insert(se.fqn.clone(), se);
    }

    pub fn resolve_simple(&self, name: &str) -> &[String] {
        self.by_simple.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

fn ident_text(id: &crate::syntax::Identifier) -> String {
    match id {
        crate::syntax::Identifier::Simple(s) => s.clone(),
        crate::syntax::Identifier::QualifiedIdentifier(parts) => parts.join("."),
        crate::syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

fn fqn_for_type(ns_prefix: Option<&str>, name: &str) -> String {
    if let Some(ns) = ns_prefix {
        format!("{}.{name}", ns)
    } else {
        name.to_string()
    }
}

pub struct SymbolsPass;

impl AnalyzerPass for SymbolsPass {
    fn id(&self) -> &'static str { "semantic.symbols" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let file_ns = Query::from(cu)
            .of::<NamespaceDeclaration>()
            .map(|ns| ident_text(&ns.name))
            .collect::<Vec<_>>();
        // Only support single namespace prefix per file (file-scoped or outermost); fall back to None
        let ns_prefix = file_ns.first().map(|s| s.as_str());

        let mut table = SymbolTable::default();
        let mut duplicates: Vec<(String, String)> = Vec::new(); // (name, fqn)

        for td in Query::from(cu).of::<TypeDeclaration>() {
            let (name, kind) = match td {
                TypeDeclaration::Class(c) => (ident_text(&c.name), SymbolKind::Class),
                TypeDeclaration::Struct(s) => (ident_text(&s.name), SymbolKind::Struct),
                TypeDeclaration::Interface(i) => (ident_text(&i.name), SymbolKind::Interface),
                TypeDeclaration::Record(r) => (ident_text(&r.name), SymbolKind::Record),
                TypeDeclaration::Enum(e) => (ident_text(&e.name), SymbolKind::Enum),
                TypeDeclaration::Delegate(d) => (ident_text(&d.name), SymbolKind::Delegate),
            };
            let fqn = fqn_for_type(ns_prefix, &name);
            let entry = SymbolEntry { name: name.clone(), kind, fqn: fqn.clone() };
            if let Some(existing) = table.by_fqn.get(&fqn) {
                duplicates.push((name.clone(), existing.fqn.clone()));
            } else {
                table.insert(entry);
            }
        }

        // Emit duplicate symbol diagnostics (same FQN within file)
        for (name, fqn) in duplicates {
            crate::framework::diagnostic_builder::DiagnosticBuilder::new(crate::DiagnosticCode::BSE03011)
                .with_message(format!("Duplicate symbol '{name}' ({fqn}) in the same file"))
                .emit(session);
        }

        session.insert_artifact(table);
    }
}
