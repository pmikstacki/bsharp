use super::Type;
use serde::{Deserialize, Serialize};

/// Represents a list of type arguments, like `<T, U>`.
#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeArgumentList {
    pub types: Vec<Type>,
}
