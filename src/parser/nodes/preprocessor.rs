use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;

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
}
