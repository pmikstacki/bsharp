use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    
    // Floating-point types
    Float,
    Double,
    Decimal,
    
    // Character and string types
    Char,
    String,
}
