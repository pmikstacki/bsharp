// Import PrimitiveType from the same directory's mod.rs
use super::{CallingConvention, PrimitiveType};
use crate::Identifier;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Primitive(p) => write!(f, "{}", p),
            Type::Reference(id) => write!(f, "{}", id),
            Type::Generic { base, args } => {
                write!(f, "{}<", base)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 { f.write_str(", ")?; }
                    write!(f, "{}", a)?;
                }
                f.write_str(">")
            }
            Type::Array { element_type, rank } => {
                write!(f, "{}", element_type)?;
                if *rank == 1 {
                    f.write_str("[]")
                } else {
                    f.write_str("[")?;
                    for i in 0..(rank.saturating_sub(1)) {
                        if i > 0 { f.write_str("")?; }
                        f.write_str(",")?;
                    }
                    f.write_str("]")
                }
            }
            Type::Pointer(inner) => write!(f, "{}*", inner),
            Type::Nullable(inner) | Type::NullableReference(inner) => write!(f, "{}?", inner),
            Type::Dynamic => f.write_str("dynamic"),
            Type::Void => f.write_str("void"),
            Type::ImplicitArray => f.write_str("new[]"),
            Type::Var => f.write_str("var"),
            Type::FunctionPointer { calling_convention, parameter_types, return_type } => {
                f.write_str("delegate*")?;
                if let Some(cc) = calling_convention {
                    match cc {
                        CallingConvention::Managed => f.write_str(" managed")?,
                        CallingConvention::Unmanaged => f.write_str(" unmanaged")?,
                    }
                }
                f.write_str("<")?;
                for (i, p) in parameter_types.iter().enumerate() {
                    if i > 0 { f.write_str(", ")?; }
                    write!(f, "{}", p)?;
                }
                if !parameter_types.is_empty() { f.write_str(", ")?; }
                write!(f, "{}", return_type)?;
                f.write_str(">")
            }
            Type::RefReturn(inner) => write!(f, "ref {}", inner),
            Type::RefReadOnlyReturn(inner) => write!(f, "ref readonly {}", inner),
        }
    }
}
