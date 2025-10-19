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
pub mod traits;
pub mod trivia;
pub mod types;
pub mod tokens;


use std::collections::HashMap;

pub type SpanTable = HashMap<String, std::ops::Range<usize>>;

// Compatibility: allow internal paths like `crate::parser::...` to resolve by re-exporting the crate root under `parser`.
pub mod parser {
    pub use super::*;
}

// Optional, minimal public API surface for traits
pub use traits::parsable::{Parsable, ParsableSpanned};
