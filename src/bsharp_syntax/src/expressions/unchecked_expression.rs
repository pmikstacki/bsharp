use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedExpression {
    pub expr: Box<Expression>,
}
