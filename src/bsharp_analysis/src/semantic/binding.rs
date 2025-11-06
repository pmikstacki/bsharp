use crate::framework::{AnalysisSession, AnalyzerPass, Phase, Query};
use crate::syntax::ast::CompilationUnit;
use crate::semantic::symbols::SymbolTable;
use bsharp_syntax::types::Type;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BindingTarget {
    Namespace(String),
    Type(String),
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BindingTable {
    pub types_by_simple: HashMap<String, String>, // simple name -> resolved FQN
}

fn ident_text(id: &crate::syntax::Identifier) -> String {
    match id {
        crate::syntax::Identifier::Simple(s) => s.clone(),
        crate::syntax::Identifier::QualifiedIdentifier(parts) => parts.join("."),
        crate::syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
    }
}

pub struct BindingPass;

impl AnalyzerPass for BindingPass {
    fn id(&self) -> &'static str { "semantic.binding" }
    fn phase(&self) -> Phase { Phase::Semantic }
    fn depends_on(&self) -> &'static [&'static str] { &["semantic.symbols"] }
    fn run(&self, cu: &CompilationUnit, session: &mut AnalysisSession) {
        let Some(symtab) = session.get_artifact::<SymbolTable>() else {
            return;
        };
        let symtab = symtab.as_ref();
        let mut table = BindingTable::default();

        for t in Query::from(cu).of::<Type>() {
            match t {
                Type::Reference(id) => {
                    let name = ident_text(id);
                    let cand = symtab.resolve_simple(&name);
                    match cand.len() {
                        0 => {
                            crate::framework::diagnostic_builder::DiagnosticBuilder::new(crate::DiagnosticCode::BSE03012)
                                .with_message(format!("Unresolved name '{name}'"))
                                .emit(session);
                        }
                        1 => {
                            table.types_by_simple.insert(name, cand[0].clone());
                        }
                        _ => {
                            // Ambiguous; treat as unresolved for now
                            crate::framework::diagnostic_builder::DiagnosticBuilder::new(crate::DiagnosticCode::BSE03012)
                                .with_message(format!("Ambiguous name '{}': {} candidates", name, cand.len()))
                                .emit(session);
                        }
                    }
                }
                Type::Generic { base, .. } => {
                    let name = ident_text(base);
                    let cand = symtab.resolve_simple(&name);
                    if cand.len() == 1 {
                        table.types_by_simple.insert(name, cand[0].clone());
                    } else if cand.is_empty() {
                        crate::framework::diagnostic_builder::DiagnosticBuilder::new(crate::DiagnosticCode::BSE03012)
                            .with_message(format!("Unresolved name '{name}'"))
                            .emit(session);
                    } else {
                        crate::framework::diagnostic_builder::DiagnosticBuilder::new(crate::DiagnosticCode::BSE03012)
                            .with_message(format!("Ambiguous name '{}': {} candidates", name, cand.len()))
                            .emit(session);
                    }
                }
                _ => {}
            }
        }

        session.insert_artifact(table);
    }
}
