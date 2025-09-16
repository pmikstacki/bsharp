// Navigation analysis module - organized by functionality

pub mod traits;         // Navigation trait definitions
pub mod implementations; // Trait implementations for AST nodes

// Re-export main types for easy access
pub use traits::{AstNavigate, FindDeclarations, DeclarationInfo, DeclarationType};
 