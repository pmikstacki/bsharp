use crate::expressions::Expression;
use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LockStatement {
    pub expr: Expression,
    pub body: Box<Statement>,
}
