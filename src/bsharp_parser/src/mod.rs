pub mod bsharp;
pub mod declaration_helpers;
pub mod expressions;
pub mod facade;
pub mod helpers;
pub mod identifier_parser;
pub mod keywords;
pub mod parse_mode;
pub mod preprocessor;
pub mod statement_parser;
pub mod types;

use std::collections::HashMap;

/// Byte-span table keyed by a stable string key (e.g., "class::<ns>.<name>")
pub type SpanTable = HashMap<String, std::ops::Range<usize>>;
