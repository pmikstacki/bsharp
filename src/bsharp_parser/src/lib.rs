//! bsharp_parser crate
//! 
//! Spanned-first public entrypoints:
//! - Expressions: `parser::expressions::primary_expression_parser::parse_expression_spanned`
//!   and `parser::expressions::primary_expression_parser::parse_primary_expression_spanned`.
//! - Full source: `bsharp::parse_csharp_source_spanned`.
//! 
//! Idiom: obtain unspanned nodes via `.map(|s| s.node)` on returned `Spanned<T>`.
//! Legacy unspanned helpers are deprecated and not re-exported at the crate surface.
//! Prefer the spanned-first APIs for consistent error reporting and diagnostics.
pub mod bsharp;
pub mod declaration_helpers;
pub mod expressions;
pub mod facade;
pub mod helpers;
pub mod identifier_parser;
pub mod keywords;
pub mod parse_mode;
pub mod statement_parser;
pub mod syntax;
pub mod tokens;
pub mod traits;
pub mod trivia;
pub mod types;

// Test-only diagnostics exposure (behind feature)
pub mod test_diagnostics;
pub mod errors;

// Sidecar span types and parser extension
pub mod span;
pub mod span_ext;

use std::collections::HashMap;

pub type SpanTable = HashMap<String, std::ops::Range<usize>>;

// Compatibility: allow internal paths like `crate::parser::...` to resolve by re-exporting the crate root under `parser`.
pub mod parser {
    pub use super::*;
}

// Optional, minimal public API surface for traits
pub use traits::parsable::{Parsable, ParsableSpanned};

// Re-export span API
pub use span::{Spanned, ByteRange, LineOffset, TextRange, HasSpan};
pub use span_ext::ParserExt as SpannedParserExt;

// Public spanned root entrypoint
pub use bsharp::parse_csharp_source_spanned;
