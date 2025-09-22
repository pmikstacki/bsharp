// Analysis module - organized by feature area

pub mod control_flow; // Control flow analysis
pub mod dependencies; // Dependency analysis
pub mod diagnostics; // Diagnostic system with error codes
pub mod metrics; // Code metrics (counts, complexity)
pub mod naming; // Naming convention analysis
pub mod navigation; // AST navigation and search
pub mod quality; // Code quality analysis
pub mod semantic;
pub mod types; // Type usage analysis // Semantic analysis
pub mod context; // Shared analysis context and configuration

// Re-export main analysis traits and types for easy access
pub use control_flow::{ControlFlowAnalyzer, ControlFlowGraph, ControlFlowMetrics};
pub use dependencies::{
    CircularDependency, ClassDependencies, DependencyAnalyzer, DependencyGraph, DependencyMetrics,
    ModuleDependencies,
};
pub use diagnostics::{Diagnostic, DiagnosticCode, DiagnosticCollection, DiagnosticSeverity};
pub use metrics::{AstAnalysis, AstAnalyze, MetricCollector};
pub use naming::{NamingAnalyzer, NamingMetrics, NamingViolation};
pub use navigation::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
pub use quality::{QualityAnalyzer, QualityGrade, QualityIssue, QualityReport, QualitySeverity};
pub use semantic::{MemberAnalysis, SemanticAnalysisResult, SemanticAnalyzer, SemanticMemberType};
pub use types::{
    TypeAnalyzer, TypeCohesionMetrics, TypeComplexity, TypeComplexityMetrics, TypeMetrics,
    TypeUsage,
};
pub use context::{AnalysisConfig, AnalysisContext};
