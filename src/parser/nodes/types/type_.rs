use serde::{Serialize, Deserialize};
// Import PrimitiveType from the same directory's mod.rs 
use super::PrimitiveType;
use crate::parser::nodes::identifier::Identifier;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),
    Reference(Identifier),
    Generic { base: Identifier, args: Vec<Type> },
    Array { element_type: Box<Type>, rank: usize },
    Pointer(Box<Type>),
    Nullable(Box<Type>),
    Dynamic, // dynamic keyword
    Void, // void keyword
    ImplicitArray, // For implicitly typed arrays like new[] { ... }
    Var, // For 'var' keyword used in implicitly typed local variables
}

// Helper functions can be added here, e.g., for checking type compatibility
// impl Type {
//     pub fn is_numeric(&self) -> bool { ... }
// }
