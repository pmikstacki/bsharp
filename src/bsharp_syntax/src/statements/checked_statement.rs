use crate::statements::statement::Statement;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckedStatement {
    pub body: Box<Statement>, // Usually Statement::Block
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UncheckedStatement {
    pub body: Box<Statement>,
}
