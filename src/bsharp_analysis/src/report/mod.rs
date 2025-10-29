use crate::artifacts::control_flow_graph::index::ControlFlowIndex;
use crate::artifacts::dependencies::{DependencyGraph, DependencySummary};
use crate::artifacts::symbols::{SymbolId, SymbolIndex};
use crate::framework::AnalysisSession;
use crate::{AstAnalysis, DiagnosticCollection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgSummary {
    pub total_methods: usize,
    pub high_complexity_methods: usize,
    pub deep_nesting_methods: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub schema_version: u32,
    pub diagnostics: DiagnosticCollection,
    pub metrics: Option<AstAnalysis>,
    pub cfg: Option<CfgSummary>,
    pub deps: Option<DependencySummary>,
    // Quality report removed in purge; reserved for future use
    pub workspace_warnings: Vec<String>,
    pub workspace_errors: Vec<String>,
    #[serde(skip)]
    pub deps_node_keys: Option<std::collections::HashSet<String>>,
    #[serde(skip)]
    pub deps_edge_keys: Option<std::collections::HashSet<String>>,
}

impl AnalysisReport {
    pub fn from_session(session: &AnalysisSession) -> Self {
        // Diagnostics cloned as-is (sorted elsewhere if needed)
        let diagnostics = session.diagnostics.clone();

        // Metrics
        let metrics = session.artifacts.get::<AstAnalysis>().map(|a| (*a).clone());

        // CFG summary
        let cfg = session.artifacts.get::<ControlFlowIndex>().map(|idx| {
            let total_methods = idx.0.len();
            let high_complexity_threshold = session.config.cf_high_complexity_threshold;
            let deep_nesting_threshold = session.config.cf_deep_nesting_threshold;
            let mut high_complexity_methods = 0usize;
            let mut deep_nesting_methods = 0usize;
            for (_k, s) in idx.iter() {
                if s.complexity > high_complexity_threshold {
                    high_complexity_methods += 1;
                }
                if s.max_nesting > deep_nesting_threshold {
                    deep_nesting_methods += 1;
                }
            }
            CfgSummary {
                total_methods,
                high_complexity_methods,
                deep_nesting_methods,
            }
        });

        // Dependencies summary and internal key sets for workspace dedupe
        let mut deps_node_keys: Option<std::collections::HashSet<String>> = None;
        let mut deps_edge_keys: Option<std::collections::HashSet<String>> = None;
        let deps = session.artifacts.get::<DependencyGraph>().map(|g| {
            // Build node/edge keys from FQNs if available, else names
            let mut node_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
            let mut edge_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
            let sym_idx = session.artifacts.get::<SymbolIndex>();
            fn id_to_key_with_idx(sym_idx: Option<&SymbolIndex>, id: &SymbolId) -> String {
                if let Some(idx) = sym_idx
                    && let Some(sym) = idx.get(*id)
                {
                    if let Some(f) = &sym.fqn {
                        return f.clone();
                    }
                    return sym.name.clone();
                }
                format!("id:{}", id.0)
            }
            for (_sid, node) in g.nodes.iter() {
                node_keys.insert(id_to_key_with_idx(sym_idx.as_deref(), &node.id));
            }
            for e in g.edges.iter() {
                let from = id_to_key_with_idx(sym_idx.as_deref(), &e.from);
                let to = id_to_key_with_idx(sym_idx.as_deref(), &e.to);

                edge_keys.insert(format!("{}|{}|{}", from, e.dependency_type, to));
            }
            deps_node_keys = Some(node_keys.clone());
            deps_edge_keys = Some(edge_keys.clone());
            DependencySummary {
                nodes: node_keys.len(),
                edges: edge_keys.len(),
            }
        });

        AnalysisReport {
            schema_version: 1,
            diagnostics,
            metrics,
            cfg,
            deps,
            workspace_warnings: Vec::new(),
            workspace_errors: Vec::new(),
            deps_node_keys,
            deps_edge_keys,
        }
    }
}
