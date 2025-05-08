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
    // Comparison
    Equal,          // ==
    NotEqual,       // !=
    LessThan,       // <
    GreaterThan,    // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    // Logical
    LogicalAnd,     // &&
    LogicalOr,      // ||
    // Bitwise
    BitwiseAnd,     // &
    BitwiseOr,      // |
    BitwiseXor,     // ^
    LeftShift,      // <<
    RightShift,     // >>
}

impl BinaryOperator {
    // Get the precedence level of the binary operator.
    // Higher values indicate higher precedence.
    // Based on C# operator precedence: https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/operators/#operator-precedence
    pub fn precedence(&self) -> i32 {
        match self {
            // Multiplicative
            Self::Multiply | Self::Divide | Self::Modulo => 10,
            // Additive
            Self::Add | Self::Subtract => 9,
            // Shift
            Self::LeftShift | Self::RightShift => 8,
            // Relational and type testing (is, as - not here)
            Self::LessThan | Self::GreaterThan | Self::LessEqual | Self::GreaterEqual => 7,
            // Equality
            Self::Equal | Self::NotEqual => 6,
            // Bitwise AND
            Self::BitwiseAnd => 5,
            // Bitwise XOR
            Self::BitwiseXor => 4,
            // Bitwise OR
            Self::BitwiseOr => 3,
            // Logical AND
            Self::LogicalAnd => 2,
            // Logical OR
            Self::LogicalOr => 1,
            // Null-coalescing (??) - not here yet
            // Conditional (?:) - handled differently
            // Assignment operators - handled by parse_assignment_expression
            Self::Assign | Self::AddAssign | Self::SubtractAssign | Self::MultiplyAssign | 
            Self::DivideAssign | Self::ModuloAssign | Self::AndAssign | Self::OrAssign | 
            Self::XorAssign | Self::LeftShiftAssign | Self::RightShiftAssign => 0, // Lowest precedence, shouldn't be handled by binary precedence climbing
        }
    }
}
