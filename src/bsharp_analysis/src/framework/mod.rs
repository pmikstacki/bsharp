// New analysis framework module hub

pub mod config;
pub mod diagnostic_builder;
pub mod fqn;
pub mod lookup;
pub mod passes;
pub mod pipeline;
pub mod registry;
pub mod rules;
pub mod session;
pub mod visit;
pub mod walker;
mod stmt_walk;

pub use bsharp_syntax::node::ast_node::NodeRef;
pub use fqn::{class_fqn, method_fqn, namespace_fqn};
pub use lookup::{find_symbols_by_name, find_symbols_with_locations};
// Minimal re-exports for ergonomic use
pub use passes::{AnalyzerPass, Phase};
pub use pipeline::AnalyzerPipeline;
// Query API re-exports
pub use bsharp_syntax::query::Query;
pub use rules::{Rule, RuleSet, RuleTarget};
pub use session::{AnalysisSession, ArtifactStore};
pub use visit::Visit;
pub use walker::AstWalker;
