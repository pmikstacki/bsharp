use crate::artifacts::control_flow_graph::stats::MethodControlFlowStats;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Index of control flow stats for methods within a single file.
/// Key format: fully-qualified name "Namespace.Nested.Type::Method".
/// If no namespace is present, the key is "Type::Method".
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ControlFlowIndex(pub HashMap<String, MethodControlFlowStats>);

impl ControlFlowIndex {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn insert(&mut self, key: String, stats: MethodControlFlowStats) {
        self.0.insert(key, stats);
    }
    pub fn get(&self, key: &str) -> Option<&MethodControlFlowStats> {
        self.0.get(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &MethodControlFlowStats)> {
        self.0.iter()
    }
}
