use serde::{Serialize, Deserialize};
// Import PrimitiveType from the same directory's mod.rs 
use super::PrimitiveType;
use crate::parser::nodes::identifier::Identifier;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Type<'a> {
    Primitive(PrimitiveType),
    Reference(Identifier),
    Generic { base: Identifier, args: Vec<Type<'a>> },
    Array { element_type: Box<Type<'a>>, rank: usize },
    Pointer(Box<Type<'a>>),
    Nullable(Box<Type<'a>>),
    Dynamic, // dynamic keyword
    Void, // void keyword
    ImplicitArray, // For implicitly typed arrays like new[] { ... }
    Var, // For 'var' keyword used in implicitly typed local variables
    // This variant uses the lifetime to satisfy the compiler
    #[serde(skip)]
    Phantom(PhantomData<&'a ()>),
}

// Helper functions can be added here, e.g., for checking type compatibility
// impl Type {
//     pub fn is_numeric(&self) -> bool { ... }
// }
