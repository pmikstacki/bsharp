use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThrowExpression {
    pub expr: Option<Box<Expression>>, // None for 'throw;' in expressions
}
