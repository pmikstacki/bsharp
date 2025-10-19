use std::fmt::Display;
use serde::{Deserialize, Serialize};
use bsharp_syntax_derive::AstNode;

#[derive(AstNode, Debug, PartialEq, Hash, Eq, Clone, Copy, Serialize, Deserialize)]
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

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::LogicalNot => write!(f, "!"),
            UnaryOperator::BitwiseNot => write!(f, "~"),
            UnaryOperator::Increment => write!(f, "++"),
            UnaryOperator::Decrement => write!(f, "--"),
            UnaryOperator::AddressOf => write!(f, "&"),
            UnaryOperator::PointerIndirection => write!(f, "*"),
            UnaryOperator::IndexFromEnd => write!(f, "^"),
            UnaryOperator::NullForgiving => write!(f, "!"),
        }    
    }
}
