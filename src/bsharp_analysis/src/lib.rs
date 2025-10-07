// Analysis module - organized by feature area

// New framework-driven layout
pub mod artifacts; // symbols, metrics, cfg, dependencies
pub mod context; // AnalysisContext, AnalysisConfig
pub mod diagnostics; // Diagnostic types (kept)
pub mod framework; // session, rules, passes, walker, registry, pipeline
pub mod metrics;
pub mod passes; // indexing, control_flow, dependencies, reporting
pub mod project; // Project model
pub mod quality; // Keep legacy quality module for now
pub mod report; // AnalysisReport
pub mod rules; // naming, semantic, control_flow_smells
pub mod types; // Keep legacy types module for now // Keep metrics module (AstAnalysis, visitor, complexity)

// Expose workspace module for CLI consumption (WorkspaceLoader, etc.)
#[cfg(feature = "dev_utils")]
mod visitors;
pub mod workspace;

// Minimal re-exports only where ergonomic
pub use bsharp_parser::syntax;
pub use context::{AnalysisConfig, AnalysisContext};
pub use diagnostics::diagnostic_code::DiagnosticCode;
pub use diagnostics::diagnostic_collection::DiagnosticCollection;
pub use diagnostics::severity::DiagnosticSeverity;
pub use diagnostics::source_location::SourceLocation;
pub use framework::diagnostic_builder::DiagnosticBuilder;
pub use metrics::AstAnalysis;
pub use report::AnalysisReport;
