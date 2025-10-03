use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOperator {
    // Arithmetic
    Plus,  // + (prefix)
    Minus, // - (prefix)

    // Logical
    LogicalNot, // ! (prefix)

    // Bitwise
    BitwiseNot, // ~ (prefix)

    // Increment/Decrement (prefix/postfix contextually distinguished by Expression enum variants)
    Increment, // ++
    Decrement, // --

    // C# Specific Unary Operators
    AddressOf,          // & (unsafe, prefix)
    PointerIndirection, // * (unsafe, prefix)
    IndexFromEnd,       // ^ (prefix, e.g. ^1)
    NullForgiving,      // ! (postfix, e.g. expr!)
}
