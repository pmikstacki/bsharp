pub mod analyzer;
pub mod definitions;

pub use analyzer::TypeAnalyzer;
pub use definitions::{TypeUsage, TypeMetrics, TypeComplexity, TypeComplexityMetrics, TypeCohesionMetrics, TypeInfo, TypeKind};

// Remove all struct and enum definitions below this line, as they are now in definitions.rs and analyzer.rs 