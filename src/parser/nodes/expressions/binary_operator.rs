use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,            // +
    Subtract,       // -
    Multiply,       // *
    Divide,         // /
    Modulo,         // %

    // Assignment
    Assign,         // =
    AddAssign,      // +=
    SubtractAssign, // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModuloAssign,   // %=
    AndAssign,      // &=
    OrAssign,       // |=
    XorAssign,      // ^=
    LeftShiftAssign,// <<=
    RightShiftAssign,// >>=
    UnsignedRightShiftAssign, // >>>= (New)
    NullCoalescingAssign, // ??= (New)

    // Comparison / Type Testing
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    Is,             // is (New)
    As,             // as (New)

    // Logical
    LogicalAnd,     // &&
    LogicalOr,      // ||
    
    // Bitwise
    BitwiseAnd,     // &
    BitwiseOr,      // |
    BitwiseXor,     // ^
    LeftShift,      // <<
    RightShift,     // >>
    UnsignedRightShift, // >>> (New)

    // Null Coalescing
    NullCoalescing, // ?? (New)

    // Range
    Range,          // .. (New)
}

impl BinaryOperator {
    // Get the precedence level of the binary operator.
    // Higher values indicate higher precedence.
    // Based on C# operator precedence: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/operators/#operator-precedence
    pub fn precedence(&self) -> i32 {
        match self {
            // Primary operators (like range `..`) have high precedence,
            // but are often handled by specific grammar rules rather than this generic function.
            // For now, giving Range a high value, but its parsing might be specialized.
            Self::Range => 12, 

            // Multiplicative
            Self::Multiply | Self::Divide | Self::Modulo => 11,
            // Additive
            Self::Add | Self::Subtract => 10,
            // Shift
            Self::LeftShift | Self::RightShift | Self::UnsignedRightShift => 9,
            // Relational and type testing
            Self::LessThan | Self::GreaterThan | Self::LessEqual | Self::GreaterEqual | Self::Is | Self::As => 8,
            // Equality
            Self::Equal | Self::NotEqual => 7,
            // Bitwise AND
            Self::BitwiseAnd => 6,
            // Bitwise XOR
            Self::BitwiseXor => 5,
            // Bitwise OR
            Self::BitwiseOr => 4,
            // Logical AND
            Self::LogicalAnd => 3,
            // Logical OR
            Self::LogicalOr => 2,
            // Null Coalescing
            Self::NullCoalescing => 1, // ?? has lower precedence than || and &&
            // Assignment operators (lowest regular precedence)
            // Conditional operator ?: is even lower but handled differently.
            Self::Assign | Self::AddAssign | Self::SubtractAssign | 
            Self::MultiplyAssign | Self::DivideAssign | Self::ModuloAssign |
            Self::AndAssign | Self::OrAssign | Self::XorAssign |
            Self::LeftShiftAssign | Self::RightShiftAssign | Self::UnsignedRightShiftAssign |
            Self::NullCoalescingAssign => 0,
        }
    }
}
