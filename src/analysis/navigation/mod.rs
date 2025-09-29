// Navigation analysis module - organized by functionality

pub mod implementations; // Split by ast/, declarations/, statements/
pub mod traits; // Navigation trait definitions // Trait implementations for AST nodes
pub mod with_context; // Helpers that use AnalysisContext + SpanTable

// Re-export main types for easy access
pub use traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
pub use with_context::find_by_name_with_context;
