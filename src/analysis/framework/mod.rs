// New analysis framework module hub

pub mod session;
pub mod rules;
pub mod walker;
pub mod passes;
pub mod registry;
pub mod pipeline;
pub mod config;
pub mod diagnostic_builder;

// Minimal re-exports for ergonomic use
pub use session::{AnalysisSession, ArtifactStore};
pub use rules::{Rule, RuleSet, RuleTarget};
pub use passes::{AnalyzerPass, Phase};
pub use walker::{AstWalker, NodeRef, Visit};
pub use pipeline::AnalyzerPipeline;
