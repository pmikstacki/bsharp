// Analysis module - organized by feature area

pub mod metrics;      // Code metrics (counts, complexity)
pub mod navigation;   // AST navigation and search
pub mod control_flow; // Control flow analysis
pub mod types;        // Type usage analysis
pub mod dependencies; // Dependency analysis
pub mod naming;       // Naming convention analysis
pub mod quality;      // Code quality analysis

// Re-export main analysis traits and types for easy access
pub use metrics::{AstAnalyze, AstAnalysis, MetricCollector};
pub use navigation::{AstNavigate, FindDeclarations, DeclarationInfo, DeclarationType};
pub use control_flow::{ControlFlowAnalyzer, ControlFlowGraph, ControlFlowMetrics};
pub use types::{TypeAnalyzer, TypeUsage, TypeComplexity, TypeMetrics, TypeComplexityMetrics, TypeCohesionMetrics};
pub use dependencies::{DependencyAnalyzer, DependencyGraph, CircularDependency, DependencyMetrics, ModuleDependencies, ClassDependencies};
pub use naming::{NamingAnalyzer, NamingViolation, NamingMetrics};
pub use quality::{QualityAnalyzer, QualityIssue, QualityReport, QualitySeverity, QualityGrade}; 