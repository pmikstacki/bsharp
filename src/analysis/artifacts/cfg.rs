// Control Flow artifacts: per-method control flow stats and optional CFG placeholder
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lightweight per-method control flow statistics used by smell rules and reporting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MethodControlFlowStats {
    pub complexity: usize,
    pub max_nesting: usize,
    pub exit_points: usize,
    pub statement_count: usize,
}

/// Index of control flow stats for methods within a single file.
/// Key format (v1): "ClassName::MethodName". In multi-file v2, include FileId/FQN.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowIndex(pub HashMap<String, MethodControlFlowStats>);

impl ControlFlowIndex {
    pub fn new() -> Self { Self(HashMap::new()) }
    pub fn insert(&mut self, key: String, stats: MethodControlFlowStats) { self.0.insert(key, stats); }
    pub fn get(&self, key: &str) -> Option<&MethodControlFlowStats> { self.0.get(key) }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &MethodControlFlowStats)> { self.0.iter() }
}
