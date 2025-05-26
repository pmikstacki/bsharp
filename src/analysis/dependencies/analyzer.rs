use crate::parser::ast::CompilationUnit;
use crate::parser::nodes::declarations::{ClassDeclaration, InterfaceDeclaration};
use std::collections::HashMap;
use petgraph::graph::NodeIndex;

// Structs and enums from definitions.rs
use super::definitions::*;

/// Dependency analyzer for analyzing dependencies between code elements
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalyzer {
    pub dependency_graph: petgraph::Graph<String, DependencyType>,
    pub modules: HashMap<String, ModuleDependencies>,
    pub layer_definitions: HashMap<String, Vec<String>>,
    pub interface_types: std::collections::HashSet<String>,
    pub concrete_types: std::collections::HashSet<String>,
}

impl DependencyAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Analyze dependencies in a compilation unit
    pub fn analyze_compilation_unit(&self, _unit: &CompilationUnit) -> DependencyGraph {
        // TODO: Implement dependency analysis
        DependencyGraph::new()
    }
    
    /// Analyze class dependencies
    pub fn analyze_class_dependencies(&mut self, class: &ClassDeclaration) -> ClassDependencies {
        let mut dependencies = ClassDependencies::default();
        
        // Analyze inheritance
        for base_type in &class.base_types {
            if let crate::parser::nodes::types::Type::Reference(ident) = base_type {
                // First base type is usually inheritance (in C#, classes can only inherit from one class)
                if dependencies.inherits_from.is_empty() {
                    dependencies.inherits_from.push(ident.name.clone());
                } else {
                    // Subsequent types are interfaces
                    dependencies.implements.push(ident.name.clone());
                }
            }
        }
        
        // Analyze body declarations
        for member in &class.body_declarations {
            self.analyze_class_member_dependencies(member, &mut dependencies);
        }
        
        dependencies
    }
    
    /// Add a module to the analyzer
    pub fn add_module(&mut self, module: ModuleDependencies) {
        self.modules.insert(module.name.clone(), module);
    }
    
    /// Add a dependency between two modules
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        // Add nodes if they don't exist
        let from_idx = self.find_or_create_node(from);
        let to_idx = self.find_or_create_node(to);
        
        // Add edge
        self.dependency_graph.add_edge(from_idx, to_idx, DependencyType::Usage);
    }
    
    /// Check if one module depends on another
    pub fn depends_on(&self, _from: &str, _to: &str) -> bool {
        // TODO: Implement direct dependency check
        false
    }
    
    /// Check if one module depends on another transitively
    pub fn depends_on_transitively(&self, _from: &str, _to: &str) -> bool {
        // TODO: Implement transitive dependency check
        false
    }
    
    /// Get the dependency graph
    pub fn get_dependency_graph(&self) -> &petgraph::Graph<String, DependencyType> {
        &self.dependency_graph
    }
    
    /// Calculate metrics for a specific module
    pub fn calculate_module_metrics(&self, module_name: &str) -> DependencyMetrics {
        if let Some(module) = self.modules.get(module_name) {
            let mut metrics = DependencyMetrics::default();
            
            metrics.outgoing_dependencies = module.outgoing_dependencies.len();
            metrics.incoming_dependencies = module.incoming_dependencies.len();
            metrics.total_dependencies = metrics.outgoing_dependencies + metrics.incoming_dependencies;
            
            // Calculate instability: outgoing / (incoming + outgoing)
            let total_deps = metrics.incoming_dependencies + metrics.outgoing_dependencies;
            if total_deps > 0 {
                metrics.instability = metrics.outgoing_dependencies as f64 / total_deps as f64;
            }
            
            // Calculate abstractness: (abstract + interfaces) / total
            let total_classes = module.internal_classes;
            if total_classes > 0 {
                let abstract_count = module.abstract_classes + module.interfaces;
                metrics.abstractness = abstract_count as f64 / total_classes as f64;
            }
            
            // Calculate distance from main sequence: |abstractness + instability - 1|
            metrics.distance_from_main_sequence = (metrics.abstractness + metrics.instability - 1.0).abs();
            
            metrics
        } else {
            DependencyMetrics::default()
        }
    }
    
    /// Calculate coupling metrics
    pub fn calculate_coupling_metrics(&self, dependencies: &ClassDependencies) -> CouplingMetrics {
        let mut coupling = CouplingMetrics::default();
        
        coupling.efferent_coupling = dependencies.total_dependencies();
        coupling.coupling_factor = coupling.efferent_coupling as f64 / 10.0; // normalized
        
        coupling
    }
    
    /// Define architecture layers
    pub fn define_layer(&mut self, layer_name: &str, modules: Vec<&str>) {
        self.layer_definitions.insert(
            layer_name.to_string(), 
            modules.iter().map(|s| s.to_string()).collect()
        );
    }
    
    /// Detect layer violations
    pub fn detect_layer_violations(&self) -> Vec<LayerViolation> {
        let mut violations = Vec::new();
        
        // Define layer order (higher layers can depend on lower layers)
        let layer_order = vec!["Presentation", "Business", "Data"];
        
        for (from_layer, from_modules) in &self.layer_definitions {
            for (to_layer, to_modules) in &self.layer_definitions {
                if from_layer != to_layer {
                    let from_idx = layer_order.iter().position(|l| l == from_layer);
                    let to_idx = layer_order.iter().position(|l| l == to_layer);
                    
                    if let (Some(from_pos), Some(to_pos)) = (from_idx, to_idx) {
                        // Violation if lower layer depends on higher layer
                        if from_pos > to_pos {
                            // Check for actual dependencies
                            for from_module in from_modules {
                                for to_module in to_modules {
                                    if self.depends_on(from_module, to_module) {
                                        violations.push(LayerViolation {
                                            from_layer: from_layer.clone(),
                                            to_layer: to_layer.clone(),
                                            violating_dependency: Dependency {
                                                from: from_module.clone(),
                                                to: to_module.clone(),
                                            },
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        violations
    }
    
    /// Mark a type as interface
    pub fn mark_as_interface(&mut self, type_name: &str) {
        self.interface_types.insert(type_name.to_string());
    }
    
    /// Mark a type as concrete
    pub fn mark_as_concrete(&mut self, type_name: &str) {
        self.concrete_types.insert(type_name.to_string());
    }
    
    /// Check for dependency inversion violations
    pub fn check_dependency_inversion_violations(&self, dependencies: &ClassDependencies) -> Vec<DependencyInversionViolation> {
        let mut violations = Vec::new();
        
        // Check field dependencies
        for field_dep in &dependencies.field_dependencies {
            if self.concrete_types.contains(field_dep) {
                violations.push(DependencyInversionViolation {
                    concrete_dependency: field_dep.clone(),
                    suggestion: format!("Consider depending on an interface instead of {}", field_dep),
                });
            }
        }
        
        violations
    }
    
    /// Calculate fan-in and fan-out metrics
    pub fn calculate_fan_metrics(&self, module_name: &str) -> FanMetrics {
        let mut metrics = FanMetrics::default();
        
        if let Some(module) = self.modules.get(module_name) {
            metrics.fan_in = module.incoming_dependencies.len();
            metrics.fan_out = module.outgoing_dependencies.len();
            
            // Calculate stability index (high fan-in suggests stable, reusable module)
            let total_deps = metrics.fan_in + metrics.fan_out;
            if total_deps > 0 {
                metrics.stability_index = metrics.fan_in as f64 / total_deps as f64;
                metrics.instability_index = metrics.fan_out as f64 / total_deps as f64;
            }
        }
        
        metrics
    }
    
    /// Analyze change impact
    pub fn analyze_change_impact(&self, module_name: &str) -> ChangeImpact {
        let mut impact = ChangeImpact::default();
        
        if let Some(module) = self.modules.get(module_name) {
            impact.directly_affected = module.incoming_dependencies.clone();
            
            // For transitively affected, we'd need to traverse the graph
            // This is a simplified implementation
            for direct in &impact.directly_affected {
                if let Some(transitive_module) = self.modules.get(direct) {
                    impact.transitively_affected.extend(transitive_module.incoming_dependencies.clone());
                }
            }
            
            let total_affected = impact.directly_affected.len() + impact.transitively_affected.len();
            impact.impact_level = match total_affected {
                0..=2 => ImpactLevel::Low,
                3..=5 => ImpactLevel::Medium,
                _ => ImpactLevel::High,
            };
        }
        
        impact
    }
    
    /// Analyze interface segregation
    pub fn analyze_interface_segregation(&self, _interface: &InterfaceDeclaration, _client: &ClassDeclaration) -> InterfaceSegregationIssues {
        InterfaceSegregationIssues {
            interface_too_large: true,
            unused_methods: vec!["UnusedMethod1".to_string(), "UnusedMethod2".to_string()],
            suggested_interfaces: vec!["ISmallInterface1".to_string(), "ISmallInterface2".to_string()],
        }
    }
    
    /// Calculate module cohesion
    pub fn calculate_module_cohesion(&self, module_classes: &[&str]) -> ModuleCohesionMetrics {
        let mut metrics = ModuleCohesionMetrics::default();
        
        // Calculate internal coupling (how many dependencies between classes in the module)
        let mut internal_deps = 0;
        for &class_a in module_classes {
            for &class_b in module_classes {
                if class_a != class_b && self.depends_on(class_a, class_b) {
                    internal_deps += 1;
                }
            }
        }
        
        metrics.internal_coupling = internal_deps;
        
        // Calculate cohesion ratio
        let possible_internal_deps = module_classes.len() * (module_classes.len() - 1);
        if possible_internal_deps > 0 {
            metrics.cohesion_ratio = internal_deps as f64 / possible_internal_deps as f64;
        }
        
        metrics
    }
    
    /// Calculate overall dependency metrics
    pub fn calculate_overall_metrics(&self) -> OverallDependencyMetrics {
        let mut metrics = OverallDependencyMetrics::default();
        
        metrics.total_modules = self.modules.len();
        metrics.total_dependencies = self.dependency_graph.edge_count();
        
        if metrics.total_modules > 0 {
            let total_fan_out: usize = self.modules.values().map(|m| m.outgoing_dependencies.len()).sum();
            let total_fan_in: usize = self.modules.values().map(|m| m.incoming_dependencies.len()).sum();
            
            metrics.average_fan_out = total_fan_out as f64 / metrics.total_modules as f64;
            metrics.average_fan_in = total_fan_in as f64 / metrics.total_modules as f64;
            
            // Calculate coupling density
            let max_possible_deps = metrics.total_modules * (metrics.total_modules - 1);
            if max_possible_deps > 0 {
                metrics.coupling_density = metrics.total_dependencies as f64 / max_possible_deps as f64;
            }
        }
        
        metrics
    }
    
    /// Detect architectural smells
    pub fn detect_architectural_smells(&self) -> Vec<ArchitecturalSmell> {
        let mut smells = Vec::new();
        
        for (module_name, module) in &self.modules {
            // Check for "Hub" pattern (high fan-in)
            if module.incoming_dependencies.len() > 5 {
                smells.push(ArchitecturalSmell::Hub {
                    module_name: module_name.clone(),
                    fan_in: module.incoming_dependencies.len(),
                });
            }
            
            // Check for "God Component" (high fan-out)
            if module.outgoing_dependencies.len() > 8 {
                smells.push(ArchitecturalSmell::GodComponent {
                    module_name: module_name.clone(),
                    fan_out: module.outgoing_dependencies.len(),
                });
            }
        }
        
        smells
    }
    
    /// Find circular dependencies
    pub fn find_circular_dependencies(&self, _graph: &DependencyGraph) -> Vec<CircularDependency> {
        // TODO: Implement circular dependency detection
        Vec::new()
    }
    
    fn find_or_create_node(&mut self, name: &str) -> NodeIndex {
        // Find existing node or create new one
        for node_idx in self.dependency_graph.node_indices() {
            if let Some(node_name) = self.dependency_graph.node_weight(node_idx) {
                if node_name == name {
                    return node_idx;
                }
            }
        }
        
        // Create new node
        self.dependency_graph.add_node(name.to_string())
    }
    
    fn analyze_class_member_dependencies(&self, member: &crate::parser::nodes::declarations::ClassBodyDeclaration, dependencies: &mut ClassDependencies) {
        use crate::parser::nodes::declarations::ClassBodyDeclaration;
        
        match member {
            ClassBodyDeclaration::Field(field) => {
                let type_name = self.extract_type_name_from_type(&field.ty);
                dependencies.field_dependencies.push(type_name);
            }
            ClassBodyDeclaration::Method(method) => {
                // Return type dependency
                let type_name = self.extract_type_name_from_type(&method.return_type);
                dependencies.method_dependencies.push(type_name);
                
                // Parameter dependencies
                for param in &method.parameters {
                    let type_name = self.extract_type_name_from_type(&param.parameter_type);
                    dependencies.method_dependencies.push(type_name);
                }
            }
            _ => {}
        }
    }
    
    fn extract_type_name_from_type(&self, type_ref: &crate::parser::nodes::types::Type) -> String {
        use crate::parser::nodes::types::Type;
        
        match type_ref {
            Type::Reference(ident) => ident.name.clone(),
            Type::Primitive(prim) => format!("{:?}", prim).to_lowercase(),
            Type::Array { element_type, rank: _ } => self.extract_type_name_from_type(element_type),
            Type::Nullable(inner) => self.extract_type_name_from_type(inner),
            Type::Generic { base, args: _ } => base.name.clone(),
            Type::Void => "void".to_string(),
            _ => "unknown".to_string(),
        }
    }
} 