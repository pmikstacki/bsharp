use serde::{Serialize, Deserialize};
use super::Type;
use std::marker::PhantomData;

/// Represents a list of type arguments, like `<T, U>`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeArgumentList<'a> {
    pub types: Vec<Type<'a>>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    _phantom: PhantomData<&'a ()>,
}
