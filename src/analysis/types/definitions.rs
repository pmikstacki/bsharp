use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeUsage {
    pub primitive_types: HashMap<String, usize>,
    pub custom_types: HashMap<String, usize>,
    pub generic_types: HashMap<String, usize>,
    pub nullable_types: usize,
    pub array_types: usize,
}

impl TypeUsage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total_types(&self) -> usize {
        self.primitive_types.len() + self.custom_types.len() + self.generic_types.len()
    }
}

/// Type metrics for detailed analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeMetrics {
    pub total_types_used: usize,
    pub inheritance_depth: usize,
    pub interface_implementations: usize,
    pub generic_type_parameters: usize,
    pub field_types: Vec<String>,
    pub property_types: Vec<String>,
    pub method_parameter_types: Vec<String>,
    pub method_return_types: Vec<String>,
    pub implemented_interfaces: Vec<String>,
    pub generic_constraints: Vec<String>,
    pub array_types: Vec<String>,
    pub nullable_types: Vec<String>,
    pub generic_type_usages: Vec<String>,
    pub ref_parameters: usize,
    pub out_parameters: usize,
    pub async_return_types: Vec<String>,
}

impl TypeMetrics {
    pub fn combine(mut self, other: TypeMetrics) -> TypeMetrics {
        self.total_types_used += other.total_types_used;
        self.inheritance_depth = self.inheritance_depth.max(other.inheritance_depth);
        self.interface_implementations += other.interface_implementations;
        self.generic_type_parameters += other.generic_type_parameters;
        self.field_types.extend(other.field_types);
        self.property_types.extend(other.property_types);
        self.method_parameter_types
            .extend(other.method_parameter_types);
        self.method_return_types.extend(other.method_return_types);
        self.implemented_interfaces
            .extend(other.implemented_interfaces);
        self.generic_constraints.extend(other.generic_constraints);
        self.array_types.extend(other.array_types);
        self.nullable_types.extend(other.nullable_types);
        self.generic_type_usages.extend(other.generic_type_usages);
        self.ref_parameters += other.ref_parameters;
        self.out_parameters += other.out_parameters;
        self.async_return_types.extend(other.async_return_types);
        self
    }
}

/// Type complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeComplexity {
    Simple,      // int, string, bool
    Moderate,    // List<T>, Dictionary<K,V>
    Complex,     // Func<T1, T2, T3, TResult>
    VeryComplex, // Deeply nested generics
}

/// Type complexity metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct TypeComplexityMetrics {
    pub nesting_depth: usize,
    pub generic_type_count: usize,
    pub constraint_count: usize,
    pub overall_complexity: f64,
}

/// Type cohesion metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct TypeCohesionMetrics {
    pub semantic_cohesion: f64,
    pub type_relatedness: f64,
    pub overall_cohesion: f64,
}

/// Information about a discovered type
#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize, Deserialize
pub struct TypeInfo {
    /// Simple name of the type (no namespace / nesting)
    pub name: String,
    /// Fully-qualified name built from namespace and nesting (dot-joined)
    pub fqn: String,
    /// Namespace of the type, if any
    pub namespace: Option<String>,
    /// Kind of the type (class/struct/interface/record/enum/delegate)
    pub kind: TypeKind,
    /// Base types or implemented interfaces by FQN (best-effort; may be short names if unresolved)
    pub base_types: Vec<String>,
    /// Member counts summary for the type body
    pub member_counts: MemberCounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)] // Added Serialize, Deserialize
pub enum TypeKind {
    Class,
    Interface,
    Struct,
    Enum,
    Delegate,
    Record,
}

/// Summary counts of members grouped by kind
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemberCounts {
    pub fields: usize,
    pub properties: usize,
    pub methods: usize,
    pub events: usize,
    pub indexers: usize,
    pub nested_types: usize,
}

/// Detailed, structural complexity for a single type reference
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeComplexityDetail {
    pub array_depth: usize,
    pub generic_depth: usize,
    pub total_generic_args: usize,
    pub is_nullable: bool,
    pub is_pointer: bool,
}
