// Declare the subdirectories as modules
pub mod declarations;
pub mod expressions;
pub mod identifier;
pub mod statements;
pub mod types;
pub use identifier::Identifier;
// Ergonomic re-exports for dynamic traversal and queries
pub use node::ast_node::AstNode;

pub mod root;
pub mod trivia;
pub mod node;
pub mod emitters;
pub mod query;
mod formatter;
pub use formatter::{FormatOptions, Formatter};
// Added for XML documentation

// Optional: Re-export all public items from submodules for easier access
// pub use types::*;
// pub use declarations::*;
// pub use statements::*;
// pub use expressions::*;

// Compatibility alias so code can use `crate::ast::...`
pub use crate::root::ast;

// Ergonomic re-exports for dynamic traversal and queries
pub use crate::query::Query;
