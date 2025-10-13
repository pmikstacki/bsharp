use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>, // Expression if condition is true
    pub alternative: Box<Expression>, // Expression if condition is false
}
