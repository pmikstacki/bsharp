use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::analysis::artifacts::symbols::SymbolId;

/// Dependency graph representation (inlined from legacy definitions for new artifacts)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<SymbolId, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, id: SymbolId, node_type: DependencyNodeType) {
        self.nodes.insert(id, DependencyNode { id, node_type });
    }

    pub fn add_dependency(&mut self, from: SymbolId, to: SymbolId, dependency_type: DependencyType) {
        self.edges.push(DependencyEdge { from, to, dependency_type });
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn edge_count(&self) -> usize { self.edges.len() }
}

/// Node in a dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: SymbolId,
    pub node_type: DependencyNodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyNodeType {
    Class,
    Interface,
    Method,
    Field,
    Property,
}

/// Edge in a dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: SymbolId,
    pub to: SymbolId,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Inheritance,
    Implementation,
    Usage,
    MethodCall,
    FieldAccess,
    PropertyAccess,
}

/// Minimal summary of dependency counts for reporting convenience.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencySummary {
    pub nodes: usize,
    pub edges: usize,
}
