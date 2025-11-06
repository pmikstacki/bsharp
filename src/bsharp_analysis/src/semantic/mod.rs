// Semantic analysis passes and data

pub mod symbols;
pub mod binding;
pub mod types;
pub mod overload;
pub mod generics;
pub mod flow;
pub mod nullability;
pub mod attributes;
pub mod access;
pub mod extensions;

// Re-export key artifacts for tests
pub use symbols::{SymbolEntry, SymbolKind as SemSymbolKind, SymbolTable};
pub use binding::{BindingTable, BindingTarget};
