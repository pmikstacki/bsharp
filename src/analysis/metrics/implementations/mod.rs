// Module for organizing AstAnalyze trait implementations by AST node type
// This follows the modular architecture outlined in the refinement document

pub mod compilation_unit;
pub mod namespace;
pub mod class_declaration;
pub mod method_declaration;
pub mod statement;

// Re-export the namespace helper function for external use
pub use namespace::analyze_namespace_member; 