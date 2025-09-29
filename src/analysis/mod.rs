// Analysis module - organized by feature area

// New framework-driven layout
pub mod diagnostics; // Diagnostic types (kept)
pub mod context; // AnalysisContext, AnalysisConfig
pub mod framework; // session, rules, passes, walker, registry, pipeline
pub mod project; // Project model
pub mod artifacts; // symbols, metrics, cfg, dependencies
pub mod passes; // indexing, control_flow, dependencies, reporting
pub mod rules; // naming, semantic, control_flow_smells
pub mod report; // AnalysisReport
pub mod navigation; // Temporary: keep navigation utilities for CLI symbol queries
pub mod quality; // Keep legacy quality module for now
pub mod types; // Keep legacy types module for now
pub mod metrics; // Keep metrics module (AstAnalysis, visitor, complexity)

// Minimal re-exports only where ergonomic
pub use framework::diagnostic_builder::DiagnosticBuilder;
pub use diagnostics::diagnostic_code::DiagnosticCode;
pub use diagnostics::diagnostic_collection::DiagnosticCollection;
pub use diagnostics::severity::DiagnosticSeverity;
pub use diagnostics::source_location::SourceLocation;
pub use context::{AnalysisConfig, AnalysisContext};
pub use report::AnalysisReport;
pub use metrics::{AstAnalysis, AstAnalyze, MetricCollector};
pub use navigation::{AstNavigate, FindDeclarations};

