use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dependency graph representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
    pub edges: Vec<DependencyEdge>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_node(&mut self, name: String, node_type: DependencyNodeType) {
        self.nodes.insert(name.clone(), DependencyNode {
            name,
            node_type,
        });
    }
    
    pub fn add_dependency(&mut self, from: String, to: String, dependency_type: DependencyType) {
        self.edges.push(DependencyEdge {
            from,
            to,
            dependency_type,
        });
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

/// Node in a dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub name: String,
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
    pub from: String,
    pub to: String,
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

/// Circular dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependency {
    pub cycle: Vec<String>,
    pub dependency_types: Vec<DependencyType>,
}

/// Class-level dependency information
#[derive(Debug, Clone, Default)]
pub struct ClassDependencies {
    pub inherits_from: Vec<String>,
    pub implements: Vec<String>,
    pub field_dependencies: Vec<String>,
    pub method_dependencies: Vec<String>,
}

impl ClassDependencies {
    pub fn total_dependencies(&self) -> usize {
        self.inherits_from.len() + 
        self.implements.len() + 
        self.field_dependencies.len() + 
        self.method_dependencies.len()
    }
}

/// Module dependency information
#[derive(Debug, Clone, Default)]
pub struct ModuleDependencies {
    pub name: String,
    pub outgoing_dependencies: Vec<String>,
    pub incoming_dependencies: Vec<String>,
    pub internal_classes: usize,
    pub abstract_classes: usize,
    pub interfaces: usize,
}

/// Dependency metrics
#[derive(Debug, Clone, Default)]
pub struct DependencyMetrics {
    pub total_dependencies: usize,
    pub incoming_dependencies: usize,
    pub outgoing_dependencies: usize,
    pub coupling_factor: f64,
    pub stability: f64,
    pub abstractness: f64,
    pub instability: f64,
    pub distance_from_main_sequence: f64,
}

/// Coupling metrics
#[derive(Debug, Clone, Default)]
pub struct CouplingMetrics {
    pub efferent_coupling: usize,
    pub afferent_coupling: usize,
    pub coupling_factor: f64,
}

/// Layer violation information
#[derive(Debug, Clone)]
pub struct LayerViolation {
    pub from_layer: String,
    pub to_layer: String,
    pub violating_dependency: Dependency,
}

/// Simple dependency representation
#[derive(Debug, Clone)]
pub struct Dependency {
    pub from: String,
    pub to: String,
}

/// Dependency inversion violation
#[derive(Debug, Clone)]
pub struct DependencyInversionViolation {
    pub concrete_dependency: String,
    pub suggestion: String,
}

/// Fan-in/fan-out metrics
#[derive(Debug, Clone, Default)]
pub struct FanMetrics {
    pub fan_in: usize,
    pub fan_out: usize,
    pub stability_index: f64,
    pub instability_index: f64,
}

/// Change impact analysis
#[derive(Debug, Clone, Default)]
pub struct ChangeImpact {
    pub directly_affected: Vec<String>,
    pub transitively_affected: Vec<String>,
    pub impact_level: ImpactLevel,
}

impl ChangeImpact {
    pub fn total_affected_modules(&self) -> usize {
        self.directly_affected.len() + self.transitively_affected.len()
    }
}

#[derive(Debug, Clone, Default)]
pub enum ImpactLevel {
    #[default]
    Low,
    Medium,
    High,
}

/// Interface segregation issues
#[derive(Debug, Clone)]
pub struct InterfaceSegregationIssues {
    pub interface_too_large: bool,
    pub unused_methods: Vec<String>,
    pub suggested_interfaces: Vec<String>,
}

/// Module cohesion metrics
#[derive(Debug, Clone, Default)]
pub struct ModuleCohesionMetrics {
    pub internal_coupling: usize,
    pub cohesion_ratio: f64,
}

/// Overall dependency metrics
#[derive(Debug, Clone, Default)]
pub struct OverallDependencyMetrics {
    pub total_modules: usize,
    pub total_dependencies: usize,
    pub average_fan_out: f64,
    pub average_fan_in: f64,
    pub coupling_density: f64,
}

/// Architectural smells
#[derive(Debug, Clone)]
pub enum ArchitecturalSmell {
    Hub {
        module_name: String,
        fan_in: usize,
    },
    GodComponent {
        module_name: String,
        fan_out: usize,
    },
    CyclicDependency {
        cycle: Vec<String>,
    },
} 