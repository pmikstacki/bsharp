use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedStatement {
    pub body: Box<Statement>, // Usually Statement::Block
}

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedStatement {
    pub body: Box<Statement>,
}
