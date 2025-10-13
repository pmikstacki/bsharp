use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RangeExpression {
    pub start: Option<Box<Expression>>, // None for ..end
    pub end: Option<Box<Expression>>,   // None for start..
    pub is_inclusive: bool, // false for .., true for ..= (though C# doesn't have ..=, it's .. only)
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexExpression {
    pub value: Box<Expression>, // The expression after ^
}
