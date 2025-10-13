use crate::types::Type;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SizeofExpression {
    pub target_type: Type,
    // This marker helps Rust understand that we're intentionally using this lifetime
}
