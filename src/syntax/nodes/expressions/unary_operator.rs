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
    Await,              // await (prefix)
    AddressOf,          // & (unsafe, prefix)
    PointerIndirection, // * (unsafe, prefix)
    Cast,               // (Type) (prefix, might need specific Expression variant later)
    IndexFromEnd,       // ^ (prefix, e.g. ^1)
    NullForgiving,      // ! (postfix, e.g. expr!)
                        // TODO: Add others like typeof, sizeof? Maybe separate nodes.
}
