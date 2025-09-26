// Import PrimitiveType from the same directory's mod.rs
use super::{CallingConvention, PrimitiveType};
use crate::syntax::nodes::identifier::Identifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Type {
    Primitive(PrimitiveType),
    Reference(Identifier),
    Generic {
        base: Identifier,
        args: Vec<Type>,
    },
    Array {
        element_type: Box<Type>,
        rank: usize,
    },
    Pointer(Box<Type>),
    Nullable(Box<Type>),
    Dynamic,       // dynamic keyword
    Void,          // void keyword
    ImplicitArray, // For implicitly typed arrays like new[] { ... }
    Var,           // For 'var' keyword used in implicitly typed local variables
    FunctionPointer {
        calling_convention: Option<CallingConvention>, // Managed, Unmanaged, etc.
        parameter_types: Vec<Type>,
        return_type: Box<Type>,
    },
    NullableReference(Box<Type>), // C# 8+ nullable reference types (string?)
    RefReturn(Box<Type>),         // ref return types (ref int, ref MyClass)
    RefReadOnlyReturn(Box<Type>), // ref readonly return types (C# 7.2+)
}

// Helper functions can be added here, e.g., for checking type compatibility
// impl Type {
//     pub fn is_numeric(&self) -> bool { ... }
// }
