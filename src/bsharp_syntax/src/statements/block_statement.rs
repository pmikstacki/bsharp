use super::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(bsharp_syntax_derive::AstNode, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}
