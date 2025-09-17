// Navigation analysis module - organized by functionality

pub mod implementations;
pub mod traits; // Navigation trait definitions // Trait implementations for AST nodes

// Re-export main types for easy access
pub use traits::{AstNavigate, DeclarationInfo, DeclarationType, FindDeclarations};
