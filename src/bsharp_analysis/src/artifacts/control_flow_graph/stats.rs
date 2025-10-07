use serde::{Deserialize, Serialize};

/// Lightweight per-method control flow statistics used by smell rules and reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MethodControlFlowStats {
    pub complexity: usize,
    pub max_nesting: usize,
    pub exit_points: usize,
    pub statement_count: usize,
}