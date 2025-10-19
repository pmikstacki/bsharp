use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode,Hash, Eq, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Modulo,   // %

    // Assignment
    Assign,                   // =
    AddAssign,                // +=
    SubtractAssign,           // -=
    MultiplyAssign,           // *=
    DivideAssign,             // /=
    ModuloAssign,             // %=
    AndAssign,                // &=
    OrAssign,                 // |=
    XorAssign,                // ^=
    LeftShiftAssign,          // <<=
    RightShiftAssign,         // >>=
    UnsignedRightShiftAssign, // >>>= (New)
    NullCoalescingAssign,     // ??= (New)

    // Comparison / Type Testing
    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Is,           // is (New)
    As,           // as (New)

    // Logical
    LogicalAnd, // &&
    LogicalOr,  // ||

    // Bitwise
    BitwiseAnd,         // &
    BitwiseOr,          // |
    BitwiseXor,         // ^
    LeftShift,          // <<
    RightShift,         // >>
    UnsignedRightShift, // >>> (New)

    // Null Coalescing
    NullCoalescing, // ?? (New)

    // Range
    Range, // .. (New)
}

impl Display for BinaryOperator{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Assign => write!(f, "="),
            BinaryOperator::AddAssign => write!(f, "+="),
            BinaryOperator::SubtractAssign => write!(f, "-="),
            BinaryOperator::MultiplyAssign => write!(f, "*="),
            BinaryOperator::DivideAssign => write!(f, "/="),
            BinaryOperator::ModuloAssign => write!(f, "%="),
            BinaryOperator::AndAssign => write!(f, "&="),
            BinaryOperator::OrAssign => write!(f, "|="),
            BinaryOperator::XorAssign => write!(f, "^="),
            BinaryOperator::LeftShiftAssign => write!(f, "<<="),
            BinaryOperator::RightShiftAssign => write!(f, ">>="),
            BinaryOperator::UnsignedRightShiftAssign => write!(f, ">>>="),
            BinaryOperator::NullCoalescingAssign => write!(f, "??="),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::Is => write!(f, "is"),
            BinaryOperator::As => write!(f, "as"),
            BinaryOperator::LogicalAnd => write!(f, "&&"),
            BinaryOperator::LogicalOr => write!(f, "||"),
            BinaryOperator::BitwiseAnd => write!(f, "&"),
            BinaryOperator::BitwiseOr => write!(f, "|"),
            BinaryOperator::BitwiseXor => write!(f, "^"),
            BinaryOperator::LeftShift => write!(f, "<<"),
            BinaryOperator::RightShift => write!(f, ">>"),
            BinaryOperator::UnsignedRightShift => write!(f, ">>>"),
            BinaryOperator::NullCoalescing => write!(f, "??"),
            BinaryOperator::Range => write!(f, ".."),
        }   
    }
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
            Self::LessThan
            | Self::GreaterThan
            | Self::LessEqual
            | Self::GreaterEqual
            | Self::Is
            | Self::As => 8,
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
            Self::Assign
            | Self::AddAssign
            | Self::SubtractAssign
            | Self::MultiplyAssign
            | Self::DivideAssign
            | Self::ModuloAssign
            | Self::AndAssign
            | Self::OrAssign
            | Self::XorAssign
            | Self::LeftShiftAssign
            | Self::RightShiftAssign
            | Self::UnsignedRightShiftAssign
            | Self::NullCoalescingAssign => 0,
        }
    }

    pub fn rhs_expectation(&self) -> &'static str {
        match self {
            Self::Add => "expression after '+'",
            Self::Subtract => "expression after '-'",
            Self::Multiply => "expression after '*'",
            Self::Divide => "expression after '/'",
            Self::Modulo => "expression after '%'",

            Self::Assign => "expression after '='",
            Self::AddAssign => "expression after '+='",
            Self::SubtractAssign => "expression after '-='",
            Self::MultiplyAssign => "expression after '*='",
            Self::DivideAssign => "expression after '/='",
            Self::ModuloAssign => "expression after '%='",
            Self::AndAssign => "expression after '&='",
            Self::OrAssign => "expression after '|='",
            Self::XorAssign => "expression after '^='",
            Self::LeftShiftAssign => "expression after '<<='",
            Self::RightShiftAssign => "expression after '>>='",
            Self::UnsignedRightShiftAssign => "expression after '>>>='",
            Self::NullCoalescingAssign => "expression after '??='",

            Self::Equal => "expression after '=='",
            Self::NotEqual => "expression after '!='",
            Self::LessThan => "expression after '<'",
            Self::GreaterThan => "expression after '>'",
            Self::LessEqual => "expression after '<='",
            Self::GreaterEqual => "expression after '>='",
            Self::Is => "expression after keyword 'is'",
            Self::As => "type after keyword 'as'",

            Self::LogicalAnd => "expression after '&&'",
            Self::LogicalOr => "expression after '||'",

            Self::BitwiseAnd => "expression after '&'",
            Self::BitwiseOr => "expression after '|'",
            Self::BitwiseXor => "expression after '^'",
            Self::LeftShift => "expression after '<<'",
            Self::RightShift => "expression after '>>'",
            Self::UnsignedRightShift => "expression after '>>>'",

            Self::NullCoalescing => "expression after '??'",
            Self::Range => "expression after '..'",
        }
    }
}
