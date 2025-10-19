use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    Void,
    Bool,

    // Integral types
    Byte,
    SByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    NInt,
    NUInt,

    // Floating-point types
    Float,
    Double,
    Decimal,

    // Character and string types
    Char,
    String,
    Object, // object keyword in C#
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrimitiveType::Void => "void",
            PrimitiveType::Bool => "bool",
            // Integral types
            PrimitiveType::Byte => "byte",
            PrimitiveType::SByte => "sbyte",
            PrimitiveType::Short => "short",
            PrimitiveType::UShort => "ushort",
            PrimitiveType::Int => "int",
            PrimitiveType::UInt => "uint",
            PrimitiveType::Long => "long",
            PrimitiveType::ULong => "ulong",
            PrimitiveType::NInt => "nint",
            PrimitiveType::NUInt => "nuint",
            // Floating-point types
            PrimitiveType::Float => "float",
            PrimitiveType::Double => "double",
            PrimitiveType::Decimal => "decimal",
            // Character and string types
            PrimitiveType::Char => "char",
            PrimitiveType::String => "string",
            PrimitiveType::Object => "object",
        };
        f.write_str(s)
    }
}