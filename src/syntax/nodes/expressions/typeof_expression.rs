use crate::syntax::nodes::types::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeofExpression {
    pub target_type: Type,
    // This marker helps Rust understand that we're intentionally using this lifetime
}
