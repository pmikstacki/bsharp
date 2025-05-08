use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum UnaryOperator {
    // Arithmetic
    Plus,          // +
    Minus,         // -
    // Logical
    LogicalNot,    // !
    // Bitwise
    BitwiseNot,    // ~
    // Increment/Decrement (prefix/postfix handled by context/structure)
    Increment,     // ++
    Decrement,     // --
    // TODO: Add others like await, typeof, sizeof? Maybe separate nodes.
}
