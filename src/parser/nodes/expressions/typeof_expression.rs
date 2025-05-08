use serde::{Serialize, Deserialize};
use crate::parser::nodes::types::Type;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeofExpression<'a> {
    pub target_type: Type<'a>,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    _phantom: PhantomData<&'a ()>,
}
