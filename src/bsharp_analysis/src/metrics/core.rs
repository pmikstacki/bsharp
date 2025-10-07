use serde::{Deserialize, Serialize};

/// Statistics about an AST structure
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AstAnalysis {
    // Type counts
    pub total_classes: usize,
    pub total_interfaces: usize,
    pub total_structs: usize,
    pub total_enums: usize,
    pub total_records: usize,
    pub total_delegates: usize,

    // Member counts
    pub total_methods: usize,
    pub total_properties: usize,
    pub total_fields: usize,
    pub total_events: usize,
    pub total_constructors: usize,

    // Statement counts
    pub total_if_statements: usize,
    pub total_for_loops: usize,
    pub total_while_loops: usize,
    pub total_switch_statements: usize,
    pub total_try_statements: usize,
    pub total_using_statements: usize,

    // Complexity metrics
    pub cyclomatic_complexity: usize,
    pub lines_of_code: usize,
    pub max_nesting_depth: usize,

    // Documentation metrics
    pub documented_methods: usize,
    pub documented_classes: usize,
}

impl AstAnalysis {
    /// Combine two analyses by adding their counts
    pub fn combine(self, other: AstAnalysis) -> AstAnalysis {
        AstAnalysis {
            total_classes: self.total_classes + other.total_classes,
            total_interfaces: self.total_interfaces + other.total_interfaces,
            total_structs: self.total_structs + other.total_structs,
            total_enums: self.total_enums + other.total_enums,
            total_records: self.total_records + other.total_records,
            total_delegates: self.total_delegates + other.total_delegates,

            total_methods: self.total_methods + other.total_methods,
            total_properties: self.total_properties + other.total_properties,
            total_fields: self.total_fields + other.total_fields,
            total_events: self.total_events + other.total_events,
            total_constructors: self.total_constructors + other.total_constructors,

            total_if_statements: self.total_if_statements + other.total_if_statements,
            total_for_loops: self.total_for_loops + other.total_for_loops,
            total_while_loops: self.total_while_loops + other.total_while_loops,
            total_switch_statements: self.total_switch_statements + other.total_switch_statements,
            total_try_statements: self.total_try_statements + other.total_try_statements,
            total_using_statements: self.total_using_statements + other.total_using_statements,

            cyclomatic_complexity: self.cyclomatic_complexity + other.cyclomatic_complexity,
            lines_of_code: self.lines_of_code + other.lines_of_code,
            max_nesting_depth: self.max_nesting_depth.max(other.max_nesting_depth),

            documented_methods: self.documented_methods + other.documented_methods,
            documented_classes: self.documented_classes + other.documented_classes,
        }
    }

    /// Calculate documentation coverage percentage
    pub fn documentation_coverage(&self) -> f64 {
        let total_documentable = self.total_methods + self.total_classes;
        let total_documented = self.documented_methods + self.documented_classes;

        if total_documentable == 0 {
            0.0
        } else {
            (total_documented as f64 / total_documentable as f64) * 100.0
        }
    }

    /// Calculate average cyclomatic complexity per method
    pub fn average_cyclomatic_complexity(&self) -> f64 {
        if self.total_methods == 0 {
            0.0
        } else {
            self.cyclomatic_complexity as f64 / self.total_methods as f64
        }
    }
}
