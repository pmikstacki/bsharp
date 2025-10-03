use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DefaultExpression {
    pub target_type: Option<Type>, // None for default literal
                                   // This marker helps Rust understand that we're intentionally using this lifetime
}
