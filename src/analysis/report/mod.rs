use serde::{Deserialize, Serialize};

use crate::analysis::artifacts::cfg::ControlFlowIndex;
use crate::analysis::artifacts::dependencies::DependencySummary;
use crate::analysis::artifacts::metrics::AstAnalysis;
use crate::analysis::diagnostics::diagnostic_collection::DiagnosticCollection;

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
    pub workspace_warnings: Vec<String>,
    pub workspace_errors: Vec<String>,
}

impl AnalysisReport {
    pub fn from_session(session: &crate::analysis::framework::session::AnalysisSession) -> Self {
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
                if s.complexity > high_complexity_threshold { high_complexity_methods += 1; }
                if s.max_nesting > deep_nesting_threshold { deep_nesting_methods += 1; }
            }
            CfgSummary { total_methods, high_complexity_methods, deep_nesting_methods }
        });

        // Dependencies summary
        let deps = session.artifacts.get::<crate::analysis::artifacts::dependencies::DependencyGraph>().map(|g| {
            DependencySummary { nodes: g.nodes.len(), edges: g.edges.len() }
        });

        AnalysisReport { schema_version: 1, diagnostics, metrics, cfg, deps, workspace_warnings: Vec::new(), workspace_errors: Vec::new() }
    }
}
