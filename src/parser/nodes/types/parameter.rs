use serde::{Serialize, Deserialize};
// Import TypeSyntax from the same directory's mod.rs (which will re-export it)
use super::Type;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub parameter_type: Type,
    pub name: Identifier,
}
