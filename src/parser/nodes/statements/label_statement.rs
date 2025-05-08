use serde::{Serialize, Deserialize};
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LabelStatement {
    pub label: Identifier,
}
