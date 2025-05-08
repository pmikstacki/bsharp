use serde::{Serialize, Deserialize};
// Import TypeSyntax from the same directory's mod.rs (which will re-export it)
use super::Type;
use crate::parser::nodes::identifier::Identifier;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Parameter<'a> {
    pub ty: Type<'a>,
    pub name: Identifier,
    // This marker helps Rust understand that we're intentionally using this lifetime
    #[serde(skip)]
    pub _phantom: PhantomData<&'a ()>,
}
