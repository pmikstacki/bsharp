use crate::expressions::Expression;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexingExpression {
    pub target: Box<Expression>,
    pub index: Box<Expression>,
}
