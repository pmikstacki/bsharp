use crate::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreprocessorDirective {
    Define { symbol: Identifier },
    Undef { symbol: Identifier },
    If { condition: String },
    Elif { condition: String },
    Else,
    Endif,
    Region { name: Option<String> },
    EndRegion,
    Error { message: String },
    Warning { message: String },
    Pragma { pragma: String },
    Line { line: String },
    // Fallback for directives we do not currently model specifically.
    // The text contains the rest of the line after '#'.
    Unknown { text: String },
}
