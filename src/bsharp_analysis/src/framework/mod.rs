// New analysis framework module hub

pub mod config;
pub mod diagnostic_builder;
pub mod passes;
pub mod pipeline;
pub mod registry;
pub mod rules;
pub mod session;
pub mod walker;

// Minimal re-exports for ergonomic use
pub use passes::{AnalyzerPass, Phase};
pub use pipeline::AnalyzerPipeline;
pub use rules::{Rule, RuleSet, RuleTarget};
pub use session::{AnalysisSession, ArtifactStore};
pub use walker::{AstWalker, NodeRef, Visit};
