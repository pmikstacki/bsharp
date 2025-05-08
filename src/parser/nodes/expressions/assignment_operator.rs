use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum AssignmentOperator {
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
    // TODO: NullCoalescingAssign, // ??=
}
