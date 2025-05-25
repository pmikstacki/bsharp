use serde::{Serialize, Deserialize};
use super::Type;


/// Represents a list of type arguments, like `<T, U>`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeArgumentList {
    pub types: Vec<Type>,
}
