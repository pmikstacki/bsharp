use crate::framework::{AnalysisSession, AnalyzerPass, Phase, Query};
use crate::diag;
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
                            diag!(session, crate::DiagnosticCode::BSE03012, at id, msg: format!("Unresolved name '{name}'"));
                        }
                        1 => {
                            table.types_by_simple.insert(name, cand[0].clone());
                        }
                        _ => {
                            // Ambiguous; treat as unresolved for now
                            diag!(
                                session,
                                crate::DiagnosticCode::BSE03012,
                                at id,
                                msg: format!("Ambiguous name '{}': {} candidates", name, cand.len())
                            );
                        }
                    }
                }
                Type::Generic { base, .. } => {
                    let name = ident_text(base);
                    let cand = symtab.resolve_simple(&name);
                    if cand.len() == 1 {
                        table.types_by_simple.insert(name, cand[0].clone());
                    } else if cand.is_empty() {
                        diag!(session, crate::DiagnosticCode::BSE03012, at base, msg: format!("Unresolved name '{name}'"));
                    } else {
                        diag!(
                            session,
                            crate::DiagnosticCode::BSE03012,
                            at base,
                            msg: format!("Ambiguous name '{}': {} candidates", name, cand.len())
                        );
                    }
                }
                _ => {}
            }
        }

        session.insert_artifact(table);
    }
}
