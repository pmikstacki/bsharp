// Declare the subdirectories as modules
pub mod declarations;
pub mod expressions;
pub mod identifier;
pub mod statements;
pub mod types;
pub use identifier::Identifier;
// Ergonomic re-exports for dynamic traversal and queries
pub use node::ast_node::AstNode;

pub mod emitters;
pub mod formatter;
pub mod node;
pub mod query;
pub mod root;
pub mod trivia;
pub use formatter::FormatOptions;
pub use formatter::Formatter;
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
