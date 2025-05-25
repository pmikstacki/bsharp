use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeofExpression {
    pub target_type: Type,
    // This marker helps Rust understand that we're intentionally using this lifetime

}
