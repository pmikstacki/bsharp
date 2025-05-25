use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DefaultExpression {
    pub target_type: Option<Type>, // None for default literal
    // This marker helps Rust understand that we're intentionally using this lifetime

}
