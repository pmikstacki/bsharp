pub mod ast;
pub mod comment_parser;
pub mod errors;
pub mod nodes;
pub mod parser_helpers;
pub mod test_helpers;
pub mod keywords;
// Re-export the new idiomatic navigation traits from analysis as single source of truth
pub use crate::analysis::{AstNavigate, FindDeclarations};
// Re-export parser facade (logic lives in parser crate; this module stays API-only)
pub use crate::parser::facade::Parser;

//------------------------------------------------------------------------------
// Public Parser API
//------------------------------------------------------------------------------

// Parser facade moved to `parser/facade.rs`

//------------------------------------------------------------------------------
// Basic Parsers (Helpers retained for potential reuse)
//------------------------------------------------------------------------------

/*
================================================================================
 Outdated Parsers and Tests Removed/Commented Out During Refactoring (2025-05-03)

 The syntax logic previously defined below (primitive_type, expression,
 statement, class_member, using_directive, namespace_declaration, etc.)
 and the associated test module have been moved to the `src/parser/` directory
 or are being reimplemented there.

 This section (originally lines 62-375) is commented out to allow the rest
 of the project to compile while the refactoring is in progress.
================================================================================
*/
