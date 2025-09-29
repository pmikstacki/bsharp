pub mod bsharp;
pub mod declaration_helpers;
pub mod expressions;
pub mod facade;
pub mod identifier_parser;
pub mod statement_parser;
pub mod types;
pub mod keywords;

use std::collections::HashMap;

/// Byte-span table keyed by a stable string key (e.g., "class::<ns>.<name>")
pub type SpanTable = HashMap<String, std::ops::Range<usize>>;
