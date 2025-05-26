use crate::parser::nodes::types::Type;
use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SizeofExpression {
    pub target_type: Type,
    // This marker helps Rust understand that we're intentionally using this lifetime

}
