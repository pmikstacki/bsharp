pub mod analyzer;
pub mod definitions;

pub use analyzer::TypeAnalyzer;
pub use definitions::{
    TypeCohesionMetrics, TypeComplexity, TypeComplexityMetrics, TypeInfo, TypeKind, TypeMetrics,
    TypeUsage,
};

// Remove all struct and enum definitions below this line, as they are now in definitions.rs and analyzer.rs
